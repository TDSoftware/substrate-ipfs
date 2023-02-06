// // TODO: TDS comment in and try to fix compiler errors


// #![cfg(test)]

// use crate::{self as pallet_tds_ipfs, /* decl_tests, */ Config, Pallet};
// use crate::*;
// use codec::Decode;
// use frame_support::{
// 	assert_ok, parameter_types,
// 	traits::{ConstU32, ConstU64},
// };
// use sp_core::{
// 	offchain::{testing, OffchainWorkerExt, TransactionPoolExt},
// 	sr25519::Signature,
// 	H256,
// };
// use std::sync::Arc;

// // use sp_keystore::{testing::KeyStore, KeystoreExt, SyncCryptoStore};
// use sp_runtime::{
// 	testing::{Header, TestXt},
// 	traits::{BlakeTwo256, Extrinsic as ExtrinsicT, IdentifyAccount, IdentityLookup, Verify},
// 	RuntimeAppPublic,
// };

// type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
// type Block = frame_system::mocking::MockBlock<Test>;
// type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

// // Configure a mock runtime to test the pallet.
// frame_support::construct_runtime!(
//   pub enum Test where
//   	Block = Block,
//   	NodeBlock = Block,
//   	UncheckedExtrinsic = UncheckedExtrinsic,
//   {
//     System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
//     TDSIpfs: pallet_tds_ipfs::{Pallet, Call, Storage, Event<T>},
// 	//TDSIpfsCore: pallet_tds_ipfs::{Pallet, Call, Storage, Event<T>},
// 	RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Pallet, Storage},
//   }
// );


// parameter_types! {
// 	pub BlockWeights: frame_system::limits::BlockWeights =
// 		frame_system::limits::BlockWeights::simple_max(
// 			frame_support::weights::Weight::from_ref_time(1024).set_proof_size(u64::MAX),
// 		);
// 	pub static ExistentialDeposit: u64 = 0;
// }

// impl frame_system::Config for Test {
// 	type BaseCallFilter = frame_support::traits::Everything;
// 	type BlockWeights = ();
// 	type BlockLength = ();
// 	type DbWeight = ();
// 	type RuntimeOrigin = RuntimeOrigin;
// 	type RuntimeCall = RuntimeCall;
// 	type Index = u64;
// 	type BlockNumber = u64;
// 	type Hash = H256;
// 	type Hashing = BlakeTwo256;
// 	type AccountId = AccountId;
// 	type Lookup = IdentityLookup<Self::AccountId>;
// 	type Header = Header;
// 	type RuntimeEvent = RuntimeEvent;
// 	type BlockHashCount = ConstU64<250>;
// 	type Version = ();
// 	type PalletInfo = PalletInfo;
// 	type AccountData = ();
// 	type OnNewAccount = ();
// 	type OnKilledAccount = ();
// 	type SystemWeightInfo = ();
// 	type SS58Prefix = ();
// 	type OnSetCode = ();
// 	type MaxConsumers = ConstU32<16>;
// }

// impl pallet_randomness_collective_flip::Config for Test {}
// type Extrinsic = TestXt<RuntimeCall, ()>;


// impl frame_system::offchain::SigningTypes for Test {
// 	type Public = <Signature as Verify>::Signer;
// 	type Signature = Signature;
// }

// impl<LocalCall> frame_system::offchain::SendTransactionTypes<LocalCall> for Test
// where
// 	RuntimeCall: From<LocalCall>,
// {
// 	type OverarchingCall = RuntimeCall;
// 	type Extrinsic = Extrinsic;
// }

// impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for Test
// where
// 	RuntimeCall: From<LocalCall>,
// {
// 	fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
// 		call: RuntimeCall,
// 		_public: <Signature as Verify>::Signer,
// 		_account: AccountId,
// 		nonce: u64,
// 	) -> Option<(RuntimeCall, <Extrinsic as ExtrinsicT>::SignaturePayload)> {
// 		Some((call, (nonce, ())))
// 	}
// }

// impl Config for Test {
// 	type AuthorityId = pallet_tds_ipfs::crypto::TestAuthId;
// 	type RuntimeCall = RuntimeCall;
// 	type RuntimeEvent = RuntimeEvent;
// 	type IpfsRandomness = RandomnessCollectiveFlip;

// }

