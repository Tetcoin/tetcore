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

//! Test utilities

use super::*;

use fabric_support::{parameter_types, traits::{OnInitialize, OnFinalize}};
use tet_core::H256;
use tp_runtime::{
	traits::{BlakeTwo256, IdentityLookup}, testing::Header,
};
use crate as recovery;

type UncheckedExtrinsic = fabric_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = fabric_system::mocking::MockBlock<Test>;

fabric_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: fabric_system::{Module, Call, Config, Storage, Event<T>},
		Balances: noble_balances::{Module, Call, Storage, Config<T>, Event<T>},
		Recovery: recovery::{Module, Call, Storage, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub BlockWeights: fabric_system::limits::BlockWeights =
		fabric_system::limits::BlockWeights::simple_max(1024);
}

impl fabric_system::Config for Test {
	type BaseCallFilter = ();
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type NobleInfo = NobleInfo;
	type AccountData = noble_balances::AccountData<u128>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}

impl noble_balances::Config for Test {
	type MaxLocks = ();
	type Balance = u128;
	type DustRemoval = ();
	type Event = Event;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}

parameter_types! {
	pub const ConfigDepositBase: u64 = 10;
	pub const FriendDepositFactor: u64 = 1;
	pub const MaxFriends: u16 = 3;
	pub const RecoveryDeposit: u64 = 10;
}

impl Config for Test {
	type Event = Event;
	type Call = Call;
	type Currency = Balances;
	type ConfigDepositBase = ConfigDepositBase;
	type FriendDepositFactor = FriendDepositFactor;
	type MaxFriends = MaxFriends;
	type RecoveryDeposit = RecoveryDeposit;
}

pub type BalancesCall = noble_balances::Call<Test>;
pub type RecoveryCall = super::Call<Test>;

pub fn new_test_ext() -> tet_io::TestExternalities {
	let mut t = fabric_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	noble_balances::GenesisConfig::<Test> {
		balances: vec![(1, 100), (2, 100), (3, 100), (4, 100), (5, 100)],
	}.assimilate_storage(&mut t).unwrap();
	t.into()
}

/// Run until a particular block.
pub fn run_to_block(n: u64) {
	while System::block_number() < n {
		if System::block_number() > 1 {
			System::on_finalize(System::block_number());
		}
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
	}
}
