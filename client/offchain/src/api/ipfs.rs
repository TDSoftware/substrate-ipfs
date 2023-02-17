//! This module is composed of two structs: [`IpfsApi`] and [`IpfsWorker`]. Calling the [`ipfs`]
//! function returns a pair of [`IpfsApi`] and [`IpfsWorker`] that share some state.
//!
//! The [`IpfsApi`] is (indirectly) passed to the runtime when calling an offchain worker, while
//! the [`IpfsWorker`] must be processed in the background. The [`IpfsApi`] mimics the API of the
//! IPFS-related methods available to offchain workers.
//!
//! The reason for this design is driven by the fact that IPFS requests should continue running
//! in the background even if the runtime isn't actively calling any function.
//!
//! All credit goes to:
//! https://github.com/rs-ipfs/substrate/blob/2f565db044133cacfdfc166ca9b96594644e34e9/client/offchain/src/api/ipfs.rs

use crate::api::timestamp;
use cid::{Cid};
use fnv::FnvHashMap;
use futures::{future, prelude::*, pin_mut};
use rust_ipfs::unixfs::UnixfsStatus;
use rust_ipfs::{
	BitswapStats, Block, Connection, Ipfs, IpfsPath, IpfsTypes,
	Multiaddr, MultiaddrWithPeerId, PeerId, PublicKey, SubscriptionStream,
};

use rust_ipfs::path::PathRoot::{
	Ipld
};

use log::{error, info};
use sc_utils::mpsc::{tracing_unbounded, TracingUnboundedReceiver, TracingUnboundedSender};
use sp_core::offchain::{
	IpfsRequest, IpfsRequestId, IpfsRequestStatus, IpfsResponse, OpaqueMultiaddr, Timestamp,
};
use tokio::sync::broadcast::error;
use std::fmt::Debug;
use std::{
	collections::BTreeMap,
	convert::TryInto,
	fmt, mem,
	pin::Pin,
	str,
	task::{Context, Poll},
};

const LOG_TARGET: &str = "offchain-worker::http";

// wasm-friendly implementations of Ipfs::{add, get}
async fn ipfs_add<T: IpfsTypes>(ipfs: &Ipfs<T>, data: Vec<u8>, version: u8) -> Result<Cid, rust_ipfs::Error> {
	let data_packed = tokio_stream::once(data).boxed();
	let mut data_stream = ipfs.add_unixfs(data_packed).await?;

	let mut result: Result<Cid, rust_ipfs::Error> = Result::Err(rust_ipfs::Error::msg("Unknown error"));
	while let Some(status) = data_stream.next().await {
        match status {
            UnixfsStatus::ProgressStatus {
                written,
                total_size,
            } => match total_size {
                Some(size) => tracing::debug!("{written} out of {size} stored"),
                None => tracing::debug!("{written} been stored"),
            },
            UnixfsStatus::FailedStatus {
                written: _,
                total_size: _,
                error: _,
            } => {
                result = Result::Err(rust_ipfs::Error::msg("Error adding file"))
            }
            UnixfsStatus::CompletedStatus { path, written, .. } => {
                tracing::debug!("{written} been stored with path {path}");
				let cid_str = path.to_string().replace("/ipfs/", "");

				match Cid::try_from(cid_str) {
        			Ok(cid) => result = Ok(cid),
        			Err(_) => result = Err(rust_ipfs::Error::msg("Unable to upload file")),
   				}
            }
        }
    }

	// TODO: TDS IPFS
	// let dag = ipfs.dag();

	// let links: Vec<rust_ipfs::path::PathRoot::Ipld> = vec![];
	// let mut pb_node = BTreeMap::<String, rust_ipfs::path::PathRoot>::new();
	// pb_node.insert("Data".to_string(), data.into());
	// pb_node.insert("Links".to_string(), links.into());

	// // TODO: https://docs.rs/cid/0.7.0/cid, https://docs.rs/cid/0.5.1/src/cid/codec.rs.html#9-11 https://docs.rs/ipfs/0.2.1/ipfs/type.Cid.html
	// dag.put(pb_node.into(), Codec::DagProtobuf, None).await
	tracing::info!("IPFS add file result: {:?}", result);
	result

}

