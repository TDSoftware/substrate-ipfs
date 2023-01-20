#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>

pub use pallet::*;

use codec::{Decode, Encode};
use sp_runtime::{
  offchain::{
    ipfs,
    storage::{MutateStorageError, StorageRetrievalError, StorageValueRef},
  },
  RuntimeDebug,
};

#[cfg(feature = "std")]
use frame_support::serde::{Deserialize, Serialize};

use log::info;
use sp_core::offchain::{Duration, IpfsRequest, IpfsResponse, OpaqueMultiaddr};
use sp_std::{str, vec::Vec};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use frame_support::traits::Randomness;


/** Create a "unique" id for each command

   Note: Nodes on the network will come to the same value for each id.
*/
pub fn generate_id<T: Config>() -> [u8; 32] {
  let payload = (

	T::IpfsRandomness::random(&b"ipfs-request-id"[..]).0,
	<frame_system::Pallet<T>>::block_number(),
  );
  payload.using_encoded(sp_io::hashing::blake2_256)
}

/** Process each IPFS `command_request` in the offchain worker
1) lock the request for asynchronous processing
2) Call each command in CommandRequest.ipfs_commands
  - Make sure each command is successfully before attempting the next
 */
pub fn ocw_process_command<T: Config>(
  block_number: T::BlockNumber,
  command_request: CommandRequest<T>,
  persistence_key: &[u8; 24],
) -> Result<Vec<IpfsResponse>, Error<T>> {
  // TODO: make the lock optional: Not all requests may need a lock
  let acquire_lock = acquire_command_request_lock::<T>(block_number, &command_request);

  match acquire_lock {
    Ok(_block) => {
      let mut result = Vec::<IpfsResponse>::new();

      for command in command_request.clone().ipfs_commands {
        match command {
          IpfsCommand::ConnectTo(ref address) => {
            match ipfs_request::<T>(IpfsRequest::Connect(OpaqueMultiaddr(
              address.clone(),
            ))) {
              Ok(IpfsResponse::Success) => Ok(result.push(IpfsResponse::Success)),
              _ => Err(Error::<T>::RequestFailed),
            }
          },

          IpfsCommand::DisconnectFrom(ref address) => {
            match ipfs_request::<T>(IpfsRequest::Disconnect(OpaqueMultiaddr(
              address.clone(),
            ))) {
              Ok(IpfsResponse::Success) => Ok(result.push(IpfsResponse::Success)),
              _ => Err(Error::<T>::RequestFailed),
            }
          },

          IpfsCommand::AddBytes(ref bytes_to_add) => {
            match ipfs_request::<T>(IpfsRequest::AddBytes(bytes_to_add.clone())) {
              Ok(IpfsResponse::AddBytes(cid)) =>
                Ok(result.push(IpfsResponse::AddBytes(cid))),
              _ => Err(Error::<T>::RequestFailed),
            }
          },

          IpfsCommand::CatBytes(ref cid) => {
            match ipfs_request::<T>(IpfsRequest::CatBytes(cid.clone())) {
              Ok(IpfsResponse::CatBytes(bytes_received)) =>
                Ok(result.push(IpfsResponse::CatBytes(bytes_received))),
              _ => Err(Error::<T>::RequestFailed),
            }
          },

          IpfsCommand::InsertPin(ref cid) => {
            match ipfs_request::<T>(IpfsRequest::InsertPin(cid.clone(), false)) {
              Ok(IpfsResponse::Success) => Ok(result.push(IpfsResponse::Success)),
              _ => Err(Error::<T>::RequestFailed),
            }
          },

          IpfsCommand::RemovePin(ref cid) => {
            match ipfs_request::<T>(IpfsRequest::RemovePin(cid.clone(), false)) {
              Ok(IpfsResponse::Success) => Ok(result.push(IpfsResponse::Success)),
              _ => Err(Error::<T>::RequestFailed),
            }
          },

          IpfsCommand::RemoveBlock(ref cid) => {
            match ipfs_request::<T>(IpfsRequest::RemoveBlock(cid.clone())) {
              Ok(IpfsResponse::Success) => Ok(result.push(IpfsResponse::Success)),
              _ => Err(Error::<T>::RequestFailed),
            }
          },
          IpfsCommand::FindPeer(ref peer_id) => {
            match ipfs_request::<T>(IpfsRequest::FindPeer(peer_id.clone())) {
              Ok(IpfsResponse::FindPeer(addresses)) =>
                Ok(result.push(IpfsResponse::FindPeer(addresses))),
              _ => Err(Error::<T>::RequestFailed),
            }
          },
          IpfsCommand::GetProviders(ref cid) => {
            match ipfs_request::<T>(IpfsRequest::GetProviders(cid.clone())) {
              Ok(IpfsResponse::GetProviders(peer_ids)) =>
                Ok(result.push(IpfsResponse::GetProviders(peer_ids))),
              _ => Err(Error::<T>::RequestFailed),
            }
          },
        };
      }

      processed_commands::<T>(&command_request, persistence_key);

      Ok(result)
    },
    _ => Err(Error::<T>::FailedToAcquireLock),
  }
}
/** Send a request to the local IPFS node; Can only be called in an offchain worker. * */
pub fn ipfs_request<T: Config>(request: IpfsRequest) -> Result<IpfsResponse, Error<T>> {
  let ipfs_request =
    ipfs::PendingRequest::new(request).map_err(|_| Error::CannotCreateRequest)?;

   info!("ipfs_request {:?}", ipfs_request);

  // TODO: make milliseconds a const
  ipfs_request
    .try_wait(Some(sp_io::offchain::timestamp().add(Duration::from_millis(1_200))))
    .map_err(|_| Error::<T>::RequestTimeout)?
    .map(|req| req.response)
    .map_err(|_error| Error::<T>::RequestFailed)
}

