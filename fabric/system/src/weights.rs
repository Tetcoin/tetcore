// This file is part of Tetcore.

// Copyright (C) 2020-2021 Parity Technologies (UK) Ltd.
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

//! Weights for fabric_system
//! THIS FILE WAS AUTO-GENERATED USING THE TETCORE BENCHMARK CLI VERSION 2.0.0
//! DATE: 2020-10-28, STEPS: [50, ], REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/tetcore
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --noble=fabric_system
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./fabric/system/src/weights.rs
// --template=./.maintain/fabric-weight-template.hbs


#![allow(unused_parens)]
#![allow(unused_imports)]

use fabric_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use tetcore_std::marker::PhantomData;

/// Weight functions needed for fabric_system.
pub trait WeightInfo {
	fn remark(b: u32, ) -> Weight;
	fn set_heap_pages() -> Weight;
	fn set_changes_trie_config() -> Weight;
	fn set_storage(i: u32, ) -> Weight;
	fn kill_storage(i: u32, ) -> Weight;
	fn kill_prefix(p: u32, ) -> Weight;
	fn suicide() -> Weight;
}

/// Weights for fabric_system using the Tetcore node and recommended hardware.
pub struct TetcoreWeight<T>(PhantomData<T>);
impl<T: crate::Config> WeightInfo for TetcoreWeight<T> {
	fn remark(_b: u32, ) -> Weight {
		(1_973_000 as Weight)
	}
	fn set_heap_pages() -> Weight {
		(2_816_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_changes_trie_config() -> Weight {
		(11_539_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn set_storage(i: u32, ) -> Weight {
		(0 as Weight)
			.saturating_add((833_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(i as Weight)))
	}
	fn kill_storage(i: u32, ) -> Weight {
		(2_131_000 as Weight)
			.saturating_add((597_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(i as Weight)))
	}
	fn kill_prefix(p: u32, ) -> Weight {
		(11_844_000 as Weight)
			.saturating_add((857_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(p as Weight)))
	}
	fn suicide() -> Weight {
		(37_209_000 as Weight)
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn remark(_b: u32, ) -> Weight {
		(1_973_000 as Weight)
	}
	fn set_heap_pages() -> Weight {
		(2_816_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn set_changes_trie_config() -> Weight {
		(11_539_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	fn set_storage(i: u32, ) -> Weight {
		(0 as Weight)
			.saturating_add((833_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(i as Weight)))
	}
	fn kill_storage(i: u32, ) -> Weight {
		(2_131_000 as Weight)
			.saturating_add((597_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(i as Weight)))
	}
	fn kill_prefix(p: u32, ) -> Weight {
		(11_844_000 as Weight)
			.saturating_add((857_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(p as Weight)))
	}
	fn suicide() -> Weight {
		(37_209_000 as Weight)
	}
}
