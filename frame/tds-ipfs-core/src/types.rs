use codec::{Decode, Encode};
use sp_std::vec::Vec;

#[cfg(feature = "std")]
use frame_support::serde::{Deserialize};
use frame_support::pallet_prelude::TypeInfo;

/// Wrapper class containing the cid and meta data
#[derive(Debug, Clone, Encode, Decode, PartialEq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Deserialize))]
pub struct CIDData {
	pub data: Vec<u8>,
	pub meta_data: Vec<u8>
	// can add further data such as enums here
}

impl CIDData {
	/// creates a new instance with the given data
	///
	pub fn new(data: Vec<u8>, meta_data: Vec<u8>) -> CIDData {
		CIDData {data, meta_data }
	}
}
