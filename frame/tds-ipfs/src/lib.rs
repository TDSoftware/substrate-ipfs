#![cfg_attr(not(feature = "std"), no_std)]

//! Pallet provides basic IPFS functionality such as adding or receiving bytes, connecting to nodes ...
//!
//! Credits goes to:
//!
//! https://github.com/WunderbarNetwork/substrate
//!

use frame_system::offchain::{AppCrypto, CreateSignedTransaction, SendSignedTransaction, Signer};
use frame_support::{ dispatch::DispatchResult};

use log::{info};
use sp_core::offchain::{IpfsRequest, IpfsResponse};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod crypto {
  use sp_core::sr25519::Signature as Sr25519Signature;
  use sp_runtime::{
    app_crypto::{app_crypto, sr25519},
    traits::Verify,
    MultiSignature, MultiSigner,
  };

  app_crypto!(sr25519, sp_core::crypto::key_types::IPFS);
  pub struct TestAuthId;

  impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for TestAuthId {
    type RuntimeAppPublic = Public;
    type GenericPublic = sp_core::sr25519::Public;
    type GenericSignature = sp_core::sr25519::Signature;
  }

  // Implemented for mock runtime in tests
  impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature>
    for TestAuthId
  {
    type RuntimeAppPublic = Public;
    type GenericPublic = sp_core::sr25519::Public;
    type GenericSignature = sp_core::sr25519::Signature;
  }
}

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
  use super::*;

  use frame_support::{pallet_prelude::*};
  use frame_system::pallet_prelude::*;
  use sp_std::vec::Vec;
  use sp_std::{str};

  use pallet_tds_ipfs_core::{
    addresses_to_utf8_safe_bytes, generate_id, ipfs_request, ocw_parse_ipfs_response,
    ocw_process_command,
	CommandRequest, Error as IpfsError, IpfsCommand, TypeEquality,
	storage::set_offchain_data,
  };

  use sp_core::crypto::KeyTypeId;

  pub const KEY_TYPE: KeyTypeId = sp_core::crypto::key_types::IPFS;
  const PROCESSED_COMMANDS: &[u8; 24] = b"ipfs::processed_commands";

  /// Configure the pallet by specifying the parameters and types on which it depends.

  #[pallet::config]
  pub trait Config:
    frame_system::Config + pallet_tds_ipfs_core::Config + CreateSignedTransaction<Call<Self>>
  {
    /// The identifier type for an offchain worker.
    type AuthorityId: AppCrypto<Self::Public, Self::Signature>;

    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

    /// overarching dispatch call type.
	type RuntimeCall: From<Call<Self>>;

	type IpfsRandomness: frame_support::traits::Randomness<Self::Hash, Self::BlockNumber>;
  }

  /// The current storage version.
  const STORAGE_VERSION: frame_support::traits::StorageVersion = frame_support::traits::StorageVersion::new(1);

  #[pallet::pallet]
  #[pallet::generate_store(pub(super) trait Store)]
  #[pallet::storage_version(STORAGE_VERSION)]
  #[pallet::without_storage_info]
  pub struct Pallet<T>(_);


  // The pallet's runtime storage items.
  // https://docs.substrate.io/v3/runtime/storage

  /** Store a list of Commands for the ocw to process */
  #[pallet::storage]
  #[pallet::getter(fn commands)]
  pub type Commands<T: Config> = StorageValue<_, Vec<CommandRequest<T>>>;

  /** Pallets use events to inform users when important changes are made.

     Pre offchain worker
     - ConnectionRequested(T::AccountId)
     - DisconnectedRequested(T::AccountId)
     - QueuedDataToAdd(T::AccountId)
     - QueuedDataToCat(T::AccountId)
     - QueuedDataToPin(T::AccountId)
     - QueuedDataToRemove(T::AccountId)
     - QueuedDataToUnpin(T::AccountId)
     - FindPeerIssued(T::AccountId)
     - FindProvidersIssued(T::AccountId)
     - OcwCallback(T::AccountId)

     //Post offchain worker
     Requester, IpfsConnectionAddress
     - ConnectedTo(T::AccountId, Vec<u8>),
     Requester, IpfsDisconnectionAddress
     - DisconnectedFrom(T::AccountId, Vec<u8>),

     Requester, Cid
     - AddedCid(T::AccountId, Vec<u8>),
     Requester, Cid, Bytes
     - CatBytes(T::AccountId, Vec<u8>, Vec<u8>),

     Requester, Cid
     - InsertedPin(T::AccountId, Vec<u8>),
     Requester, Cid
     - RemovedPin(T::AccountId, Vec<u8>),
     Requester, Cid
     - RemovedBlock(T::AccountId, Vec<u8>),

     Requester, ListOfPeers
     - FoundPeers(T::AccountId, Vec<u8>),
     Requester, Cid, Providers
     - CidProviders(T::AccountId, Vec<u8>, Vec<u8>),
  */
  #[pallet::event]
  #[pallet::generate_deposit(pub(super) fn deposit_event)]
  pub enum Event<T: Config> {
    ConnectionRequested(T::AccountId),
    DisconnectedRequested(T::AccountId),
    QueuedDataToAdd(T::AccountId),
    QueuedDataToCat(T::AccountId),
    QueuedDataToPin(T::AccountId),
    QueuedDataToRemove(T::AccountId),
    QueuedDataToUnpin(T::AccountId),
    FindPeerIssued(T::AccountId),
    FindProvidersIssued(T::AccountId),
    OcwCallback(T::AccountId),

    // Requester, IpfsConnectionAddress
    ConnectedTo(T::AccountId, Vec<u8>),
    // Requester, IpfsDisconnectionAddress
    DisconnectedFrom(T::AccountId, Vec<u8>),
    // Requester, Cid
    AddedCid(T::AccountId, Vec<u8>),
    // Requester, Cid, Bytes
    CatBytes(T::AccountId, Vec<u8>, Vec<u8>),
    // Requester, Cid
    InsertedPin(T::AccountId, Vec<u8>),
    // Requester, Cid
    RemovedPin(T::AccountId, Vec<u8>),
    // Requester, Cid
    RemovedBlock(T::AccountId, Vec<u8>),
    // Requester, ListOfPeers
    FoundPeers(T::AccountId, Vec<u8>),
    // Requester, Cid, Providers
    CidProviders(T::AccountId, Vec<u8>, Vec<u8>),
  }

  /** Errors inform users that something went wrong.
  - RequestFailed,
  */
  #[pallet::error]
  pub enum Error<T> {}

  /** Modify the genesis state of the blockchain.
     This is pretty pointless atm since we overwrite the data each block.

     TODO: Until we have a working signed ocw callback is would mostly likely be used for configuring initial state in specs.

     Optional values are:
     -	commands: Vec<CommandRequest<T>>
  **/
  #[pallet::genesis_config]
  pub struct GenesisConfig<T: Config> {
    pub commands: Vec<CommandRequest<T>>,
  }

  #[cfg(feature = "std")]
  impl<T: Config> Default for GenesisConfig<T> {
    fn default() -> GenesisConfig<T> {
      GenesisConfig { commands: Vec::<CommandRequest<T>>::new() }
    }
  }

  #[pallet::genesis_build]
  impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
    fn build(&self) {
      // TODO: Allow the configs to use strings, then convert them to the Vec<u8> collections.
      Commands::<T>::set(Some(Vec::<CommandRequest<T>>::new()));
    }
  }

  #[pallet::hooks]
  impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
