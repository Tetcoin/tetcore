// This file is part of Tetcore.

// Copyright (C) 2020-2021 Parity Technologies (UK) Ltd.
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

//! Mock file for system benchmarking.

#![cfg(test)]

use tp_runtime::traits::IdentityLookup;
use fabric_support::{
	impl_outer_origin,
	dispatch::{Dispatchable, DispatchInfo, PostDispatchInfo},
};

type AccountId = u64;
type AccountIndex = u32;
type BlockNumber = u64;

impl_outer_origin! {
	pub enum Origin for Test where system = fabric_system {}
}

#[derive(Debug, codec::Encode, codec::Decode)]
pub struct Call;

impl Dispatchable for Call {
	type Origin = ();
	type Config = ();
	type Info = DispatchInfo;
	type PostInfo = PostDispatchInfo;
	fn dispatch(self, _origin: Self::Origin)
		-> tp_runtime::DispatchResultWithInfo<Self::PostInfo> {
			panic!("Do not use dummy implementation for dispatch.");
	}
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Test;

impl fabric_system::Config for Test {
	type BaseCallFilter = ();
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Index = AccountIndex;
	type BlockNumber = BlockNumber;
	type Call = Call;
	type Hash = tet_core::H256;
	type Hashing = ::tp_runtime::traits::BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = tp_runtime::testing::Header;
	type Event = ();
	type BlockHashCount = ();
	type Version = ();
	type NobleInfo = ();
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
}

impl crate::Config for Test {}

pub fn new_test_ext() -> tet_io::TestExternalities {
	let t = fabric_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	tet_io::TestExternalities::new(t)
}
