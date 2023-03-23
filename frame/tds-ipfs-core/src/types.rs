use codec::{Decode, Encode};
use sp_std::vec::Vec;

#[cfg(feature = "std")]
use frame_support::serde::{Deserialize};
use frame_support::pallet_prelude::TypeInfo;

/// Wrapper class containing the file data to upload to ipfs and referring meta data
#[derive(Debug, Clone, Encode, Decode, PartialEq, TypeInfo, Default)]
#[cfg_attr(feature = "std", derive(Deserialize))]
pub struct OffchainData {
	pub data: Vec<u8>,
	pub meta_data: Vec<u8>
	// can add further data such as enums here
}

impl OffchainData {
	/// creates a new instance with the given data
	///
	pub fn new(data: Vec<u8>, meta_data: Vec<u8>) -> OffchainData {
		OffchainData { data: data, meta_data }
	}
}

/// Wrapper class containing infos of a file within IPFS which had been uploaded by the substrate node
#[derive(Debug, Clone, Encode, Decode, PartialEq, TypeInfo, Default)]
#[cfg_attr(feature = "std", derive(Deserialize))]
pub struct IpfsFile {
	pub cid: Vec<u8>,
	pub meta_data: Vec<u8>
}

impl IpfsFile {
	/// creates a new instance with the given data
	///
	pub fn new(cid: Vec<u8>, meta_data: Vec<u8>) -> IpfsFile {
		IpfsFile { cid, meta_data }
	}
}
