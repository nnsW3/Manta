// Copyright 2020-2024 Manta Network.
// This file is part of Manta.
//
// Manta is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Manta is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Manta.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for pallet_author_inherent
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-12-21, STEPS: `50`, REPEAT: 40, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("/home/aye/actions-runner/_worker/Manta/Manta/tests/data/fork.json"), DB CACHE: 1024

// Executed Command:
// ./target/production/manta
// benchmark
// pallet
// --chain=/home/aye/actions-runner/_worker/Manta/Manta/tests/data/fork.json
// --steps=50
// --repeat=40
// --pallet=pallet_author_inherent
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./scripts/benchmarking/frame-weights-output/pallet_author_inherent.rs
// --template=.github/resources/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_author_inherent.
pub trait WeightInfo {
	fn kick_off_authorship_validation() -> Weight;
}

/// Weights for pallet_author_inherent using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: ParachainSystem ValidationData (r:1 w:0)
	/// Proof Skipped: ParachainSystem ValidationData (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AuthorInherent HighestSlotSeen (r:1 w:1)
	/// Proof: AuthorInherent HighestSlotSeen (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: AuthorInherent Author (r:1 w:0)
	/// Proof: AuthorInherent Author (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: ParachainStaking SelectedCandidates (r:1 w:0)
	/// Proof Skipped: ParachainStaking SelectedCandidates (max_values: Some(1), max_size: None, mode: Measured)
	fn kick_off_authorship_validation() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `190`
		//  Estimated: `1675`
		// Minimum execution time: 13_680_000 picoseconds.
		Weight::from_parts(14_060_000, 1675)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: ParachainSystem ValidationData (r:1 w:0)
	/// Proof Skipped: ParachainSystem ValidationData (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AuthorInherent HighestSlotSeen (r:1 w:1)
	/// Proof: AuthorInherent HighestSlotSeen (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: AuthorInherent Author (r:1 w:0)
	/// Proof: AuthorInherent Author (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: ParachainStaking SelectedCandidates (r:1 w:0)
	/// Proof Skipped: ParachainStaking SelectedCandidates (max_values: Some(1), max_size: None, mode: Measured)
	fn kick_off_authorship_validation() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `190`
		//  Estimated: `1675`
		// Minimum execution time: 13_680_000 picoseconds.
		Weight::from_parts(14_060_000, 1675)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
