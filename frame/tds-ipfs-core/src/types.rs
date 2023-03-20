use codec::{Decode, Encode};
use frame_system::Config;
use sp_io::{offchain_index};

use sp_runtime::offchain::storage::{StorageValueRef, StorageRetrievalError};
use sp_std::vec::Vec;

#[cfg(feature = "std")]
use frame_support::serde::{Deserialize};
use frame_support::pallet_prelude::TypeInfo;

/// Wrapper class containing the cid and meta data
#[derive(Debug, Clone, Encode, Decode, PartialEq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Deserialize))]
pub struct CID_Data {
	pub data: Vec<u8>,
	pub meta_data: Vec<u8>
	// can add further data such as enums here
}

impl CID_Data {
	/// creates a new instance with the given data
	///
	pub fn new(data: Vec<u8>, meta_data: Vec<u8>) -> CID_Data {
		CID_Data {data, meta_data }
	}
}