async fn ipfs_get<T: IpfsTypes>(ipfs: &Ipfs<T>, path: IpfsPath) -> Result<Vec<u8>, rust_ipfs::Error> {
	let path_copy = path.clone();
	let stream_result = ipfs.cat_unixfs(path, None).await;
	let default_error_msg = "Unable to cat file: ".to_string() + &path_copy.to_string();

	let default_error = rust_ipfs::Error::msg(default_error_msg);

	if let Ok(stream) = stream_result {
		pin_mut!(stream);
		let mut data = Vec::<u8>::new();

		loop {
			match stream.next().await {
				Some(Ok(bytes)) => {
					data.append(&mut bytes.clone());
				}
				Some(Err( _ )) => {
					return Err(default_error);
				}
				None => {
					break
				}
			}
		}
		Ok(data)
	}
	else {
		Err(rust_ipfs::Error::msg("Unable to upload file"))
	}
}

/// Creates a pair of [`IpfsApi`] and [`IpfsWorker`].
pub fn ipfs<I: ::rust_ipfs::IpfsTypes>(ipfs_node: ::rust_ipfs::Ipfs<I>) -> (IpfsApi, IpfsWorker<I>) {
	let (to_worker, from_api) = tracing_unbounded("mpsc_ocw_to_ipfs_worker", 10000_00);
	let (to_api, from_worker) = tracing_unbounded("mpsc_ocw_to_ipfs_api", 10000_00);

	let api = IpfsApi {
		to_worker,
		from_worker: from_worker.fuse(),
		// We start with a random ID for the first IPFS request, to prevent mischievous people from
		// writing runtime code with hardcoded IDs.
		next_id: IpfsRequestId(rand::random::<u16>() % 2000),
		requests: FnvHashMap::default(),
	};

	let worker = IpfsWorker { to_api, from_api, ipfs_node, requests: Vec::new() };

	(api, worker)
}

/// Provides IPFS capabilities.
///
/// Since this struct is a helper for offchain workers, its API is mimicking the API provided
/// to offchain workers.
pub struct IpfsApi {
	/// Used to sends messages to the worker.
	to_worker: TracingUnboundedSender<ApiToWorker>,
	/// Used to receive messages from the worker.
	/// We use a `Fuse` in order to have an extra protection against panicking.
	from_worker: stream::Fuse<TracingUnboundedReceiver<WorkerToApi>>,
	/// Id to assign to the next IPFS request that is started.
	next_id: IpfsRequestId,
	/// List of IPFS requests in preparation or in progress.
	requests: FnvHashMap<IpfsRequestId, IpfsApiRequest>,
}

/// One active request within `IpfsApi`.
enum IpfsApiRequest {
	Dispatched,
	Response(IpfsNativeResponse),
	Fail(rust_ipfs::Error),
}

impl IpfsApi {
	/// Mimics the corresponding method in the offchain API.
	pub fn request_start(&mut self, request: IpfsRequest) -> Result<IpfsRequestId, ()> {
		tracing::info!("Starting request {:?}", request);
		let id = self.next_id;
		debug_assert!(!self.requests.contains_key(&id));

		match self.next_id.0.checked_add(1) {
			Some(id) => self.next_id.0 = id,
			None => {
				error!("Overflow in offchain worker IPFS request ID assignment");
				return Err(())
			},
		};

		let _ = self.to_worker.unbounded_send(ApiToWorker { id, request });

		self.requests.insert(id, IpfsApiRequest::Dispatched);

		Ok(id)
	}