/** Parse Each ipfs response resulting in bytes to be used in callback
  - If multiple responses are found the last response with bytes is returned. ( TODO: handle multiple responses )
*/
pub fn ocw_parse_ipfs_response<T: Config>(responses: Vec<IpfsResponse>) -> Vec<u8> {
  let mut callback_response = Vec::<u8>::new();

  // TODO: Return a complete data response for each of the processed commands.
  //  - possibly double parsing of the response from the client,
  // 		however the abstraction could be better for other uses?
  // 	- Return multiple responses worth of data.
  for response in responses.clone() {
    match response {
      IpfsResponse::CatBytes(bytes_received) =>
        if bytes_received.len() > 1 {
          callback_response = bytes_received
        },
      IpfsResponse::AddBytes(cid) | IpfsResponse::RemoveBlock(cid) => callback_response = cid,

      IpfsResponse::GetClosestPeers(peer_ids) | IpfsResponse::GetProviders(peer_ids) =>
        callback_response = multiple_bytes_to_utf8_safe_bytes(peer_ids),

      IpfsResponse::FindPeer(addresses) |
      IpfsResponse::LocalAddrs(addresses) |
      IpfsResponse::Peers(addresses) => callback_response = addresses_to_utf8_safe_bytes(addresses),

      IpfsResponse::LocalRefs(refs) =>
        callback_response = multiple_bytes_to_utf8_safe_bytes(refs),
      IpfsResponse::Addrs(_) => {},
      IpfsResponse::BitswapStats { .. } => {},
      IpfsResponse::Identity(_, _) => {},
      IpfsResponse::Success => {},
    }
  }

  callback_response
}

/** Convert a vector of addresses into a comma seperated utf8 safe vector of bytes */
pub fn addresses_to_utf8_safe_bytes(addresses: Vec<OpaqueMultiaddr>) -> Vec<u8> {
  multiple_bytes_to_utf8_safe_bytes(addresses.iter().map(|addr| addr.0.clone()).collect())
}

/** Flatten a Vector of bytes into a comma seperated utf8 safe vector of bytes */
pub fn multiple_bytes_to_utf8_safe_bytes(response: Vec<Vec<u8>>) -> Vec<u8> {
  let mut bytes = Vec::<u8>::new();

  for res in response {
    match str::from_utf8(&res) {
      Ok(str) =>
        if bytes.len() == 0 {
          bytes = Vec::from(str.as_bytes());
        } else {
          bytes = [bytes, Vec::from(str.as_bytes())].join(", ".as_bytes());
        },
      _ => {},
    }
  }

  bytes
}

