// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_vesting
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-11-07, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm2`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/substrate
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_vesting
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/vesting/src/weights.rs
// --header=./HEADER-APACHE2
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_vesting.
pub trait WeightInfo {
	fn vest_locked(l: u32, s: u32, ) -> Weight;
	fn vest_unlocked(l: u32, s: u32, ) -> Weight;
	fn vest_other_locked(l: u32, s: u32, ) -> Weight;
	fn vest_other_unlocked(l: u32, s: u32, ) -> Weight;
	fn vested_transfer(l: u32, s: u32, ) -> Weight;
	fn force_vested_transfer(l: u32, s: u32, ) -> Weight;
	fn not_unlocking_merge_schedules(l: u32, s: u32, ) -> Weight;
	fn unlocking_merge_schedules(l: u32, s: u32, ) -> Weight;
}

/// Weights for pallet_vesting using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[1, 28]`.
	fn vest_locked(l: u32, s: u32, ) -> Weight {
		// Minimum execution time: 45_113 nanoseconds.
		Weight::from_ref_time(44_114_539 as u64)
			// Standard Error: 958
			.saturating_add(Weight::from_ref_time(56_239 as u64).saturating_mul(l as u64))
			// Standard Error: 1_704
			.saturating_add(Weight::from_ref_time(64_926 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[1, 28]`.
	fn vest_unlocked(l: u32, s: u32, ) -> Weight {
		// Minimum execution time: 43_918 nanoseconds.
		Weight::from_ref_time(43_452_573 as u64)
			// Standard Error: 984
			.saturating_add(Weight::from_ref_time(50_162 as u64).saturating_mul(l as u64))
			// Standard Error: 1_752
			.saturating_add(Weight::from_ref_time(42_080 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[1, 28]`.
	fn vest_other_locked(l: u32, s: u32, ) -> Weight {
		// Minimum execution time: 43_603 nanoseconds.
		Weight::from_ref_time(42_696_097 as u64)
			// Standard Error: 996
			.saturating_add(Weight::from_ref_time(65_316 as u64).saturating_mul(l as u64))
			// Standard Error: 1_772
			.saturating_add(Weight::from_ref_time(65_862 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[1, 28]`.
	fn vest_other_unlocked(l: u32, s: u32, ) -> Weight {
		// Minimum execution time: 43_099 nanoseconds.
		Weight::from_ref_time(42_937_914 as u64)
			// Standard Error: 884
			.saturating_add(Weight::from_ref_time(52_079 as u64).saturating_mul(l as u64))
			// Standard Error: 1_573
			.saturating_add(Weight::from_ref_time(36_274 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[0, 27]`.
	fn vested_transfer(l: u32, s: u32, ) -> Weight {
		// Minimum execution time: 59_023 nanoseconds.
		Weight::from_ref_time(59_606_862 as u64)
			// Standard Error: 2_078
			.saturating_add(Weight::from_ref_time(55_335 as u64).saturating_mul(l as u64))
			// Standard Error: 3_698
			.saturating_add(Weight::from_ref_time(26_743 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[0, 27]`.
	fn force_vested_transfer(l: u32, s: u32, ) -> Weight {
		// Minimum execution time: 58_249 nanoseconds.
		Weight::from_ref_time(59_025_976 as u64)
			// Standard Error: 2_078
			.saturating_add(Weight::from_ref_time(55_736 as u64).saturating_mul(l as u64))
			// Standard Error: 3_697
			.saturating_add(Weight::from_ref_time(24_903 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[2, 28]`.
	fn not_unlocking_merge_schedules(l: u32, s: u32, ) -> Weight {
		// Minimum execution time: 45_279 nanoseconds.
		Weight::from_ref_time(44_197_440 as u64)
			// Standard Error: 946
			.saturating_add(Weight::from_ref_time(62_308 as u64).saturating_mul(l as u64))
			// Standard Error: 1_747
			.saturating_add(Weight::from_ref_time(64_473 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[2, 28]`.
	fn unlocking_merge_schedules(l: u32, s: u32, ) -> Weight {
		// Minimum execution time: 44_925 nanoseconds.
		Weight::from_ref_time(44_219_676 as u64)
			// Standard Error: 889
			.saturating_add(Weight::from_ref_time(60_311 as u64).saturating_mul(l as u64))
			// Standard Error: 1_641
			.saturating_add(Weight::from_ref_time(63_095 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[1, 28]`.
	fn vest_locked(l: u32, s: u32, ) -> Weight {
		// Minimum execution time: 45_113 nanoseconds.
		Weight::from_ref_time(44_114_539 as u64)
			// Standard Error: 958
			.saturating_add(Weight::from_ref_time(56_239 as u64).saturating_mul(l as u64))
			// Standard Error: 1_704
			.saturating_add(Weight::from_ref_time(64_926 as u64).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[1, 28]`.
	fn vest_unlocked(l: u32, s: u32, ) -> Weight {
		// Minimum execution time: 43_918 nanoseconds.
		Weight::from_ref_time(43_452_573 as u64)
			// Standard Error: 984
			.saturating_add(Weight::from_ref_time(50_162 as u64).saturating_mul(l as u64))
			// Standard Error: 1_752
			.saturating_add(Weight::from_ref_time(42_080 as u64).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[1, 28]`.
	fn vest_other_locked(l: u32, s: u32, ) -> Weight {
		// Minimum execution time: 43_603 nanoseconds.
		Weight::from_ref_time(42_696_097 as u64)
			// Standard Error: 996
			.saturating_add(Weight::from_ref_time(65_316 as u64).saturating_mul(l as u64))
			// Standard Error: 1_772
			.saturating_add(Weight::from_ref_time(65_862 as u64).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[1, 28]`.
	fn vest_other_unlocked(l: u32, s: u32, ) -> Weight {
		// Minimum execution time: 43_099 nanoseconds.
		Weight::from_ref_time(42_937_914 as u64)
			// Standard Error: 884
			.saturating_add(Weight::from_ref_time(52_079 as u64).saturating_mul(l as u64))
			// Standard Error: 1_573
			.saturating_add(Weight::from_ref_time(36_274 as u64).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[0, 27]`.
	fn vested_transfer(l: u32, s: u32, ) -> Weight {
		// Minimum execution time: 59_023 nanoseconds.
		Weight::from_ref_time(59_606_862 as u64)
			// Standard Error: 2_078
			.saturating_add(Weight::from_ref_time(55_335 as u64).saturating_mul(l as u64))
			// Standard Error: 3_698
			.saturating_add(Weight::from_ref_time(26_743 as u64).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[0, 27]`.
	fn force_vested_transfer(l: u32, s: u32, ) -> Weight {
		// Minimum execution time: 58_249 nanoseconds.
		Weight::from_ref_time(59_025_976 as u64)
			// Standard Error: 2_078
			.saturating_add(Weight::from_ref_time(55_736 as u64).saturating_mul(l as u64))
			// Standard Error: 3_697
			.saturating_add(Weight::from_ref_time(24_903 as u64).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[2, 28]`.
	fn not_unlocking_merge_schedules(l: u32, s: u32, ) -> Weight {
		// Minimum execution time: 45_279 nanoseconds.
		Weight::from_ref_time(44_197_440 as u64)
			// Standard Error: 946
			.saturating_add(Weight::from_ref_time(62_308 as u64).saturating_mul(l as u64))
			// Standard Error: 1_747
			.saturating_add(Weight::from_ref_time(64_473 as u64).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[2, 28]`.
	fn unlocking_merge_schedules(l: u32, s: u32, ) -> Weight {
		// Minimum execution time: 44_925 nanoseconds.
		Weight::from_ref_time(44_219_676 as u64)
			// Standard Error: 889
			.saturating_add(Weight::from_ref_time(60_311 as u64).saturating_mul(l as u64))
			// Standard Error: 1_641
			.saturating_add(Weight::from_ref_time(63_095 as u64).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
}
