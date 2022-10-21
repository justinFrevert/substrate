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

//! Autogenerated weights for pallet_utility
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
// --pallet=pallet_utility
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/utility/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_utility.
pub trait WeightInfo {
	fn batch(c: u32, ) -> Weight;
	fn as_derivative() -> Weight;
	fn batch_all(c: u32, ) -> Weight;
	fn dispatch_as() -> Weight;
	fn force_batch(c: u32, ) -> Weight;
}

/// Weights for pallet_utility using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// The range of component `c` is `[0, 1000]`.
	fn batch(c: u32, ) -> Weight {
		// Minimum execution time: 13_214 nanoseconds.
		Weight::from_ref_time(23_135_099 as u64)
			// Standard Error: 1_709
			.saturating_add(Weight::from_ref_time(3_522_538 as u64).saturating_mul(c as u64))
	}
	fn as_derivative() -> Weight {
		// Minimum execution time: 6_262 nanoseconds.
		Weight::from_ref_time(6_439_000 as u64)
	}
	/// The range of component `c` is `[0, 1000]`.
	fn batch_all(c: u32, ) -> Weight {
		// Minimum execution time: 13_490 nanoseconds.
		Weight::from_ref_time(22_483_716 as u64)
			// Standard Error: 1_778
			.saturating_add(Weight::from_ref_time(3_596_168 as u64).saturating_mul(c as u64))
	}
	fn dispatch_as() -> Weight {
		// Minimum execution time: 15_588 nanoseconds.
		Weight::from_ref_time(15_954_000 as u64)
	}
	/// The range of component `c` is `[0, 1000]`.
	fn force_batch(c: u32, ) -> Weight {
		// Minimum execution time: 13_657 nanoseconds.
		Weight::from_ref_time(18_190_291 as u64)
			// Standard Error: 1_576
			.saturating_add(Weight::from_ref_time(3_511_137 as u64).saturating_mul(c as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// The range of component `c` is `[0, 1000]`.
	fn batch(c: u32, ) -> Weight {
		// Minimum execution time: 13_214 nanoseconds.
		Weight::from_ref_time(23_135_099 as u64)
			// Standard Error: 1_709
			.saturating_add(Weight::from_ref_time(3_522_538 as u64).saturating_mul(c as u64))
	}
	fn as_derivative() -> Weight {
		// Minimum execution time: 6_262 nanoseconds.
		Weight::from_ref_time(6_439_000 as u64)
	}
	/// The range of component `c` is `[0, 1000]`.
	fn batch_all(c: u32, ) -> Weight {
		// Minimum execution time: 13_490 nanoseconds.
		Weight::from_ref_time(22_483_716 as u64)
			// Standard Error: 1_778
			.saturating_add(Weight::from_ref_time(3_596_168 as u64).saturating_mul(c as u64))
	}
	fn dispatch_as() -> Weight {
		// Minimum execution time: 15_588 nanoseconds.
		Weight::from_ref_time(15_954_000 as u64)
	}
	/// The range of component `c` is `[0, 1000]`.
	fn force_batch(c: u32, ) -> Weight {
		// Minimum execution time: 13_657 nanoseconds.
		Weight::from_ref_time(18_190_291 as u64)
			// Standard Error: 1_576
			.saturating_add(Weight::from_ref_time(3_511_137 as u64).saturating_mul(c as u64))
	}
}