	/// Mimics the corresponding method in the offchain API.
	pub fn response_wait(
		&mut self,
		ids: &[IpfsRequestId],
		deadline: Option<Timestamp>,
	) -> Vec<IpfsRequestStatus> {
		let mut deadline = timestamp::deadline_to_future(deadline);
		let mut output = vec![IpfsRequestStatus::DeadlineReached; ids.len()];

		loop {
			{
				let mut must_wait_more = false;
				let mut out_idx = 0;
				for id in ids {
					match self.requests.get_mut(id) {
						None => output[out_idx] = IpfsRequestStatus::Invalid,
						Some(IpfsApiRequest::Dispatched) => must_wait_more = true,
						Some(IpfsApiRequest::Fail(e)) =>
							output[out_idx] = IpfsRequestStatus::IoError(e.to_string().into_bytes()),
						Some(IpfsApiRequest::Response(IpfsNativeResponse::Success)) => {},
						Some(IpfsApiRequest::Response(ref mut resp)) => {
							output[out_idx] = IpfsRequestStatus::Finished(IpfsResponse::from(
								mem::replace(resp, IpfsNativeResponse::Success),
							));
						},
					};
					out_idx += 1;
				}
				debug_assert_eq!(output.len(), ids.len());

				// Are we ready to call `return`?
				let is_done =
					if let future::MaybeDone::Done(_) = deadline { true } else { !must_wait_more };

				if is_done {
					// Requests in "fail" mode are purged before returning.
					debug_assert_eq!(output.len(), ids.len());
					for n in (0..ids.len()).rev() {
						if let IpfsRequestStatus::IoError(_) = output[n] {
							self.requests.remove(&ids[n]);
						}
					}
					return output
				}
			}

			// Grab next message from the worker. We call `continue` if deadline is reached so that
			// we loop back and `return`.
			let next_message = {
				let mut next_msg = future::maybe_done(self.from_worker.next());

				tracing::info!("next_msg 1{:?}", next_msg);
				futures::executor::block_on(future::select(&mut next_msg, &mut deadline));
				tracing::info!("next_msg 2 {:?}", next_msg);

				if let future::MaybeDone::Done(msg) = next_msg {
					msg
				} else {
					tracing::info!("No future::MaybeDone::Done");
					debug_assert!(matches!(deadline, future::MaybeDone::Done(..)));
					continue
				}
			};

			// Update internal state based on received message.
			tracing::info!("IPFS next message {:?}", next_message);
			match next_message {
				Some(WorkerToApi::Response { id, value }) => match self.requests.remove(&id) {
					Some(IpfsApiRequest::Dispatched) => {
						self.requests.insert(id, IpfsApiRequest::Response(value));
					},
					_ => error!("State mismatch between the API and worker"),
				},

				Some(WorkerToApi::Fail { id, error }) => match self.requests.remove(&id) {
					Some(IpfsApiRequest::Dispatched) => {
						tracing::info!("IPFS response fail");
						self.requests.insert(id, IpfsApiRequest::Fail(error));
					},
					_ => error!("State mismatch between the API and worker"),
				},

				None => {
					error!("IPFS Worker XXX returned None 2: response_wait");
					return ids
						.iter()
						.map(|_| {
							IpfsRequestStatus::IoError(b"The IPFS worker has crashed!".to_vec())
						})
						.collect()
				},
			}
		}
	}
}

impl fmt::Debug for IpfsApi {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_list().entries(self.requests.iter()).finish()
	}
}

impl fmt::Debug for IpfsApiRequest {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			IpfsApiRequest::Dispatched => f.debug_tuple("IpfsApiRequest::Dispatched").finish(),
			IpfsApiRequest::Response(_) => f.debug_tuple("IpfsApiRequest::Response").finish(),
			IpfsApiRequest::Fail(err) => f.debug_tuple("IpfsApiRequest::Fail").field(err).finish(),
		}
	}
}

