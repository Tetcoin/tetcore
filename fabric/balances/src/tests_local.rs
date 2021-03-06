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

use tp_runtime::{
	traits::IdentityLookup,
	testing::Header,
};
use tet_core::H256;
use tet_io;
use fabric_support::parameter_types;
use fabric_support::traits::StorageMapShim;
use fabric_support::weights::{Weight, DispatchInfo, IdentityFee};
use crate::{
	self as noble_balances,
	Module, Config, decl_tests,
};
use noble_transaction_payment::CurrencyAdapter;

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
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub BlockWeights: fabric_system::limits::BlockWeights =
		fabric_system::limits::BlockWeights::simple_max(1024);
	pub static ExistentialDeposit: u64 = 0;
}
impl fabric_system::Config for Test {
	type BaseCallFilter = ();
	type BlockWeights = BlockWeights;
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
parameter_types! {
	pub const TransactionByteFee: u64 = 1;
}
impl noble_transaction_payment::Config for Test {
	type OnChargeTransaction = CurrencyAdapter<Module<Test>, ()>;
	type TransactionByteFee = TransactionByteFee;
	type WeightToFee = IdentityFee<u64>;
	type FeeMultiplierUpdate = ();
}
parameter_types! {
	pub const MaxLocks: u32 = 50;
}
impl Config for Test {
	type Balance = u64;
	type DustRemoval = ();
	type Event = Event;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = StorageMapShim<
		super::Account<Test>,
		system::Provider<Test>,
		u64,
		super::AccountData<u64>,
	>;
	type MaxLocks = MaxLocks;
	type WeightInfo = ();
}

pub struct ExtBuilder {
	existential_deposit: u64,
	monied: bool,
}
impl Default for ExtBuilder {
	fn default() -> Self {
		Self {
			existential_deposit: 1,
			monied: false,
		}
	}
}
impl ExtBuilder {
	pub fn existential_deposit(mut self, existential_deposit: u64) -> Self {
		self.existential_deposit = existential_deposit;
		self
	}
	pub fn monied(mut self, monied: bool) -> Self {
		self.monied = monied;
		if self.existential_deposit == 0 {
			self.existential_deposit = 1;
		}
		self
	}
	pub fn set_associated_consts(&self) {
		EXISTENTIAL_DEPOSIT.with(|v| *v.borrow_mut() = self.existential_deposit);
	}
	pub fn build(self) -> tet_io::TestExternalities {
		self.set_associated_consts();
		let mut t = fabric_system::GenesisConfig::default().build_storage::<Test>().unwrap();
		noble_balances::GenesisConfig::<Test> {
			balances: if self.monied {
				vec![
					(1, 10 * self.existential_deposit),
					(2, 20 * self.existential_deposit),
					(3, 30 * self.existential_deposit),
					(4, 40 * self.existential_deposit),
					(12, 10 * self.existential_deposit)
				]
			} else {
				vec![]
			},
		}.assimilate_storage(&mut t).unwrap();

		let mut ext = tet_io::TestExternalities::new(t);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
}

decl_tests!{ Test, ExtBuilder, EXISTENTIAL_DEPOSIT }

#[test]
fn emit_events_with_no_existential_deposit_suicide_with_dust() {
	<ExtBuilder>::default()
		.existential_deposit(2)
		.build()
		.execute_with(|| {
			assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));

			assert_eq!(
				events(),
				[
					Event::fabric_system(fabric_system::Event::NewAccount(1)),
					Event::noble_balances(RawEvent::Endowed(1, 100)),
					Event::noble_balances(RawEvent::BalanceSet(1, 100, 0)),
				]
			);

			let _ = Balances::slash(&1, 98);

			// no events
			assert_eq!(events(), []);

			let _ = Balances::slash(&1, 1);

			assert_eq!(
				events(),
				[
					Event::noble_balances(RawEvent::DustLost(1, 1)),
					Event::fabric_system(fabric_system::Event::KilledAccount(1))
				]
			);
		});
}