fn offchain_worker(block_number: T::BlockNumber) {
      if let Err(_err) = Self::print_metadata(&"*** IPFS off-chain worker started with ***") {
        log::error!("IPFS: Error occurred during `print_metadata`");
      }

      if let Err(_err) = Self::ocw_process_command_requests(block_number) {
        log::error!("IPFS: command request");
      }
    }
  }

  // Dispatchable functions allows users to interact with the pallet and invoke state changes.
  // These functions materialize as "extrinsics", which are often compared to transactions.
  // Dispatchable functions must be annotated with a weight and must return a DispatchResult.

  #[pallet::call]
  impl<T: Config> Pallet<T> {

	/// Adds arbitrary bytes to the IPFS repository. The registered `Cid` is printed out in the
    /// logs.

	#[pallet::call_index(0)]
    #[pallet::weight(200_000)]
    pub fn add_bytes(origin: OriginFor<T>, received_bytes: Vec<u8>, version: u8) -> DispatchResult {
      let requester = ensure_signed(origin)?;
	  let block_number = frame_system::Pallet::<T>::block_number();
	  set_offchain_data::<T>(block_number, received_bytes);

	  let mut commands = Vec::<IpfsCommand>::new();
      commands.push(IpfsCommand::AddBytes(version));

	  log::info!("IPFS CALL: add_bytes");

      let ipfs_command_request = CommandRequest::<T> {
        identifier: generate_id::<T>(),
        requester: requester.clone(),
        ipfs_commands: commands,
      };

      Commands::<T>::append(ipfs_command_request);
      Ok(Self::deposit_event(Event::QueuedDataToAdd(requester)))
    }

	/** Find IPFS data by the `Cid`; if it is valid UTF-8, it is printed in the logs.
    Otherwise the decimal representation of the bytes is displayed instead. **/

	#[pallet::call_index(1)]
    #[pallet::weight(100_000)]
    pub fn cat_bytes(origin: OriginFor<T>, cid: Vec<u8>) -> DispatchResult {
      let requester = ensure_signed(origin)?;
      let mut commands = Vec::<IpfsCommand>::new();
      commands.push(IpfsCommand::CatBytes(cid));

      let ipfs_command_request = CommandRequest::<T> {
        identifier: generate_id::<T>(),
        requester: requester.clone(),
        ipfs_commands: commands,
      };

      Commands::<T>::append(ipfs_command_request);
      Ok(Self::deposit_event(Event::QueuedDataToCat(requester)))
    }

    /** Mark a `multiaddr` as a desired connection target. The connection target will be established
    during the next run of the off-chain `connection` process.
    - Example: /ip4/127.0.0.1/tcp/4001/p2p/<Ipfs node Id> */

	#[pallet::call_index(2)]
	#[pallet::weight(100_000)]
    pub fn connect(origin: OriginFor<T>, address: Vec<u8>) -> DispatchResult {
      let requester = ensure_signed(origin)?;
      let mut commands = Vec::<IpfsCommand>::new();
      commands.push(IpfsCommand::ConnectTo(address));

      let ipfs_command_request = CommandRequest::<T> {
        identifier: generate_id::<T>(),
        requester: requester.clone(),
        ipfs_commands: commands,
      };

      Commands::<T>::append(ipfs_command_request);
      Ok(Self::deposit_event(Event::ConnectionRequested(requester)))
    }

    /** Queues a `Multiaddr` to be disconnected. The connection will be severed during the next
    run of the off-chain `connection` process.
    **/

	#[pallet::call_index(3)]
    #[pallet::weight(500_000)]
    pub fn disconnect(origin: OriginFor<T>, address: Vec<u8>) -> DispatchResult {
      let requester = ensure_signed(origin)?;
      let mut commands = Vec::<IpfsCommand>::new();
      commands.push(IpfsCommand::DisconnectFrom(address));

      let ipfs_command_request = CommandRequest::<T> {
        identifier: generate_id::<T>(),
        requester: requester.clone(),
        ipfs_commands: commands,
      };

      Commands::<T>::append(ipfs_command_request);
      Ok(Self::deposit_event(Event::DisconnectedRequested(requester)))
    }

    /** Add arbitrary bytes to the IPFS repository. The registered `Cid` is printed in the
     * logs * */

	 #[pallet::call_index(4)]
    #[pallet::weight(300_000)]
    pub fn remove_block(origin: OriginFor<T>, cid: Vec<u8>) -> DispatchResult {
      let requester = ensure_signed(origin)?;
      let mut commands = Vec::<IpfsCommand>::new();
      commands.push(IpfsCommand::RemoveBlock(cid));

      let ipfs_command_request = CommandRequest::<T> {
        identifier: generate_id::<T>(),
        requester: requester.clone(),
        ipfs_commands: commands,
      };

      Commands::<T>::append(ipfs_command_request);
      Ok(Self::deposit_event(Event::QueuedDataToRemove(requester)))
    }

    /** Pin a given `Cid` non-recursively. * */

	#[pallet::call_index(5)]
	#[pallet::weight(100_000)]
    pub fn insert_pin(origin: OriginFor<T>, cid: Vec<u8>) -> DispatchResult {
      let requester = ensure_signed(origin)?;
      let mut commands = Vec::<IpfsCommand>::new();
      commands.push(IpfsCommand::InsertPin(cid));

      let ipfs_command_request = CommandRequest::<T> {
        identifier: generate_id::<T>(),
        requester: requester.clone(),
        ipfs_commands: commands,
      };

      Commands::<T>::append(ipfs_command_request);
      Ok(Self::deposit_event(Event::QueuedDataToPin(requester)))
    }

    /** Unpin a given `Cid` non-recursively. * */

	#[pallet::call_index(6)]
	#[pallet::weight(100_000)]
    pub fn remove_pin(origin: OriginFor<T>, cid: Vec<u8>) -> DispatchResult {
      let requester = ensure_signed(origin)?;
      let mut commands = Vec::<IpfsCommand>::new();
      commands.push(IpfsCommand::RemovePin(cid));

      let ipfs_command_request = CommandRequest::<T> {
        identifier: generate_id::<T>(),
        requester: requester.clone(),
        ipfs_commands: commands,
      };

      Commands::<T>::append(ipfs_command_request);
      Ok(Self::deposit_event(Event::QueuedDataToUnpin(requester)))
    }

    /** Find addresses associated with the given `PeerId` * */

	#[pallet::call_index(7)]
	#[pallet::weight(100_000)]
    pub fn dht_find_peer(origin: OriginFor<T>, peer_id: Vec<u8>) -> DispatchResult {
      let requester = ensure_signed(origin)?;
      let mut commands = Vec::<IpfsCommand>::new();
      commands.push(IpfsCommand::FindPeer(peer_id));

      let ipfs_command_request = CommandRequest::<T> {
        identifier: generate_id::<T>(),
        requester: requester.clone(),
        ipfs_commands: commands,
      };

      Commands::<T>::append(ipfs_command_request);
      Ok(Self::deposit_event(Event::FindPeerIssued(requester)))
    }

    /** Find the list of `PeerId`'s known to be hosting the given `Cid` * */

	#[pallet::call_index(8)]
	#[pallet::weight(100_000)]
    pub fn dht_find_providers(origin: OriginFor<T>, cid: Vec<u8>) -> DispatchResult {
      let requester = ensure_signed(origin)?;
      let mut commands = Vec::<IpfsCommand>::new();
      commands.push(IpfsCommand::GetProviders(cid));

      let ipfs_command_request = CommandRequest::<T> {
        identifier: generate_id::<T>(),
        requester: requester.clone(),
        ipfs_commands: commands,
      };

      Commands::<T>::append(ipfs_command_request);
      Ok(Self::deposit_event(Event::FindProvidersIssued(requester)))
    }

	#[pallet::call_index(9)]
    #[pallet::weight(0)]
    pub fn ocw_callback(
      origin: OriginFor<T>,
      identifier: [u8; 32],
      data: Vec<u8>,
    ) -> DispatchResult {
      let signer = ensure_signed(origin)?;
      let mut callback_command: Option<CommandRequest<T>> = None;

      Commands::<T>::mutate(|command_requests| {
        let mut commands = command_requests.clone().unwrap();

        if let Some(index) = commands.iter().position(|cmd| cmd.identifier == identifier) {
          info!("Removing at index {}", index.clone());
          callback_command = Some(commands.swap_remove(index).clone());
        };

        *command_requests = Some(commands);
      });

      Self::deposit_event(Event::OcwCallback(signer));

      match Self::command_callback(&callback_command.unwrap(), data) {
		Ok(_) => Ok(()),
		Err(_) => Err(DispatchError::Corruption),
	  }
    }
  }

  impl<T: Config> Pallet<T> {

    /**
       Iterate over all of the Active CommandRequests calling them.
    */
    fn ocw_process_command_requests(block_number: T::BlockNumber) -> Result<(), Error<T>> {
      let commands: Vec<CommandRequest<T>> =
        Commands::<T>::get().unwrap_or(Vec::<CommandRequest<T>>::new());

      for command_request in commands {
		if contains_value_of_type_in_vector(&IpfsCommand::AddBytes(0), &command_request.ipfs_commands) {
			log::info!("IPFS CALL: ocw_process_command_requests for Add Bytes");
		}

		match ocw_process_command::<T>(
          block_number,
          command_request.clone(),
          PROCESSED_COMMANDS,
        ) {
          Ok(responses) => {
            let callback_response = ocw_parse_ipfs_response::<T>(responses);
            _ = Self::signed_callback(&command_request, callback_response);
          },
          Err(e) => match e {
            IpfsError::<T>::RequestFailed => {
				log::error!("IPFS: failed to perform a request")
            },
            _ => {},
          },
        }
      }

      Ok(())
    }

    /** Output the current state of IPFS worker */
    fn print_metadata(message: &str) -> Result<(), IpfsError<T>> {
      let peers = if let IpfsResponse::Peers(peers) = ipfs_request::<T>(IpfsRequest::Peers)? {
        peers
      } else {
        Vec::new()
      };

      info!("{}", message);
      info!("IPFS: Is currently connected to {} peers", peers.len());
      if !peers.is_empty() {
        info!("IPFS: Peer Ids: {:?}", str::from_utf8(&addresses_to_utf8_safe_bytes(peers)))
      }

      info!("IPFS: CommandRequest size: {}", Commands::<T>::decode_len().unwrap_or(0));
      Ok(())
    }

    /** callback to the on-chain validators to continue processing the CID  * */
    fn signed_callback(
      command_request: &CommandRequest<T>,
      data: Vec<u8>,
    ) -> Result<(), IpfsError<T>> {
      let signer = Signer::<T, T::AuthorityId>::all_accounts();

	  if !signer.can_sign() {
        log::error!("*** IPFS *** ---- No local accounts available. Consider adding one via `author_insertKey` RPC.");
        return Err(IpfsError::<T>::RequestFailed)?
      }

      let results = signer.send_signed_transaction(|_account| Call::ocw_callback {
        identifier: command_request.identifier,
        data: data.clone(),
      });

	  if contains_value_of_type_in_vector(&IpfsCommand::AddBytes(0), &command_request.ipfs_commands) {
		log::info!("IPFS CALL: signed_callback for Add Bytes");
	  }

      for (_account, result) in &results {
        match result {
          Ok(()) => {
            info!("callback sent")
          },
          Err(e) => {
            log::error!("Failed to submit transaction {:?}", e)
          },
        }
      }

      Ok(())
    }

    //
    // - Ideally the callback function can be override by another pallet that is coupled to this
    //   one allowing for custom functionality.
    // - data can be used for callbacks IE add a cid to the signer / uploader.
    // - Validate a connected peer has the CID, and which peer has it etc.

    fn command_callback(command_request: &CommandRequest<T>, data: Vec<u8>) -> Result<(), ()> {
      if let Ok(utf8_str) = str::from_utf8(&*data) {
        info!("Received string: {:?}", utf8_str);
      } else {
        info!("Received data: {:?}", data);
      }

	  if contains_value_of_type_in_vector(&IpfsCommand::AddBytes(0), &command_request.ipfs_commands) {
		log::info!("IPFS CALL: command_callback for Add Bytes");
	  }

      for command in command_request.clone().ipfs_commands {
        match command {
          IpfsCommand::ConnectTo(address) => Self::deposit_event(Event::ConnectedTo(
            command_request.clone().requester,
            address,
          )),

          IpfsCommand::DisconnectFrom(address) => Self::deposit_event(
            Event::DisconnectedFrom(command_request.clone().requester, address),
          ),
          IpfsCommand::AddBytes( _ ) => Self::deposit_event(Event::AddedCid(
            command_request.clone().requester,
            data.clone(),
          )),

          IpfsCommand::CatBytes(cid) => Self::deposit_event(Event::CatBytes(
            command_request.clone().requester,
            cid,
            data.clone(),
          )),

          IpfsCommand::InsertPin(cid) => Self::deposit_event(Event::InsertedPin(
            command_request.clone().requester,
            cid,
          )),

          IpfsCommand::RemoveBlock(cid) => Self::deposit_event(Event::RemovedBlock(
            command_request.clone().requester,
            cid,
          )),

          IpfsCommand::RemovePin(cid) => Self::deposit_event(Event::RemovedPin(
            command_request.clone().requester,
            cid,
          )),

          IpfsCommand::FindPeer(_) => Self::deposit_event(Event::FoundPeers(
            command_request.clone().requester,
            data.clone(),
          )),

          IpfsCommand::GetProviders(cid) => Self::deposit_event(Event::CidProviders(
            command_request.clone().requester,
            cid,
            data.clone(),
          )),
        }
      }

      Ok(())

    }
  }

  fn find_value_of_type_in_vector<T : TypeEquality + Clone>(value: &T, vector: &Vec<T>) -> Option<T> {
	let found_value = vector.iter().find(|curr_value| {
		value.eq_type(*curr_value)
  	});

	let ret_val: Option<T> = match found_value {
		Some(value) => Some(value.clone()),
		None => None
	};

	ret_val
  }

  fn contains_value_of_type_in_vector<T : TypeEquality + Clone>(value: &T, vector: &Vec<T>) -> bool {
	let ret_val = match find_value_of_type_in_vector(value, vector) {
		Some(_) => true,
		None => false,
	};

	ret_val
  }


  #[test]
