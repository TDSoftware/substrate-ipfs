use crate::{self as pallet_tds_ipfs_core, multiple_bytes_to_utf8_safe_bytes, generate_id, addresses_to_utf8_safe_bytes};
use frame_support::{parameter_types};

use sp_core::H256;
use sp_runtime::{
  testing::Header,
  traits::{BlakeTwo256, IdentityLookup}, offchain::{OpaqueMultiaddr},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
  pub enum Test where
    Block = Block,
    NodeBlock = Block,
    UncheckedExtrinsic = UncheckedExtrinsic,
  {
    System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
    TDSIpfsCore: pallet_tds_ipfs_core::{Pallet, Call, Storage, Event<T>},
	RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Pallet, Storage},
  }
);

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
  type MaxConsumers = frame_support::traits::ConstU32<16>;
  type RuntimeCall = RuntimeCall;
  type Index = u64;
  type BlockNumber = u64;
  type Hash = H256;
  type Hashing = BlakeTwo256;
  type AccountId = u64;
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
}

impl pallet_randomness_collective_flip::Config for Test {}

impl pallet_tds_ipfs_core::Config for Test {
  type RuntimeEvent = RuntimeEvent;
  type IpfsRandomness = RandomnessCollectiveFlip;
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
}

pub fn mock_generate_id() ->  [u8; 32] {
	let pair = generate_id::<Test>();
	pair
}

pub fn mock_addresses_to_utf8_safe_bytes(address: &str) -> Vec<u8> {
	let bytes = address.as_bytes().to_vec();
	let mut vec = Vec::<OpaqueMultiaddr>::new();
	let first = OpaqueMultiaddr::new(bytes);

	vec.push(first);
	let result = addresses_to_utf8_safe_bytes(vec);

	result
}

pub fn mock_multiple_bytes_to_utf8_safe_bytes(response: Vec<Vec<u8>>) -> Vec<u8> {
	let result = multiple_bytes_to_utf8_safe_bytes(response);
	result
}
