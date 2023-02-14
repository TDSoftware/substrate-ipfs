use frame_support::*;
use codec::{Decode, Encode};
use frame_system::Config;
use sp_io::offchain_index;
use sp_std::{str, vec::Vec};

#[cfg(feature = "std")]
use frame_support::serde::{Deserialize};
use sp_runtime::offchain::storage::{StorageRetrievalError, StorageValueRef};

const OFFCHAIN_KEY_PREFIX: &[u8] = b"ipfs_core::indexing1";

#[derive(Debug, Encode, Decode, Default)]
#[cfg_attr(feature = "std", derive(Deserialize))]
pub struct OffchainStorageData {
  pub data: Vec<u8>,
  // TODO: add action/usage like AddBytes ...
}

/**
 * Writes offchain data if in onchain context
*/
pub fn set_offchain_data<T: Config>(block_number: T::BlockNumber, data: Vec<u8>) {
	let key = offchain_data_key::<T>(block_number);
	offchain_index::set(&key, &data.encode());
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
pub fn offchain_data_for_key<T: codec::Decode>(key: Vec<u8>) -> Result<Option<T>, StorageRetrievalError> {
	let storage_ref = StorageValueRef::persistent(&key);
	storage_ref.get::<T>()
}

/**
 * Returns offchain indexed data for the given block number
*/
pub fn offchain_data_for_block_number<T: codec::Decode, C: Config>(block_number: C::BlockNumber) -> Result<Option<T>, StorageRetrievalError> {
	let key = offchain_data_key::<C>(block_number);
	let storage_ref = StorageValueRef::persistent(&key);
	storage_ref.get::<T>()
}