fn test_find_value_of_type_in_vector() {
	let cmd_add = IpfsCommand::AddBytes(vec![1,2,3,4,5], 1);
	let cmd_add_two = IpfsCommand::AddBytes(vec![6,2,4,4,5], 0);
	let cmd_cat = IpfsCommand::CatBytes(vec![3,4,5,5,6]);
	let cmd_connect = IpfsCommand::ConnectTo(vec![3,4,5,5,6]);
	let cmd_disconnect = IpfsCommand::DisconnectFrom(vec![3,4,5,5,6]);

	let vec = vec![cmd_add.clone(), cmd_cat, cmd_connect];

	assert!(find_value_of_type_in_vector(&cmd_add, &vec) != None);
	assert!(find_value_of_type_in_vector(&cmd_add_two, &vec) != None);
	assert!(find_value_of_type_in_vector(&cmd_disconnect, &vec) == None);
}

#[test]
fn test_contains_value_of_type_in_vector() {
	let cmd_add = IpfsCommand::AddBytes(vec![1,2,3,4,5], 1);
	let cmd_add_two = IpfsCommand::AddBytes(vec![6,2,4,4,5], 0);
	let cmd_cat = IpfsCommand::CatBytes(vec![3,4,5,5,6]);
	let cmd_connect = IpfsCommand::ConnectTo(vec![3,4,5,5,6]);
	let cmd_disconnect = IpfsCommand::DisconnectFrom(vec![3,4,5,5,6]);

	let vec = vec![cmd_add.clone(), cmd_cat, cmd_connect];

	assert!(contains_value_of_type_in_vector(&cmd_add, &vec));
	assert!(contains_value_of_type_in_vector(&cmd_add_two, &vec));
	assert!(contains_value_of_type_in_vector(&cmd_disconnect, &vec) == false);
}


}
