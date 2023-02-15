use codec::{Decode, Encode};
use frame_system::Config;
use sp_io::{offchain_index};

use sp_runtime::offchain::storage::{StorageValueRef, StorageRetrievalError};
use sp_std::vec::Vec;

#[cfg(feature = "std")]
use frame_support::serde::{Deserialize};

const OFFCHAIN_KEY_PREFIX: &[u8] = b"ipfs_core::indexing1";

#[derive(Debug, Encode, Decode, Default)]
#[cfg_attr(feature = "std", derive(Deserialize))]

/**
 * Wrapper class containing the serialzed data
 */
pub struct OffchainStorageData {
	pub data: Vec<u8>,
	// can add further data such as enums here
}

impl OffchainStorageData {
	pub fn new(data: Vec<u8>) -> OffchainStorageData {
		OffchainStorageData{data: data}
	}
}

/**
 * Writes offchain data if in onchain context
*/
pub fn set_offchain_data<T: Config>(block_number: T::BlockNumber, data: Vec<u8>, is_in_offchain_context: bool) {
	let key = offchain_data_key::<T>(block_number);

	let storage_data = OffchainStorageData::new(data);

	if is_in_offchain_context {
		let key = offchain_data_key::<T>(block_number);
		let storage_ref = StorageValueRef::persistent(&key);

		storage_ref.set(&storage_data);
	}
	else {
		offchain_index::set(&key, &storage_data.encode());
	}
}

/**
 * Writes offchain data if in onchain context
*/
pub fn offchain_data<T: Config>(block_number: T::BlockNumber) -> Result<Vec<u8>, StorageRetrievalError>  {
	match offchain_storage_data_for_block_number::<T>(block_number) {
		Ok(data) => {
			let offchain_storage_data = data.expect("expected offchain storage data");
			Ok(offchain_storage_data.data)
		},
		Err(error) => {
			Err(error)
		}
	}
}

/**
 * Creates an offchain data key/identifier for the given block number
*/
pub fn offchain_data_key<T: Config>(block_number: T::BlockNumber) -> Vec<u8> {
	block_number.using_encoded(|encoded_bn| {
		OFFCHAIN_KEY_PREFIX.clone().into_iter()
			.chain(b"/".into_iter())
			.chain(encoded_bn)
			.copied()
			.collect::<Vec<u8>>()
	})
}

/**
 * Returns offchain indexed data for the given key
*/
pub fn offchain_storage_data_for_key(key: &Vec<u8>) -> Result<Option<OffchainStorageData>, StorageRetrievalError> {
	let storage_ref = StorageValueRef::persistent(key);
	let stored_data_result = storage_ref.get::<OffchainStorageData>();

	stored_data_result
}

/**
 * Returns offchain indexed data for the given block number
*/
pub fn offchain_storage_data_for_block_number<T: Config>(block_number: T::BlockNumber) -> Result<Option<OffchainStorageData>, StorageRetrievalError> {
	let key = offchain_data_key::<T>(block_number);
	let ret_val = offchain_storage_data_for_key(&key);

	ret_val
}