/// Message send from the API to the worker.
struct ApiToWorker {
	/// ID to send back when the response comes back.
	id: IpfsRequestId,
	/// Request to start executing.
	request: IpfsRequest,
}

#[derive(Debug)]
/// Message send from the API to the worker.
enum WorkerToApi {
	/// A request has succeeded.
	Response {
		/// The ID that was passed to the worker.
		id: IpfsRequestId,
		/// Status code of the response.
		value: IpfsNativeResponse,
	},
	/// A request has failed because of an error. The request is then no longer valid.
	Fail {
		/// The ID that was passed to the worker.
		id: IpfsRequestId,
		/// Error that happened.
		error: rust_ipfs::Error,
	},
}

/// Must be continuously polled for the [`IpfsApi`] to properly work.
pub struct IpfsWorker<I: rust_ipfs::IpfsTypes> {
	/// Used to sends messages to the `IpfsApi`.
	to_api: TracingUnboundedSender<WorkerToApi>,
	/// Used to receive messages from the `IpfsApi`.
	from_api: TracingUnboundedReceiver<ApiToWorker>,
	/// The engine that runs IPFS requests.
	ipfs_node: rust_ipfs::Ipfs<I>,
	/// IPFS requests that are being worked on by the engine.
	requests: Vec<(IpfsRequestId, IpfsWorkerRequest)>,
}

/// IPFS request being processed by the worker.
struct IpfsWorkerRequest(
	Pin<Box<dyn Future<Output = Result<IpfsNativeResponse, rust_ipfs::Error>> + Send>>,
);

#[derive(Debug)]

pub enum IpfsNativeResponse {
	Addrs(Vec<(PeerId, Vec<Multiaddr>)>),
	AddBytes(Cid),
	AddListeningAddr(Multiaddr),
	BitswapStats(BitswapStats),
	CatBytes(Vec<u8>),
	Connect(()),
	Disconnect(()),
	FindPeer(Vec<Multiaddr>),
	GetBlock(Block),
	GetClosestPeers(Vec<PeerId>),
	GetProviders(Vec<PeerId>),
	Identity(PublicKey, Vec<Multiaddr>),
	InsertPin(()),
	LocalAddrs(Vec<Multiaddr>),
	LocalRefs(Vec<Cid>),
	Peers(Vec<Connection>),
	Publish(()),
	RemoveListeningAddr(()),
	RemoveBlock(/*Cid*/),
	RemovePin(()),
	Subscribe(SubscriptionStream), /* TODO: actually using the SubscriptionStream would require
	                                * it to be stored within the node. */
	SubscriptionList(Vec<Vec<u8>>),
	Unsubscribe(bool),
	// a technical placeholder replacing the actual response owned for conversion purposes.
	Success,
}

