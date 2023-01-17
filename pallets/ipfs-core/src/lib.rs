#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>


// use sp_runtime::{
//   offchain::{
//     ipfs,
//     storage::{MutateStorageError, StorageRetrievalError, StorageValueRef},
//   },
//   RuntimeDebug,
// };


#[cfg(feature = "std")]
use frame_support::{
	serde::{Deserialize, Serialize}
};


// use log::info;
// use sp_core::offchain::{Duration, IpfsRequest, IpfsResponse, OpaqueMultiaddr};
// use sp_std::{str, vec::Vec};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub use pallet::*;

/** Create a "unique" id for each command

   Note: Nodes on the network will come to the same value for each id.
*/
pub fn generate_id<T: Config>() -> [u8; 32] {
  let payload = (
	0,
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
) -> Result<Vec<sp_core::offchain::IpfsResponse>, Error<T>> {
  // TODO: make the lock optional: Not all requests may need a lock
  let acquire_lock = acquire_command_request_lock::<T>(block_number, &command_request);

  match acquire_lock {
    Ok(_block) => {
      let mut result = Vec::<sp_core::offchain::IpfsResponse>::new();

      for command in command_request.clone().ipfs_commands {
        match command {
          IpfsCommand::ConnectTo(ref address) => {
            match ipfs_request::<T>(sp_core::offchain::IpfsRequest::Connect(sp_core::offchain::OpaqueMultiaddr(
              address.clone(),
            ))) {
              Ok(sp_core::offchain::IpfsResponse::Success) => Ok(result.push(sp_core::offchain::IpfsResponse::Success)),
              _ => Err(Error::<T>::RequestFailed),
            }
          },

          IpfsCommand::DisconnectFrom(ref address) => {
            match ipfs_request::<T>(sp_core::offchain::IpfsRequest::Disconnect(sp_core::offchain::OpaqueMultiaddr(
              address.clone(),
            ))) {
              Ok(sp_core::offchain::IpfsResponse::Success) => Ok(result.push(sp_core::offchain::IpfsResponse::Success)),
              _ => Err(Error::<T>::RequestFailed),
            }
          },

          IpfsCommand::AddBytes(ref bytes_to_add) => {
            match ipfs_request::<T>(sp_core::offchain::IpfsRequest::AddBytes(bytes_to_add.clone())) {
              Ok(sp_core::offchain::IpfsResponse::AddBytes(cid)) =>
                Ok(result.push(sp_core::offchain::IpfsResponse::AddBytes(cid))),
              _ => Err(Error::<T>::RequestFailed),
            }
          },

          IpfsCommand::CatBytes(ref cid) => {
            match ipfs_request::<T>(sp_core::offchain::IpfsRequest::CatBytes(cid.clone())) {
              Ok(sp_core::offchain::IpfsResponse::CatBytes(bytes_received)) =>
                Ok(result.push(sp_core::offchain::IpfsResponse::CatBytes(bytes_received))),
              _ => Err(Error::<T>::RequestFailed),
            }
          },

          IpfsCommand::InsertPin(ref cid) => {
            match ipfs_request::<T>(sp_core::offchain::IpfsRequest::InsertPin(cid.clone(), false)) {
              Ok(sp_core::offchain::IpfsResponse::Success) => Ok(result.push(sp_core::offchain::IpfsResponse::Success)),
              _ => Err(Error::<T>::RequestFailed),
            }
          },

          IpfsCommand::RemovePin(ref cid) => {
            match ipfs_request::<T>(sp_core::offchain::IpfsRequest::RemovePin(cid.clone(), false)) {
              Ok(sp_core::offchain::IpfsResponse::Success) => Ok(result.push(sp_core::offchain::IpfsResponse::Success)),
              _ => Err(Error::<T>::RequestFailed),
            }
          },

          IpfsCommand::RemoveBlock(ref cid) => {
            match ipfs_request::<T>(sp_core::offchain::IpfsRequest::RemoveBlock(cid.clone())) {
              Ok(sp_core::offchain::IpfsResponse::Success) => Ok(result.push(sp_core::offchain::IpfsResponse::Success)),
              _ => Err(Error::<T>::RequestFailed),
            }
          },
          IpfsCommand::FindPeer(ref peer_id) => {
            match ipfs_request::<T>(sp_core::offchain::IpfsRequest::FindPeer(peer_id.clone())) {
              Ok(sp_core::offchain::IpfsResponse::FindPeer(addresses)) =>
                Ok(result.push(sp_core::offchain::IpfsResponse::FindPeer(addresses))),
              _ => Err(Error::<T>::RequestFailed),
            }
          },
          IpfsCommand::GetProviders(ref cid) => {
            match ipfs_request::<T>(sp_core::offchain::IpfsRequest::GetProviders(cid.clone())) {
              Ok(sp_core::offchain::IpfsResponse::GetProviders(peer_ids)) =>
                Ok(result.push(sp_core::offchain::IpfsResponse::GetProviders(peer_ids))),
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
pub fn ipfs_request<T: Config>(request: sp_core::offchain::IpfsRequest) -> Result<sp_core::offchain::IpfsResponse, Error<T>> {
  let ipfs_request =
  sp_runtime::offchain::ipfs::PendingRequest::new(request).map_err(|_| Error::CannotCreateRequest)?;

  // TODO: make milliseconds a const
  ipfs_request
    .try_wait(Some(sp_io::offchain::timestamp().add(sp_core::offchain::Duration::from_millis(1_200))))
    .map_err(|_| Error::<T>::RequestTimeout)?
    .map(|req| req.response)
    .map_err(|_error| Error::<T>::RequestFailed)
}

/** Parse Each ipfs response resulting in bytes to be used in callback
  - If multiple responses are found the last response with bytes is returned. ( TODO: handle multiple responses )
*/
pub fn ocw_parse_ipfs_response<T: Config>(responses: Vec<sp_core::offchain::IpfsResponse>) -> Vec<u8> {
  let mut callback_response = Vec::<u8>::new();

  // TODO: Return a complete data response for each of the processed commands.
  //  - possibly double parsing of the response from the client,
  // 		however the abstraction could be better for other uses?
  // 	- Return multiple responses worth of data.
  for response in responses.clone() {
    match response {
      sp_core::offchain::IpfsResponse::CatBytes(bytes_received) =>
        if bytes_received.len() > 1 {
          callback_response = bytes_received
        },
      sp_core::offchain::IpfsResponse::AddBytes(cid) | sp_core::offchain::IpfsResponse::RemoveBlock(cid) => callback_response = cid,

      sp_core::offchain::IpfsResponse::GetClosestPeers(peer_ids) | sp_core::offchain::IpfsResponse::GetProviders(peer_ids) =>
        callback_response = multiple_bytes_to_utf8_safe_bytes(peer_ids),

      sp_core::offchain::IpfsResponse::FindPeer(addresses) |
      sp_core::offchain::IpfsResponse::LocalAddrs(addresses) |
      sp_core::offchain::IpfsResponse::Peers(addresses) => callback_response = addresses_to_utf8_safe_bytes(addresses),

      sp_core::offchain::IpfsResponse::LocalRefs(refs) =>
        callback_response = multiple_bytes_to_utf8_safe_bytes(refs),
      sp_core::offchain::IpfsResponse::Addrs(_) => {},
      sp_core::offchain::IpfsResponse::BitswapStats { .. } => {},
      sp_core::offchain::IpfsResponse::Identity(_, _) => {},
      sp_core::offchain::IpfsResponse::Success => {},
    }
  }

  callback_response
}

/** Convert a vector of addresses into a comma seperated utf8 safe vector of bytes */
pub fn addresses_to_utf8_safe_bytes(addresses: Vec<sp_core::offchain::OpaqueMultiaddr>) -> Vec<u8> {
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
) -> Result<T::BlockNumber, sp_runtime::offchain::storage::MutateStorageError<T::BlockNumber, Error<T>>> {
  let storage = sp_runtime::offchain::storage::StorageValueRef::persistent(&command_request.identifier);

  storage.mutate(|command_identifier: Result<Option<T::BlockNumber>, sp_runtime::offchain::storage::StorageRetrievalError>| {
    match command_identifier {
      Ok(Some(block)) =>
        if block_number != block {
		log::info!("Lock failed, lock was not in current block");
		Err(Error::<T>::FailedToAcquireLock)

        } else {
          Ok(block)
        },
      _ => {
        log::info!("IPFS: Acquired lock!");
        Ok(block_number)
      },
    }
  })
}

/** Store a list of command identifiers to remove the lock in a following block */
fn processed_commands<T: Config>(
  command_request: &CommandRequest<T>,
  persistence_key: &[u8; 24],
) -> Result<Vec<[u8; 32]>, sp_runtime::offchain::storage::MutateStorageError<Vec<[u8; 32]>, ()>> {
  let processed_commands = sp_runtime::offchain::storage::StorageValueRef::persistent(persistence_key);

  processed_commands.mutate(
    |processed_commands: Result<Option<Vec<[u8; 32]>>, sp_runtime::offchain::storage::StorageRetrievalError>| {
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
    // type IpfsRandomness: frame_support::traits::Randomness<Self::Hash,Self::BlockNumber> ;
  }

  #[pallet::pallet]
  #[pallet::generate_store(pub(super) trait Store)]
  pub struct Pallet<T>(_);

  /** Commands for interacting with IPFS

  Connection Commands:
  - ConnectTo(sp_core::offchain::OpaqueMultiaddr)
  - DisconnectFrom(sp_core::offchain::OpaqueMultiaddr)

  Data Commands:
  - AddBytes(Vec<u8>)
  - CatBytes(Vec<u8>)
  - InsertPin(Vec<u8>)
  - RemoveBlock(Vec<u8>)
  - RemovePin(Vec<u8>)

   Dht Commands:
   - FindPeer(Vec<u8>)
  - GetProviders(Vec<u8>)*/

  #[derive(PartialEq, Eq, Clone, Debug, codec::Encode, codec::Decode, scale_info::TypeInfo)]
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
  #[derive(Clone, codec::Encode, codec::Decode, PartialEq, sp_runtime::RuntimeDebug, scale_info::TypeInfo)]
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



  // Dispatchable functions allows users to interact with the pallet and invoke state changes.
  // These functions materialize as "extrinsics", which are often compared to transactions.
  // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
  #[pallet::call]
  impl<T: Config> Pallet<T> {}


	#[pallet::error]
	pub enum Error<T> {
	  CannotCreateRequest,
	  RequestTimeout,
	  RequestFailed,
	  FailedToAcquireLock,
	}
}