// // impl Config for Test {
// // 	type AuthorityId = pallet_tds_ipfs::crypto::TestAuthId;
// // 	type RuntimeCall = RuntimeCall;
// // 	type RuntimeEvent = RuntimeEvent;
// // 	type IpfsRandomness = RandomnessCollectiveFlip;
// // }

// // use crate as pallet_tds_ipfs;
// // use codec::{Decode, Encode};
// // use frame_support::traits::{ConstU16, ConstU64};
// // use frame_system as system;
// // use frame_support::parameter_types;
// // use scale_info::TypeInfo;
// // use sp_core::{H256, sr25519::Signature};
// // use sp_runtime::{
// // 	testing::Header,
// // 	traits::{BlakeTwo256, IdentityLookup, Verify, IdentifyAccount},
// // };

// // type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
// // type Block = frame_system::mocking::MockBlock<Test>;

// // // Configure a mock runtime to test the pallet.
// // frame_support::construct_runtime!(
// //   pub enum Test where
// //   	Block = Block,
// //   	NodeBlock = Block,
// //   	UncheckedExtrinsic = UncheckedExtrinsic,
// //   {
// //     System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
// //     TDSIpfs: pallet_tds_ipfs::{Pallet, Call, Storage, Event<T>},
// // 	RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Pallet, Storage},
// //   }
// // );

// // parameter_types! {
// //   pub const BlockHashCount: u64 = 250;
// //   pub const SS58Prefix: u8 = 42;
// // }

// // impl frame_system::Config for Test {
// // 	type BaseCallFilter = frame_support::traits::Everything;
// // 	type BlockWeights = ();
// // 	type BlockLength = ();
// // 	type DbWeight = ();
// // 	type RuntimeOrigin = RuntimeOrigin;
// // 	type RuntimeCall = RuntimeCall;
// // 	type Index = u64;
// // 	type BlockNumber = u64;
// // 	type Hash = H256;
// // 	type Hashing = BlakeTwo256;
// // 	type AccountId = u64;
// // 	type Lookup = IdentityLookup<Self::AccountId>;
// // 	type Header = Header;
// // 	type RuntimeEvent = RuntimeEvent;
// // 	type BlockHashCount = ConstU64<250>;
// // 	type Version = ();
// // 	type PalletInfo = PalletInfo;
// // 	type AccountData = ();
// // 	type OnNewAccount = ();
// // 	type OnKilledAccount = ();
// // 	type SystemWeightInfo = ();
// // 	type SS58Prefix = ConstU16<42>;
// // 	type OnSetCode = ();
// // 	type MaxConsumers = frame_support::traits::ConstU32<16>;
// // }

// // /// Test transaction, tuple of (sender, call, signed_extra)
// // /// with index only used if sender is some.
// // ///
// // /// If sender is some then the transaction is signed otherwise it is unsigned.
// // #[derive(PartialEq, Eq, Clone, Encode, Decode, TypeInfo)]
// // pub struct TestXt<Call, Extra> {
// // 	/// Signature of the extrinsic.
// // 	pub signature: Option<(u64, Extra)>,
// // 	/// Call of the extrinsic.
// // 	pub call: Call,
// // }

// // impl pallet_randomness_collective_flip::Config for Test {}

// // type Extrinsic = TestXt<RuntimeCall, ()>;


// // impl frame_system::offchain::SigningTypes for Test {
// // 	type Public = <Signature as Verify>::Signer;
// // 	type Signature = Signature;
// // }

// // // impl<LocalCall> frame_system::offchain::SendTransactionTypes<LocalCall> for Test
// // // where
// // // 	RuntimeCall: From<LocalCall>,
// // // {
// // // 	type OverarchingCall = RuntimeCall;
// // // 	type Extrinsic = Extrinsic;
// // // }

// // /*
// // impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for Test
// // where
// // 	RuntimeCall: From<LocalCall>,
// // {
// // 	fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
// // 		call: RuntimeCall,
// // 		_public: <Signature as Verify>::Signer,
// // 		_account: AccountId,
// // 		nonce: u64,
// // 	) -> Option<(RuntimeCall, <Extrinsic as ExtrinsicT>::SignaturePayload)> {
// // 		Some((call, (nonce, ())))
// // 	}
// // }*/

// // parameter_types! {
// // 	pub const UnsignedPriority: u64 = 1 << 20;
// // }

// // // Build genesis storage according to the mock runtime.
// pub fn new_test_ext() -> sp_io::TestExternalities {
//   frame_system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
// }
