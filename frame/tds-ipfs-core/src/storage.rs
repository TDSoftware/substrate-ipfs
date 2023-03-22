#![allow(unused_imports)]
use codec::{Decode, Encode};
use frame_system::Config;
use sp_io::{offchain_index};

use sp_runtime::offchain::storage::{StorageValueRef, StorageRetrievalError};
use sp_std::vec::Vec;

#[cfg(feature = "std")]
use frame_support::serde::{Deserialize};
pub use crate::types::Offchain_Data;

pub const OFFCHAIN_KEY_PREFIX: &[u8] = b"ipfs_core::indexing1";

/// Writes cid and meta data to the offchain storage, uses block number to create the key
///
/// # Arguments
///
/// * `block_number` - Referring block number the data is for
/// * `cid` - the cid as byte vector
/// * `meta_data` - referring meta data as byte vector
/// * `is_in_offchain_context` - Tells the storage, if the method call is within onchain or offchain context.
/// 							  That is important, since in both states use different storage APIs and storing fails, it the flag is wrong
///

pub fn store_cid_data_for_values<T: Config>(block_number: T::BlockNumber, cid: Vec<u8>, meta_data: Vec<u8>, is_in_offchain_context: bool) {
	let cid_data = Offchain_Data::new(cid, meta_data);
	store_cid_data::<T>(block_number, &cid_data, is_in_offchain_context);
}

/// Writes cid data to the offchain storage with block number as key.
///
/// # Arguments
///
/// * `block_number` - Referring block number the data is for
/// * `cid_data` - cid_data to store
/// * `is_in_offchain_context` - Tells the storage, if the method call is within onchain or offchain context.
/// 							  That is important, since in both states use different storage APIs and storing fails, it the flag is wrong
///
pub fn store_cid_data<T: Config>(block_number: T::BlockNumber, cid_data: &Offchain_Data, is_in_offchain_context: bool) {
	let key = offchain_data_key::<T>(block_number);
	store_cid_data_for_key::<T>(&key, cid_data, is_in_offchain_context);
}

/// Writes cid data to the offchain storage with the given key.
///
///
/// # Arguments
///
/// * `key` - unique key to store the data
/// * `cid_data` - CID_Data instance to store
/// * `is_in_offchain_context` - Tells the storage, if the method call is within onchain or offchain context.
/// 							 That is important, since in both states use different storage APIs and storing fails, it the flag is wrong
///
pub fn store_cid_data_for_key<T: Config>(key: &Vec<u8>, cid_data: &Offchain_Data, is_in_offchain_context: bool) {
	store_offchain_data(key, cid_data, is_in_offchain_context)
}


/// Writes cdata to the offchain storage with the given key.
///
///
/// # Arguments
///
/// * `key` - unique key to store the data
/// * `data` - data to store, must implement trait Encode
/// * `is_in_offchain_context` - Tells the storage, if the method call is within onchain or offchain context.
/// 							 That is important, since in both states use different storage APIs and storing fails, it the flag is wrong
///
pub fn store_offchain_data(key: &Vec<u8>, data: &impl Encode, is_in_offchain_context: bool) {
	if is_in_offchain_context {
		let storage_ref = StorageValueRef::persistent(key);
		storage_ref.set(data);
	}
	else {
		offchain_index::set(key, &data.encode());
	}
}

/// Returns stored offchain data for the given block number
///
pub fn read_cid_for_block_number<T: Config>(block_number: T::BlockNumber) -> Result<Offchain_Data, StorageRetrievalError>  {
	match read_cid_data_for_block_number::<T>(block_number) {
		Ok(data) => {
			let cid_data = data.expect("expected cid data");
			Ok(cid_data)
		},
		Err(error) => {
			Err(error)
		}
	}
}

/// Returns cid data for the given block number
///
/// The OffchainStorageData is a wrapper class around the stored data. You can access the actual storage data through OffchainStorageData.data
///
pub fn read_cid_data_for_block_number<T: Config>(block_number: T::BlockNumber) -> Result<Option<Offchain_Data>, StorageRetrievalError> {
	let key = offchain_data_key::<T>(block_number);
	let ret_val = offchain_storage_data_for_key(&key);

	ret_val
}

/// Returns cid data for the given key
///
/// The OffchainStorageData is a wrapper class around the stored data. You can access the actual storage data through OffchainStorageData.data
///
pub fn read_cid_data_for_key(key: &Vec<u8>) -> Result<Option<Offchain_Data>, StorageRetrievalError> {
	let ret_val = offchain_storage_data_for_key::<Offchain_Data>(key);
	ret_val
}


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

/// Returns the OffchainStorageData instance for the given key
///
/// The OffchainStorageData is a wrapper class around the stored data. You can access the actual storage data through OffchainStorageData.data
///
pub fn offchain_storage_data_for_key<T: Decode>(key: &Vec<u8>) -> Result<Option<T>, StorageRetrievalError> {
	let storage_ref = StorageValueRef::persistent(key);
	let stored_data_result = storage_ref.get::<T>();

	stored_data_result
}