impl From<IpfsNativeResponse> for IpfsResponse {
	fn from(resp: IpfsNativeResponse) -> Self {
		match resp {
			IpfsNativeResponse::Addrs(resp) => {
				let mut ret = Vec::with_capacity(resp.len());

				for (peer_id, addrs) in resp {
					let peer = peer_id.as_ref().to_bytes(); // Changed from to_vec()
					let mut converted_addrs = Vec::with_capacity(addrs.len());

					for addr in addrs {
						converted_addrs.push(OpaqueMultiaddr(addr.to_string().into_bytes()));
					}

					ret.push((peer, converted_addrs));
				}

				IpfsResponse::Addrs(ret)
			},
			IpfsNativeResponse::AddBytes(cid) =>
				IpfsResponse::AddBytes(cid.to_string().into_bytes()),
			IpfsNativeResponse::BitswapStats(BitswapStats {
				blocks_sent,
				data_sent,
				blocks_received,
				data_received,
				dup_blks_received,
				dup_data_received,
				peers,
				wantlist,
			}) => IpfsResponse::BitswapStats {
				blocks_sent,
				data_sent,
				blocks_received,
				data_received,
				dup_blks_received,
				dup_data_received,
				peers: peers.into_iter().map(|peer_id| peer_id.as_ref().to_bytes()).collect(),
				wantlist: wantlist.into_iter().map(|(cid, prio)| (cid.to_bytes(), prio)).collect(),
			},
			IpfsNativeResponse::CatBytes(data) => IpfsResponse::CatBytes(data),
			IpfsNativeResponse::GetClosestPeers(peer_ids) => {
				let ids =
					peer_ids.into_iter().map(|peer_id| peer_id.to_string().into_bytes()).collect();
				IpfsResponse::GetClosestPeers(ids)
			},
			IpfsNativeResponse::GetProviders(peer_ids) => {
				let ids =
					peer_ids.into_iter().map(|peer_id| peer_id.to_string().into_bytes()).collect();
				IpfsResponse::GetProviders(ids)
			},
			IpfsNativeResponse::FindPeer(addrs) => {
				let addrs = addrs
					.into_iter()
					.map(|addr| OpaqueMultiaddr(addr.to_string().into_bytes()))
					.collect();
				IpfsResponse::FindPeer(addrs)
			},
			IpfsNativeResponse::Identity(pk, addrs) => {
				let pk = pk.to_peer_id().as_ref().to_bytes();

				let addrs = addrs
					.into_iter()
					.map(|addr| OpaqueMultiaddr(addr.to_string().into_bytes()))
					.collect();

				IpfsResponse::Identity(pk, addrs)
			},
			IpfsNativeResponse::LocalAddrs(addrs) => {
				let addrs = addrs
					.into_iter()
					.map(|addr| OpaqueMultiaddr(addr.to_string().into_bytes()))
					.collect();

				IpfsResponse::LocalAddrs(addrs)
			},
			IpfsNativeResponse::LocalRefs(cids) => {
				let cids = cids.into_iter().map(|cid| cid.to_bytes()).collect();

				IpfsResponse::LocalRefs(cids)
			},
			IpfsNativeResponse::Peers(conns) => {
				let addrs = conns
					.into_iter()
					.map(|conn| OpaqueMultiaddr(conn.addr.to_string().into_bytes()))
					.collect();

				IpfsResponse::Peers(addrs)
			},
			IpfsNativeResponse::RemoveBlock(/*cid*/) => {
				//let result: vec<u8> = cid.to_string().into_bytes();
				let result = Vec::<u8>::new();
				IpfsResponse::RemoveBlock(result)
			},
			_ => IpfsResponse::Success,
		}
	}
}

