// This file is part of Tetcore.

// Copyright (C) 2018-2021 Parity Technologies (UK) Ltd.
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

//! Test utilities

#![cfg(test)]

use crate as noble_aura;
use tp_consensus_aura::ed25519::AuthorityId;
use tp_runtime::{
	traits::IdentityLookup,
	testing::{Header, UintAuthorityId},
};
use fabric_support::parameter_types;
use tet_io;
use tet_core::H256;

type UncheckedExtrinsic = fabric_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = fabric_system::mocking::MockBlock<Test>;

fabric_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: fabric_system::{Module, Call, Config, Storage, Event<T>},
		Timestamp: noble_timestamp::{Module, Call, Storage, Inherent},
		Aura: noble_aura::{Module, Call, Storage, Config<T>, Inherent},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub BlockWeights: fabric_system::limits::BlockWeights =
		fabric_system::limits::BlockWeights::simple_max(1024);
	pub const MinimumPeriod: u64 = 1;
}

impl fabric_system::Config for Test {
	type BaseCallFilter = ();
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Index = u64;
	type BlockNumber = u64;
	type Call = Call;
	type Hash = H256;
	type Hashing = ::tp_runtime::traits::BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type NobleInfo = NobleInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
}

impl noble_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = Aura;
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

impl noble_aura::Config for Test {
	type AuthorityId = AuthorityId;
}

pub fn new_test_ext(authorities: Vec<u64>) -> tet_io::TestExternalities {
	let mut t = fabric_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	noble_aura::GenesisConfig::<Test>{
		authorities: authorities.into_iter().map(|a| UintAuthorityId(a).to_public_key()).collect(),
	}.assimilate_storage(&mut t).unwrap();
	t.into()
}
