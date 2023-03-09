use codec::{Decode, Encode};
use frame_system::Config;
use sp_io::{offchain_index};

use sp_runtime::offchain::storage::{StorageValueRef, StorageRetrievalError};
use sp_std::vec::Vec;

#[cfg(feature = "std")]
use frame_support::serde::{Deserialize};

pub const OFFCHAIN_KEY_PREFIX: &[u8] = b"ipfs_core::indexing1";

#[derive(Debug, Encode, Decode, Default)]
#[cfg_attr(feature = "std", derive(Deserialize))]

/// Wrapper class containing the serialized data
///
pub struct OffchainStorageData {
	pub data: Vec<u8>,
	pub meta_data: Vec<u8>

	// can add further data such as enums here
}

impl OffchainStorageData {
	/// creates a new instance with the given data
	///
	pub fn new(data: Vec<u8>, meta_data: Vec<u8>) -> OffchainStorageData {
		OffchainStorageData{data, meta_data }
	}
}

/// Writes data to the offchain storage
///
/// # Arguments
///
/// * `block_number` - Refering block number the data is for
/// * `data` - Data to store
/// * `is_in_offchain_context` - Tells the storage, if the method coall is within onchain or offchain context.
/// 							  That is important, since in both states use different storage APIs and storing fails, it the flag is wrong
///
pub fn set_offchain_data<T: Config>(block_number: T::BlockNumber, data: Vec<u8>, meta_data: Vec<u8>, is_in_offchain_context: bool) {
	let storage_data = OffchainStorageData::new(data, meta_data);
	set_offchain_storage_data::<T>(block_number, &storage_data, is_in_offchain_context);
}

/// Writes data to the offchain storage.
///
/// The OffchainStorageData is a wrapper class around the stored data. You can access the actual storage data through OffchainStorageData.data
///
/// # Arguments
///
/// * `block_number` - Refering block number the data is for
/// * `data` - Data to store
/// * `is_in_offchain_context` - Tells the storage, if the method coall is within onchain or offchain context.
/// 							  That is important, since in both states use different storage APIs and storing fails, it the flag is wrong
///
pub fn set_offchain_storage_data<T: Config>(block_number: T::BlockNumber, offchain_storage_data: &OffchainStorageData, is_in_offchain_context: bool) {
	let key = offchain_data_key::<T>(block_number);
	set_offchain_storage_data_for_key::<T>(&key, offchain_storage_data, is_in_offchain_context);
}

/// Writes OffchainStorageData to the offchain storage
///
/// The OffchainStorageData is a wrapper class around the stored data. You can access the actual storage data through OffchainStorageData.data
///
/// # Arguments
///
/// * `key` - unique key to store the data
/// * `offchain_storage_data` - OffchainStorageData instance to store
/// * `is_in_offchain_context` - Tells the storage, if the method coall is within onchain or offchain context.
/// 							 That is important, since in both states use different storage APIs and storing fails, it the flag is wrong
///
pub fn set_offchain_storage_data_for_key<T: Config>(key: &Vec<u8>, offchain_storage_data: &OffchainStorageData, is_in_offchain_context: bool) {
	if is_in_offchain_context {
		let storage_ref = StorageValueRef::persistent(key);
		storage_ref.set(offchain_storage_data);
	}
	else {
		offchain_index::set(key, &offchain_storage_data.encode());
	}
}

/// Returns stored offchain data for the given block number
///
pub fn offchain_data<T: Config>(block_number: T::BlockNumber) -> Result<OffchainStorageData, StorageRetrievalError>  {
	match offchain_storage_data_for_block_number::<T>(block_number) {
		Ok(data) => {
			let offchain_storage_data = data.expect("expected offchain storage data");
			Ok(offchain_storage_data)
		},
		Err(error) => {
			Err(error)
		}
	}
}
/// Clears existing data for the given key
pub fn clear_offchain_storage_data_for_key<T: Config>(key: &Vec<u8>, is_in_offchain_context: bool) {
	if is_in_offchain_context {
		let mut storage_ref = StorageValueRef::persistent(key);
		storage_ref.clear();
	}
	else {
		offchain_index::clear(key);
	}
}

/// Creates a unique storage key/identifier for a certain stored or to be stored data set
///
/// # Arguments
///
/// * `block_number` - Refering block number to create the key.
///
pub fn offchain_data_key<T: Config>(block_number: T::BlockNumber) -> Vec<u8> {
	block_number.using_encoded(|encoded_bn| {
		OFFCHAIN_KEY_PREFIX.clone().into_iter()
			.chain(b"/".into_iter())
			.chain(encoded_bn)
			.copied()
			.collect::<Vec<u8>>()
	})
}

/// Returns offchain data for the given block number
///
/// The OffchainStorageData is a wrapper class around the stored data. You can access the actual storage data through OffchainStorageData.data
///
pub fn offchain_storage_data_for_block_number<T: Config>(block_number: T::BlockNumber) -> Result<Option<OffchainStorageData>, StorageRetrievalError> {
	let key = offchain_data_key::<T>(block_number);
	let ret_val = offchain_storage_data_for_key(&key);

	ret_val
}

/// Returns the OffchainStorageData instance for the given key
///
/// The OffchainStorageData is a wrapper class around the stored data. You can access the actual storage data through OffchainStorageData.data
///
pub fn offchain_storage_data_for_key(key: &Vec<u8>) -> Result<Option<OffchainStorageData>, StorageRetrievalError> {
	let storage_ref = StorageValueRef::persistent(key);
	let stored_data_result = storage_ref.get::<OffchainStorageData>();

	stored_data_result
}