async fn ipfs_request<I: rust_ipfs::IpfsTypes>(
	ipfs: rust_ipfs::Ipfs<I>,
	request: IpfsRequest,
) -> Result<IpfsNativeResponse, rust_ipfs::Error> {
	match request {
		IpfsRequest::Addrs => Ok(IpfsNativeResponse::Addrs(ipfs.addrs().await?)),
		IpfsRequest::AddBytes(data, version) =>
			Ok(IpfsNativeResponse::AddBytes(ipfs_add(&ipfs, data, version).await?)),
		IpfsRequest::AddListeningAddr(addr) => {
			let ret = ipfs.add_listening_address(str::from_utf8(&addr.0)?.parse()?).await?;
			Ok(IpfsNativeResponse::AddListeningAddr(ret))
		},
		IpfsRequest::BitswapStats =>
			Ok(IpfsNativeResponse::BitswapStats(ipfs.bitswap_stats().await?)),
		IpfsRequest::CatBytes(cid) => {
			let data = ipfs_get(&ipfs, str::from_utf8(&cid)?.parse::<IpfsPath>()?).await?;
			Ok(IpfsNativeResponse::CatBytes(data))
		},
		IpfsRequest::Connect(addr) => {
			let addr_str = str::from_utf8(&addr.0)?;
			let addr = addr_str.parse::<MultiaddrWithPeerId>()?;
			Ok(IpfsNativeResponse::Connect(ipfs.connect(addr).await?))
		},
		IpfsRequest::Disconnect(addr) => {
			let addr_str = str::from_utf8(&addr.0)?;
			let addr = addr_str.parse::<MultiaddrWithPeerId>()?;
			Ok(IpfsNativeResponse::Disconnect(ipfs.disconnect(addr.peer_id).await?))
		},
		IpfsRequest::FindPeer(peer_id) => {
			let peer_id = str::from_utf8(&peer_id)?.parse::<PeerId>()?;
			Ok(IpfsNativeResponse::FindPeer(ipfs.find_peer(peer_id).await?))
		},
		IpfsRequest::GetBlock(cid) =>
			Ok(IpfsNativeResponse::GetBlock(ipfs.get_block(&cid.try_into()?).await?)),
		IpfsRequest::GetClosestPeers(peer_id) => {
			let peer_id = str::from_utf8(&peer_id)?.parse::<PeerId>()?;
			Ok(IpfsNativeResponse::GetClosestPeers(ipfs.get_closest_peers(peer_id).await?))
		},
		IpfsRequest::GetProviders(cid) => {
			let cid = str::from_utf8(&cid)?.parse()?;
			let mut stream = ipfs.get_providers(cid).await?.boxed();

			let mut vec = Vec::<PeerId>::new();
			while let Some(_providers) = stream.next().await {
				vec.push(_providers);
			}

			Ok(IpfsNativeResponse::GetProviders(vec))
		},
		IpfsRequest::Identity => {
			let res = ipfs.identity(None).await?;
			 Ok(IpfsNativeResponse::Identity(res.public_key, res.listen_addrs))
		},
		IpfsRequest::InsertPin(cid, recursive) => {
			let cid = str::from_utf8(&cid)?.parse()?;
			Ok(IpfsNativeResponse::InsertPin(ipfs.insert_pin(&cid, recursive).await?))
		},
		IpfsRequest::LocalAddrs => Ok(IpfsNativeResponse::LocalAddrs(ipfs.addrs_local().await?)),
		IpfsRequest::LocalRefs => {

			let resp = ipfs.refs_local().await?;
			Ok(IpfsNativeResponse::LocalRefs(resp))
		},
		IpfsRequest::Peers => {
			Ok(IpfsNativeResponse::Peers(ipfs.peers().await?))
		},
		IpfsRequest::Publish { topic, message } => {
			let ret: () = {ipfs.pubsub_publish(String::from_utf8(topic)?, message).await?;};
			Ok(IpfsNativeResponse::Publish(ret))
		},
		IpfsRequest::RemoveListeningAddr(addr) => {
			let ret = ipfs.remove_listening_address(str::from_utf8(&addr.0)?.parse()?).await?;
			Ok(IpfsNativeResponse::RemoveListeningAddr(ret))
		},
		IpfsRequest::RemoveBlock(/*cid*/) => {
			//let cid = str::from_utf8(&cid)?.parse()?;
			Ok(IpfsNativeResponse::RemoveBlock(/*ipfs.remove_block(cid).await?*/))
		},
		IpfsRequest::RemovePin(cid, recursive) => {
			let cid = str::from_utf8(&cid)?.parse()?;
			Ok(IpfsNativeResponse::RemovePin(ipfs.remove_pin(&cid, recursive).await?))
		},
		IpfsRequest::Subscribe(topic) => {
			let ret = ipfs.pubsub_subscribe(String::from_utf8(topic)?).await?;
			Ok(IpfsNativeResponse::Subscribe(ret))
		},
		IpfsRequest::SubscriptionList => {
			let list =
				ipfs.pubsub_subscribed().await?.into_iter().map(|s| s.into_bytes()).collect();
			Ok(IpfsNativeResponse::SubscriptionList(list))
		},
		IpfsRequest::Unsubscribe(topic) => {
			let ret = ipfs.pubsub_unsubscribe(str::from_utf8(&topic)?).await?;
			Ok(IpfsNativeResponse::Unsubscribe(ret))
		},
	}
}