/** Using the CommandRequest<T>.identifier we can attempt to create a lock via StorageValueRef,
leaving behind a block number of when the lock was formed. */
fn acquire_command_request_lock<T: Config>(
  block_number: T::BlockNumber,
  command_request: &CommandRequest<T>,
) -> Result<T::BlockNumber, MutateStorageError<T::BlockNumber, Error<T>>> {
  let storage = StorageValueRef::persistent(&command_request.identifier);

  storage.mutate(|command_identifier: Result<Option<T::BlockNumber>, StorageRetrievalError>| {
    match command_identifier {
      Ok(Some(block)) =>
        if block_number != block {
          info!("Lock failed, lock was not in current block. block_number: {:?}, block: {:?}",block_number, block );
		// TODO: this is wrong, remove OK and uncomment Err again
		  //Err(Error::<T>::FailedToAcquireLock)
		  Ok(block_number)
        } else {
          Ok(block)
        },
      _ => {
        info!("IPFS: Acquired lock!");
        Ok(block_number)
      },
    }
  })
}

/** Store a list of command identifiers to remove the lock in a following block */
fn processed_commands<T: Config>(
  command_request: &CommandRequest<T>,
  persistence_key: &[u8; 24],
) -> Result<Vec<[u8; 32]>, MutateStorageError<Vec<[u8; 32]>, ()>> {
  let processed_commands = StorageValueRef::persistent(persistence_key);

  processed_commands.mutate(
    |processed_commands: Result<Option<Vec<[u8; 32]>>, StorageRetrievalError>| {
      match processed_commands {
        Ok(Some(mut commands)) => {
          commands.push(command_request.identifier);

          Ok(commands)
        },
        _ => {
          let mut res = Vec::<[u8; 32]>::new();
          res.push(command_request.identifier);

          Ok(res)
        },
      }
    },
  )
}

#[frame_support::pallet]
pub mod pallet {
  use super::*;
  use frame_support::pallet_prelude::*;

  /// Configure the pallet by specifying the parameters and types on which it depends.
  #[pallet::config]
  pub trait Config: frame_system::Config {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

    type IpfsRandomness: frame_support::traits::Randomness<Self::Hash, Self::BlockNumber>;
  }

  #[pallet::pallet]
  #[pallet::generate_store(pub(super) trait Store)]
  pub struct Pallet<T>(_);

  /** Commands for interacting with IPFS

  Connection Commands:
  - ConnectTo(OpaqueMultiaddr)
  - DisconnectFrom(OpaqueMultiaddr)

  Data Commands:
  - AddBytes(Vec<u8>)
  - CatBytes(Vec<u8>)
  - InsertPin(Vec<u8>)
  - RemoveBlock(Vec<u8>)
  - RemovePin(Vec<u8>)

   Dht Commands:
   - FindPeer(Vec<u8>)
  - GetProviders(Vec<u8>)*/
  #[derive(PartialEq, Eq, Clone, Debug, Encode, Decode, TypeInfo)]
  #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
  pub enum IpfsCommand {
    // Connection Commands
    ConnectTo(Vec<u8>),
    DisconnectFrom(Vec<u8>),

    // Data Commands
    AddBytes(Vec<u8>),
    CatBytes(Vec<u8>),
    InsertPin(Vec<u8>),
    RemoveBlock(Vec<u8>),
    RemovePin(Vec<u8>),

    // DHT Commands
    FindPeer(Vec<u8>),
    GetProviders(Vec<u8>),
  }

  /** CommandRequest is used for issuing requests to an ocw that has IPFS within its runtime.

    - identifier: [u8; 32]
    - requester: T::AccountId
    - ipfs_commands Vec<IpfsCommand>
  **/
  #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
  #[scale_info(skip_type_params(T))]
  #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
  pub struct CommandRequest<T: Config> {
    pub identifier: [u8; 32],
    pub requester: T::AccountId,
    pub ipfs_commands: Vec<IpfsCommand>,
  }

  #[pallet::event]
  #[pallet::generate_deposit(pub(super) fn deposit_event)]
  pub enum Event<T: Config> {
  }

  /** Errors inform users that something went wrong.
  - CannotCreateRequest,
  - RequestTimeout,
  - RequestFailed,
  - FailedToAcquireLock,
  */
  #[pallet::error]
  pub enum Error<T> {
    CannotCreateRequest,
    RequestTimeout,
    RequestFailed,
    FailedToAcquireLock,
  }

  // Dispatchable functions allows users to interact with the pallet and invoke state changes.
  // These functions materialize as "extrinsics", which are often compared to transactions.
  // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
  #[pallet::call]
  impl<T: Config> Pallet<T> {}
}
