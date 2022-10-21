// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_elections_phragmen
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-21, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm2`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/substrate
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_elections_phragmen
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/elections-phragmen/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_elections_phragmen.
pub trait WeightInfo {
	fn vote_equal(v: u32, ) -> Weight;
	fn vote_more(v: u32, ) -> Weight;
	fn vote_less(v: u32, ) -> Weight;
	fn remove_voter() -> Weight;
	fn submit_candidacy(c: u32, ) -> Weight;
	fn renounce_candidacy_candidate(c: u32, ) -> Weight;
	fn renounce_candidacy_members() -> Weight;
	fn renounce_candidacy_runners_up() -> Weight;
	fn remove_member_without_replacement() -> Weight;
	fn remove_member_with_replacement() -> Weight;
	fn clean_defunct_voters(v: u32, d: u32, ) -> Weight;
	fn election_phragmen(c: u32, v: u32, e: u32, ) -> Weight;
}

/// Weights for pallet_elections_phragmen using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Elections Candidates (r:1 w:0)
	// Storage: Elections Members (r:1 w:0)
	// Storage: Elections RunnersUp (r:1 w:0)
	// Storage: Elections Voting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `v` is `[1, 16]`.
	fn vote_equal(v: u32, ) -> Weight {
		// Minimum execution time: 36_810 nanoseconds.
		Weight::from_ref_time(38_073_194 as u64)
			// Standard Error: 3_928
			.saturating_add(Weight::from_ref_time(117_832 as u64).saturating_mul(v as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Elections Candidates (r:1 w:0)
	// Storage: Elections Members (r:1 w:0)
	// Storage: Elections RunnersUp (r:1 w:0)
	// Storage: Elections Voting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `v` is `[2, 16]`.
	fn vote_more(v: u32, ) -> Weight {
		// Minimum execution time: 47_770 nanoseconds.
		Weight::from_ref_time(48_707_259 as u64)
			// Standard Error: 3_169
			.saturating_add(Weight::from_ref_time(135_279 as u64).saturating_mul(v as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Elections Candidates (r:1 w:0)
	// Storage: Elections Members (r:1 w:0)
	// Storage: Elections RunnersUp (r:1 w:0)
	// Storage: Elections Voting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `v` is `[2, 16]`.
	fn vote_less(v: u32, ) -> Weight {
		// Minimum execution time: 46_787 nanoseconds.
		Weight::from_ref_time(47_683_010 as u64)
			// Standard Error: 2_891
			.saturating_add(Weight::from_ref_time(228_508 as u64).saturating_mul(v as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Elections Voting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn remove_voter() -> Weight {
		// Minimum execution time: 46_147 nanoseconds.
		Weight::from_ref_time(46_806_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Elections Candidates (r:1 w:1)
	// Storage: Elections Members (r:1 w:0)
	// Storage: Elections RunnersUp (r:1 w:0)
	/// The range of component `c` is `[1, 1000]`.
	fn submit_candidacy(c: u32, ) -> Weight {
		// Minimum execution time: 42_504 nanoseconds.
		Weight::from_ref_time(47_674_318 as u64)
			// Standard Error: 922
			.saturating_add(Weight::from_ref_time(77_149 as u64).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Elections Candidates (r:1 w:1)
	/// The range of component `c` is `[1, 1000]`.
	fn renounce_candidacy_candidate(c: u32, ) -> Weight {
		// Minimum execution time: 39_996 nanoseconds.
		Weight::from_ref_time(51_619_293 as u64)
			// Standard Error: 1_004
			.saturating_add(Weight::from_ref_time(52_454 as u64).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Elections Members (r:1 w:1)
	// Storage: Elections RunnersUp (r:1 w:1)
	// Storage: Council Prime (r:1 w:1)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: Council Members (r:0 w:1)
	fn renounce_candidacy_members() -> Weight {
		// Minimum execution time: 52_278 nanoseconds.
		Weight::from_ref_time(52_578_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Elections RunnersUp (r:1 w:1)
	fn renounce_candidacy_runners_up() -> Weight {
		// Minimum execution time: 40_482 nanoseconds.
		Weight::from_ref_time(41_071_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Benchmark Override (r:0 w:0)
	fn remove_member_without_replacement() -> Weight {
		// Minimum execution time: 2_000_000_000 nanoseconds.
		Weight::from_ref_time(2_000_000_000_000 as u64)
	}
	// Storage: Elections Members (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Elections RunnersUp (r:1 w:1)
	// Storage: Council Prime (r:1 w:1)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: Council Members (r:0 w:1)
	fn remove_member_with_replacement() -> Weight {
		// Minimum execution time: 60_122 nanoseconds.
		Weight::from_ref_time(61_004_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: Elections Voting (r:5001 w:5000)
	// Storage: Elections Members (r:1 w:0)
	// Storage: Elections RunnersUp (r:1 w:0)
	// Storage: Elections Candidates (r:1 w:0)
	// Storage: Balances Locks (r:5000 w:5000)
	// Storage: System Account (r:5000 w:5000)
	/// The range of component `v` is `[5000, 10000]`.
	/// The range of component `d` is `[0, 5000]`.
	fn clean_defunct_voters(v: u32, _d: u32, ) -> Weight {
		// Minimum execution time: 289_482_802 nanoseconds.
		Weight::from_ref_time(289_950_275_000 as u64)
			// Standard Error: 259_040
			.saturating_add(Weight::from_ref_time(37_302_354 as u64).saturating_mul(v as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().reads((3 as u64).saturating_mul(v as u64)))
			.saturating_add(T::DbWeight::get().writes((3 as u64).saturating_mul(v as u64)))
	}
	// Storage: Elections Candidates (r:1 w:1)
	// Storage: Elections Members (r:1 w:1)
	// Storage: Elections RunnersUp (r:1 w:1)
	// Storage: Elections Voting (r:10001 w:0)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: Elections ElectionRounds (r:1 w:1)
	// Storage: Council Members (r:0 w:1)
	// Storage: Council Prime (r:0 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `c` is `[1, 1000]`.
	/// The range of component `v` is `[1, 10000]`.
	/// The range of component `e` is `[10000, 160000]`.
	fn election_phragmen(c: u32, v: u32, e: u32, ) -> Weight {
		// Minimum execution time: 21_545_085 nanoseconds.
		Weight::from_ref_time(21_640_807_000 as u64)
			// Standard Error: 233_654
			.saturating_add(Weight::from_ref_time(25_222_479 as u64).saturating_mul(v as u64))
			// Standard Error: 14_994
			.saturating_add(Weight::from_ref_time(1_034_126 as u64).saturating_mul(e as u64))
			.saturating_add(T::DbWeight::get().reads(280 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(c as u64)))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(v as u64)))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(c as u64)))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Elections Candidates (r:1 w:0)
	// Storage: Elections Members (r:1 w:0)
	// Storage: Elections RunnersUp (r:1 w:0)
	// Storage: Elections Voting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `v` is `[1, 16]`.
	fn vote_equal(v: u32, ) -> Weight {
		// Minimum execution time: 36_810 nanoseconds.
		Weight::from_ref_time(38_073_194 as u64)
			// Standard Error: 3_928
			.saturating_add(Weight::from_ref_time(117_832 as u64).saturating_mul(v as u64))
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: Elections Candidates (r:1 w:0)
	// Storage: Elections Members (r:1 w:0)
	// Storage: Elections RunnersUp (r:1 w:0)
	// Storage: Elections Voting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `v` is `[2, 16]`.
	fn vote_more(v: u32, ) -> Weight {
		// Minimum execution time: 47_770 nanoseconds.
		Weight::from_ref_time(48_707_259 as u64)
			// Standard Error: 3_169
			.saturating_add(Weight::from_ref_time(135_279 as u64).saturating_mul(v as u64))
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: Elections Candidates (r:1 w:0)
	// Storage: Elections Members (r:1 w:0)
	// Storage: Elections RunnersUp (r:1 w:0)
	// Storage: Elections Voting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `v` is `[2, 16]`.
	fn vote_less(v: u32, ) -> Weight {
		// Minimum execution time: 46_787 nanoseconds.
		Weight::from_ref_time(47_683_010 as u64)
			// Standard Error: 2_891
			.saturating_add(Weight::from_ref_time(228_508 as u64).saturating_mul(v as u64))
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: Elections Voting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn remove_voter() -> Weight {
		// Minimum execution time: 46_147 nanoseconds.
		Weight::from_ref_time(46_806_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: Elections Candidates (r:1 w:1)
	// Storage: Elections Members (r:1 w:0)
	// Storage: Elections RunnersUp (r:1 w:0)
	/// The range of component `c` is `[1, 1000]`.
	fn submit_candidacy(c: u32, ) -> Weight {
		// Minimum execution time: 42_504 nanoseconds.
		Weight::from_ref_time(47_674_318 as u64)
			// Standard Error: 922
			.saturating_add(Weight::from_ref_time(77_149 as u64).saturating_mul(c as u64))
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Elections Candidates (r:1 w:1)
	/// The range of component `c` is `[1, 1000]`.
	fn renounce_candidacy_candidate(c: u32, ) -> Weight {
		// Minimum execution time: 39_996 nanoseconds.
		Weight::from_ref_time(51_619_293 as u64)
			// Standard Error: 1_004
			.saturating_add(Weight::from_ref_time(52_454 as u64).saturating_mul(c as u64))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Elections Members (r:1 w:1)
	// Storage: Elections RunnersUp (r:1 w:1)
	// Storage: Council Prime (r:1 w:1)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: Council Members (r:0 w:1)
	fn renounce_candidacy_members() -> Weight {
		// Minimum execution time: 52_278 nanoseconds.
		Weight::from_ref_time(52_578_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	// Storage: Elections RunnersUp (r:1 w:1)
	fn renounce_candidacy_runners_up() -> Weight {
		// Minimum execution time: 40_482 nanoseconds.
		Weight::from_ref_time(41_071_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Benchmark Override (r:0 w:0)
	fn remove_member_without_replacement() -> Weight {
		// Minimum execution time: 2_000_000_000 nanoseconds.
		Weight::from_ref_time(2_000_000_000_000 as u64)
	}
	// Storage: Elections Members (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Elections RunnersUp (r:1 w:1)
	// Storage: Council Prime (r:1 w:1)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: Council Members (r:0 w:1)
	fn remove_member_with_replacement() -> Weight {
		// Minimum execution time: 60_122 nanoseconds.
		Weight::from_ref_time(61_004_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().writes(5 as u64))
	}
	// Storage: Elections Voting (r:5001 w:5000)
	// Storage: Elections Members (r:1 w:0)
	// Storage: Elections RunnersUp (r:1 w:0)
	// Storage: Elections Candidates (r:1 w:0)
	// Storage: Balances Locks (r:5000 w:5000)
	// Storage: System Account (r:5000 w:5000)
	/// The range of component `v` is `[5000, 10000]`.
	/// The range of component `d` is `[0, 5000]`.
	fn clean_defunct_voters(v: u32, _d: u32, ) -> Weight {
		// Minimum execution time: 289_482_802 nanoseconds.
		Weight::from_ref_time(289_950_275_000 as u64)
			// Standard Error: 259_040
			.saturating_add(Weight::from_ref_time(37_302_354 as u64).saturating_mul(v as u64))
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().reads((3 as u64).saturating_mul(v as u64)))
			.saturating_add(RocksDbWeight::get().writes((3 as u64).saturating_mul(v as u64)))
	}
	// Storage: Elections Candidates (r:1 w:1)
	// Storage: Elections Members (r:1 w:1)
	// Storage: Elections RunnersUp (r:1 w:1)
	// Storage: Elections Voting (r:10001 w:0)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: Elections ElectionRounds (r:1 w:1)
	// Storage: Council Members (r:0 w:1)
	// Storage: Council Prime (r:0 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `c` is `[1, 1000]`.
	/// The range of component `v` is `[1, 10000]`.
	/// The range of component `e` is `[10000, 160000]`.
	fn election_phragmen(c: u32, v: u32, e: u32, ) -> Weight {
		// Minimum execution time: 21_545_085 nanoseconds.
		Weight::from_ref_time(21_640_807_000 as u64)
			// Standard Error: 233_654
			.saturating_add(Weight::from_ref_time(25_222_479 as u64).saturating_mul(v as u64))
			// Standard Error: 14_994
			.saturating_add(Weight::from_ref_time(1_034_126 as u64).saturating_mul(e as u64))
			.saturating_add(RocksDbWeight::get().reads(280 as u64))
			.saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(c as u64)))
			.saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(v as u64)))
			.saturating_add(RocksDbWeight::get().writes(6 as u64))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(c as u64)))
	}
}
