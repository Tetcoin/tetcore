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

//! Mock file for offences benchmarking.

#![cfg(test)]

use super::*;
use fabric_support::{
	parameter_types,
	weights::{Weight, constants::WEIGHT_PER_SECOND},
};
use fabric_system as system;
use tp_runtime::{
	traits::IdentityLookup,
	testing::{Header, UintAuthorityId},
};


type AccountId = u64;
type AccountIndex = u32;
type BlockNumber = u64;
type Balance = u64;

parameter_types! {
	pub BlockWeights: fabric_system::limits::BlockWeights =
		fabric_system::limits::BlockWeights::simple_max(2 * WEIGHT_PER_SECOND);
}

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
	type Event = Event;
	type BlockHashCount = ();
	type Version = ();
	type NobleInfo = ();
	type AccountData = noble_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
}
parameter_types! {
	pub const ExistentialDeposit: Balance = 10;
}
impl noble_balances::Config for Test {
	type MaxLocks = ();
	type Balance = Balance;
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}

parameter_types! {
	pub const MinimumPeriod: u64 = 5;
}
impl noble_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}
impl noble_session::historical::Config for Test {
	type FullIdentification = noble_staking::Exposure<AccountId, Balance>;
	type FullIdentificationOf = noble_staking::ExposureOf<Test>;
}

tp_runtime::impl_opaque_keys! {
	pub struct SessionKeys {
		pub foo: tp_runtime::testing::UintAuthorityId,
	}
}

pub struct TestSessionHandler;
impl noble_session::SessionHandler<AccountId> for TestSessionHandler {
	const KEY_TYPE_IDS: &'static [tp_runtime::KeyTypeId] = &[];

	fn on_genesis_session<Ks: tp_runtime::traits::OpaqueKeys>(_validators: &[(AccountId, Ks)]) {}

	fn on_new_session<Ks: tp_runtime::traits::OpaqueKeys>(
		_: bool,
		_: &[(AccountId, Ks)],
		_: &[(AccountId, Ks)],
	) {}

	fn on_disabled(_: usize) {}
}

parameter_types! {
	pub const Period: u64 = 1;
	pub const Offset: u64 = 0;
}

impl noble_session::Config for Test {
	type SessionManager = noble_session::historical::NoteHistoricalRoot<Test, Staking>;
	type Keys = SessionKeys;
	type ShouldEndSession = noble_session::PeriodicSessions<Period, Offset>;
	type NextSessionRotation = noble_session::PeriodicSessions<Period, Offset>;
	type SessionHandler = TestSessionHandler;
	type Event = Event;
	type ValidatorId = AccountId;
	type ValidatorIdOf = noble_staking::StashOf<Test>;
	type DisabledValidatorsThreshold = ();
	type WeightInfo = ();
}
noble_staking_reward_curve::build! {
	const I_NPOS: tp_runtime::curve::PiecewiseLinear<'static> = curve!(
		min_inflation: 0_025_000,
		max_inflation: 0_100_000,
		ideal_stake: 0_500_000,
		falloff: 0_050_000,
		max_piece_count: 40,
		test_precision: 0_005_000,
	);
}
parameter_types! {
	pub const RewardCurve: &'static tp_runtime::curve::PiecewiseLinear<'static> = &I_NPOS;
	pub const MaxNominatorRewardedPerValidator: u32 = 64;
}

pub type Extrinsic = tp_runtime::testing::TestXt<Call, ()>;

impl noble_staking::Config for Test {
	type Currency = Balances;
	type UnixTime = noble_timestamp::Module<Self>;
	type CurrencyToVote = fabric_support::traits::SaturatingCurrencyToVote;
	type RewardRemainder = ();
	type Event = Event;
	type Slash = ();
	type Reward = ();
	type SessionsPerEra = ();
	type SlashDeferDuration = ();
	type SlashCancelOrigin = fabric_system::EnsureRoot<Self::AccountId>;
	type BondingDuration = ();
	type SessionInterface = Self;
	type RewardCurve = RewardCurve;
	type NextNewSession = Session;
	type ElectionLookahead = ();
	type Call = Call;
	type MaxNominatorRewardedPerValidator = MaxNominatorRewardedPerValidator;
	type UnsignedPriority = ();
	type MaxIterations = ();
	type MinSolutionScoreBump = ();
	type OffchainSolutionWeightLimit = ();
	type WeightInfo = ();
}

impl noble_im_online::Config for Test {
	type AuthorityId = UintAuthorityId;
	type Event = Event;
	type SessionDuration = Period;
	type ReportUnresponsiveness = Offences;
	type UnsignedPriority = ();
	type WeightInfo = ();
}

parameter_types! {
	pub OffencesWeightSoftLimit: Weight = Perbill::from_percent(60) * BlockWeights::get().max_block;
}

impl noble_offences::Config for Test {
	type Event = Event;
	type IdentificationTuple = noble_session::historical::IdentificationTuple<Self>;
	type OnOffenceHandler = Staking;
	type WeightSoftLimit = OffencesWeightSoftLimit;
}

impl<T> fabric_system::offchain::SendTransactionTypes<T> for Test where Call: From<T> {
	type Extrinsic = Extrinsic;
	type OverarchingCall = Call;
}

impl crate::Config for Test {}

pub type Block = tp_runtime::generic::Block<Header, UncheckedExtrinsic>;
pub type UncheckedExtrinsic = tp_runtime::generic::UncheckedExtrinsic<u32, Call, u64, ()>;

fabric_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: system::{Module, Call, Event<T>},
		Balances: noble_balances::{Module, Call, Storage, Config<T>, Event<T>},
		Staking: noble_staking::{Module, Call, Config<T>, Storage, Event<T>, ValidateUnsigned},
		Session: noble_session::{Module, Call, Storage, Event, Config<T>},
		ImOnline: noble_im_online::{Module, Call, Storage, Event<T>, ValidateUnsigned, Config<T>},
		Offences: noble_offences::{Module, Call, Storage, Event},
	}
);

pub fn new_test_ext() -> tet_io::TestExternalities {
	let t = fabric_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	tet_io::TestExternalities::new(t)
}