impl<I: rust_ipfs::IpfsTypes> Future for IpfsWorker<I> {
	type Output = ();

	fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
		// We use a `me` variable because the compiler isn't smart enough to allow borrowing
		// multiple fields at once through a `Deref`.
		let me = &mut *self;

		// We remove each element from `requests` one by one and add them back only if necessary.
		for n in (0..me.requests.len()).rev() {
			let (id, mut request) = me.requests.swap_remove(n);
			match Future::poll(Pin::new(&mut request.0), cx) {
				Poll::Pending => me.requests.push((id, request)),
				Poll::Ready(Ok(value)) => {
					let _ = me.to_api.unbounded_send(WorkerToApi::Response { id, value });
					cx.waker().wake_by_ref(); // reschedule in order to poll the new future
				},
				Poll::Ready(Err(error)) => {
					let _ = me.to_api.unbounded_send(WorkerToApi::Fail { id, error });
				},
			};
		}

		// Check for messages coming from the [`IpfsApi`].
		match Stream::poll_next(Pin::new(&mut me.from_api), cx) {
			Poll::Pending => {},
			Poll::Ready(None) => return Poll::Ready(()), // stops the worker
			Poll::Ready(Some(ApiToWorker { id, request })) => {
				let ipfs_node = me.ipfs_node.clone();
				let future = Box::pin(ipfs_request(ipfs_node, request));
				debug_assert!(me.requests.iter().all(|(i, _)| *i != id));
				me.requests.push((id, IpfsWorkerRequest(future)));
				cx.waker().wake_by_ref(); // reschedule the task to poll the request
			},
		}

		Poll::Pending
	}
}

impl<I: rust_ipfs::IpfsTypes> fmt::Debug for IpfsWorker<I> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_list().entries(self.requests.iter()).finish()
	}
}

impl fmt::Debug for IpfsWorkerRequest {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_tuple("IpfsWorkerRequest").finish()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::api::timestamp;
	use sp_core::offchain::{Duration, IpfsRequest, IpfsRequestStatus, IpfsResponse};

	#[test]
	fn metadata_calls() {
		let options = ipfs::IpfsOptions::inmemory_with_generated_keys();

		let rt = tokio::runtime::Runtime::new().unwrap();
		let ipfs_node = rt.block_on(async move {
			let (ipfs, fut): (Ipfs<ipfs::TestTypes>, _) =
			ipfs::UninitializedIpfs::new(options).start().await.unwrap();
			tokio::task::spawn(fut);
			ipfs
		});

		let (mut api, worker) = ipfs(ipfs_node);

		std::thread::spawn(move || {
			let worker = rt.spawn(worker);
			rt.block_on(worker).unwrap();
		});

		let deadline = timestamp::now().add(Duration::from_millis(10_000));

		let id1 = api.request_start(IpfsRequest::Addrs).unwrap();
		let id2 = api.request_start(IpfsRequest::BitswapStats).unwrap();
		let id3 = api.request_start(IpfsRequest::Identity).unwrap();
		let id4 = api.request_start(IpfsRequest::LocalAddrs).unwrap();
		let id5 = api.request_start(IpfsRequest::Peers).unwrap();

		match api.response_wait(&[id1, id2, id3, id4, id5], Some(deadline)).as_slice() {
			[IpfsRequestStatus::Finished(IpfsResponse::Addrs(..)), IpfsRequestStatus::Finished(IpfsResponse::BitswapStats { .. }), IpfsRequestStatus::Finished(IpfsResponse::Identity(..)), IpfsRequestStatus::Finished(IpfsResponse::LocalAddrs(..)), IpfsRequestStatus::Finished(IpfsResponse::Peers(..))] =>
				{},
			x => panic!("Connecting to the IPFS node failed: {:?}", x),
		}
	}
}
