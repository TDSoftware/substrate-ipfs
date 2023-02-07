// // TODO: TDS comment in and try to fix compiler errors


#![cfg(test)]

use crate::{self as pallet_tds_ipfs};
use crate::*;

use std::sync::Arc;

use frame_support::{
	parameter_types,
};
use sp_core::{
	offchain::{testing, OffchainWorkerExt, TransactionPoolExt},
	sr25519::Signature,
	H256,
};

use sp_runtime::testing::{TestXt, Header};
use sp_runtime::traits::{Verify, BlakeTwo256, IdentityLookup, IdentifyAccount};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

use sp_keystore::{testing::KeyStore, KeystoreExt, SyncCryptoStore};

 // Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
  pub enum Test where
  	Block = Block,
  	NodeBlock = Block,
  	UncheckedExtrinsic = UncheckedExtrinsic,
  {
    System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
    IpfsCore: pallet_tds_ipfs_core::{Pallet, Call, Config<T>, Storage, Event<T>},
	Ipfs: pallet_tds_ipfs::{Pallet, Call, Storage, Config<T>, Event<T>},
	RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Pallet, Storage},
  }
);

parameter_types! {
	pub BlockWeights: frame_system::limits::BlockWeights =
		frame_system::limits::BlockWeights::simple_max(
			frame_support::weights::Weight::from_ref_time(1024).set_proof_size(u64::MAX),
		);

	pub static ExistentialDeposit: u64 = 0;
}

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
  }

  impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = sp_core::sr25519::Public;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
  }

impl pallet_randomness_collective_flip::Config for Test {}
type Extrinsic = TestXt<RuntimeCall, ()>;
type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

impl frame_system::offchain::SigningTypes for Test {
	type Public = <Signature as Verify>::Signer;
	type Signature = Signature;
}

impl<LocalCall> frame_system::offchain::SendTransactionTypes<LocalCall> for Test
where
	RuntimeCall: From<LocalCall>,
{
	type OverarchingCall = RuntimeCall;
	type Extrinsic = Extrinsic;
}

impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for Test
where
	RuntimeCall: From<LocalCall>,
{
	fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
		call: RuntimeCall,
		_public: <Signature as Verify>::Signer,
		_account: AccountId,
		nonce: u64,
	) -> Option<(RuntimeCall, <Extrinsic as sp_runtime::traits::Extrinsic>::SignaturePayload)> {
		Some((call, (nonce, ())))
	}
}

impl pallet_tds_ipfs_core::Config for Test  {
    type RuntimeEvent = RuntimeEvent;
    type IpfsRandomness = RandomnessCollectiveFlip;
}

impl Config for Test {
	type AuthorityId = pallet_tds_ipfs::crypto::TestAuthId;
	type RuntimeCall = RuntimeCall;
	type IpfsRandomness = RandomnessCollectiveFlip;

	type RuntimeEvent = RuntimeEvent;
}

#[derive(Default)]
pub struct ExtBuilder {}

impl ExtBuilder {
	pub fn build(self) -> sp_io::TestExternalities {
		let storage = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
		let mut ext = sp_io::TestExternalities::new(storage);

		ext.execute_with(|| System::set_block_number(1));
		ext
	}

	pub fn build_and_execute(self, test: impl FnOnce() -> ()) {
		self.build().execute_with(|| {
			test();
		})
	}

	// TODO: use this as executable
	pub fn test() {
		// const PHRASE: &str =
		// "news slush supreme milk chapter athlete soap sausage put clutch what kitten";

		// let (offchain, offchain_state) = testing::TestOffchainExt::new();
		// let (pool, pool_state) = testing::TestTransactionPoolExt::new();

		// let keystore = KeyStore::new();
		// SyncCryptoStore::sr25519_generate_new(
		// 	&keystore,
		// 	crate::crypto::Public::ID,
		// 	Some(&format!("{}/hunter1", PHRASE)),
		// )
		// .unwrap();

		// let mut t = sp_io::TestExternalities::default();
		// t.register_extension(OffchainWorkerExt::new(offchain));
		// t.register_extension(TransactionPoolExt::new(pool));
		// t.register_extension(KeystoreExt(Arc::new(keystore)))
	}
}
