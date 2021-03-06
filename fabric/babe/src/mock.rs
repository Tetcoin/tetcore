// This file is part of Tetcore.

// Copyright (C) 2019-2021 Parity Technologies (UK) Ltd.
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

use codec::Encode;
use crate::{self as noble_babe, Config, CurrentSlot};
use tp_runtime::{
	Perbill, impl_opaque_keys,
	curve::PiecewiseLinear,
	testing::{Digest, DigestItem, Header, TestXt,},
	traits::{Header as _, IdentityLookup, OpaqueKeys},
};
use fabric_system::InitKind;
use fabric_support::{
	parameter_types, StorageValue,
	traits::{KeyOwnerProofSystem, OnInitialize},
	weights::Weight,
};
use tet_io;
use tet_core::{H256, U256, crypto::{IsWrappedBy, KeyTypeId, Pair}};
use tp_consensus_babe::{AuthorityId, AuthorityPair, Slot};
use tp_consensus_vrf::schnorrkel::{VRFOutput, VRFProof};
use tp_staking::SessionIndex;
use noble_staking::EraIndex;
use noble_session::historical as noble_session_historical;

type DummyValidatorId = u64;

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
		Historical: noble_session_historical::{Module},
		Offences: noble_offences::{Module, Call, Storage, Event},
		Babe: noble_babe::{Module, Call, Storage, Config, Inherent, ValidateUnsigned},
		Staking: noble_staking::{Module, Call, Storage, Config<T>, Event<T>},
		Session: noble_session::{Module, Call, Storage, Event, Config<T>},
		Timestamp: noble_timestamp::{Module, Call, Storage, Inherent},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const EpochDuration: u64 = 3;
	pub const ExpectedBlockTime: u64 = 1;
	pub const DisabledValidatorsThreshold: Perbill = Perbill::from_percent(16);
	pub BlockWeights: fabric_system::limits::BlockWeights =
		fabric_system::limits::BlockWeights::simple_max(1024);
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
	type Version = ();
	type Hashing = tp_runtime::traits::BlakeTwo256;
	type AccountId = DummyValidatorId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type NobleInfo = NobleInfo;
	type AccountData = noble_balances::AccountData<u128>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
}

impl<C> fabric_system::offchain::SendTransactionTypes<C> for Test
where
	Call: From<C>,
{
	type OverarchingCall = Call;
	type Extrinsic = TestXt<Call, ()>;
}

impl_opaque_keys! {
	pub struct MockSessionKeys {
		pub babe_authority: super::Module<Test>,
	}
}

impl noble_session::Config for Test {
	type Event = Event;
	type ValidatorId = <Self as fabric_system::Config>::AccountId;
	type ValidatorIdOf = noble_staking::StashOf<Self>;
	type ShouldEndSession = Babe;
	type NextSessionRotation = Babe;
	type SessionManager = noble_session::historical::NoteHistoricalRoot<Self, Staking>;
	type SessionHandler = <MockSessionKeys as OpaqueKeys>::KeyTypeIdProviders;
	type Keys = MockSessionKeys;
	type DisabledValidatorsThreshold = DisabledValidatorsThreshold;
	type WeightInfo = ();
}

impl noble_session::historical::Config for Test {
	type FullIdentification = noble_staking::Exposure<u64, u128>;
	type FullIdentificationOf = noble_staking::ExposureOf<Self>;
}

parameter_types! {
	pub const UncleGenerations: u64 = 0;
}

impl noble_authorship::Config for Test {
	type FindAuthor = noble_session::FindAccountFromAuthorIndex<Self, Babe>;
	type UncleGenerations = UncleGenerations;
	type FilterUncle = ();
	type EventHandler = ();
}

parameter_types! {
	pub const MinimumPeriod: u64 = 1;
}

impl noble_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = Babe;
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

parameter_types! {
	pub const ExistentialDeposit: u128 = 1;
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

noble_staking_reward_curve::build! {
	const REWARD_CURVE: PiecewiseLinear<'static> = curve!(
		min_inflation: 0_025_000u64,
		max_inflation: 0_100_000,
		ideal_stake: 0_500_000,
		falloff: 0_050_000,
		max_piece_count: 40,
		test_precision: 0_005_000,
	);
}

parameter_types! {
	pub const SessionsPerEra: SessionIndex = 3;
	pub const BondingDuration: EraIndex = 3;
	pub const SlashDeferDuration: EraIndex = 0;
	pub const AttestationPeriod: u64 = 100;
	pub const RewardCurve: &'static PiecewiseLinear<'static> = &REWARD_CURVE;
	pub const MaxNominatorRewardedPerValidator: u32 = 64;
	pub const ElectionLookahead: u64 = 0;
	pub const StakingUnsignedPriority: u64 = u64::max_value() / 2;
}

impl noble_staking::Config for Test {
	type RewardRemainder = ();
	type CurrencyToVote = fabric_support::traits::SaturatingCurrencyToVote;
	type Event = Event;
	type Currency = Balances;
	type Slash = ();
	type Reward = ();
	type SessionsPerEra = SessionsPerEra;
	type BondingDuration = BondingDuration;
	type SlashDeferDuration = SlashDeferDuration;
	type SlashCancelOrigin = fabric_system::EnsureRoot<Self::AccountId>;
	type SessionInterface = Self;
	type UnixTime = noble_timestamp::Module<Test>;
	type RewardCurve = RewardCurve;
	type MaxNominatorRewardedPerValidator = MaxNominatorRewardedPerValidator;
	type NextNewSession = Session;
	type ElectionLookahead = ElectionLookahead;
	type Call = Call;
	type UnsignedPriority = StakingUnsignedPriority;
	type MaxIterations = ();
	type MinSolutionScoreBump = ();
	type OffchainSolutionWeightLimit = ();
	type WeightInfo = ();
}

parameter_types! {
	pub OffencesWeightSoftLimit: Weight = Perbill::from_percent(60)
		* BlockWeights::get().max_block;
}

impl noble_offences::Config for Test {
	type Event = Event;
	type IdentificationTuple = noble_session::historical::IdentificationTuple<Self>;
	type OnOffenceHandler = Staking;
	type WeightSoftLimit = OffencesWeightSoftLimit;
}

impl Config for Test {
	type EpochDuration = EpochDuration;
	type ExpectedBlockTime = ExpectedBlockTime;
	type EpochChangeTrigger = crate::ExternalTrigger;

	type KeyOwnerProofSystem = Historical;

	type KeyOwnerProof =
		<Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, AuthorityId)>>::Proof;

	type KeyOwnerIdentification = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(
		KeyTypeId,
		AuthorityId,
	)>>::IdentificationTuple;

	type HandleEquivocation = super::EquivocationHandler<Self::KeyOwnerIdentification, Offences>;
	type WeightInfo = ();
}

pub fn go_to_block(n: u64, s: u64) {
	use fabric_support::traits::OnFinalize;

	System::on_finalize(System::block_number());
	Session::on_finalize(System::block_number());
	Staking::on_finalize(System::block_number());

	let parent_hash = if System::block_number() > 1 {
		let hdr = System::finalize();
		hdr.hash()
	} else {
		System::parent_hash()
	};

	let pre_digest = make_secondary_plain_pre_digest(0, s.into());

	System::initialize(&n, &parent_hash, &pre_digest, InitKind::Full);
	System::set_block_number(n);
	Timestamp::set_timestamp(n);

	if s > 1 {
		CurrentSlot::put(Slot::from(s));
	}

	System::on_initialize(n);
	Session::on_initialize(n);
	Staking::on_initialize(n);
}

/// Slots will grow accordingly to blocks
pub fn progress_to_block(n: u64) {
	let mut slot = u64::from(Babe::current_slot()) + 1;
	for i in System::block_number() + 1 ..= n {
		go_to_block(i, slot);
		slot += 1;
	}
}

/// Progress to the first block at the given session
pub fn start_session(session_index: SessionIndex) {
	let missing = (session_index - Session::current_index()) * 3;
	progress_to_block(System::block_number() + missing as u64 + 1);
	assert_eq!(Session::current_index(), session_index);
}

/// Progress to the first block at the given era
pub fn start_era(era_index: EraIndex) {
	start_session((era_index * 3).into());
	assert_eq!(Staking::current_era(), Some(era_index));
}

pub fn make_primary_pre_digest(
	authority_index: tp_consensus_babe::AuthorityIndex,
	slot: tp_consensus_babe::Slot,
	vrf_output: VRFOutput,
	vrf_proof: VRFProof,
) -> Digest {
	let digest_data = tp_consensus_babe::digests::PreDigest::Primary(
		tp_consensus_babe::digests::PrimaryPreDigest {
			authority_index,
			slot,
			vrf_output,
			vrf_proof,
		}
	);
	let log = DigestItem::PreRuntime(tp_consensus_babe::BABE_ENGINE_ID, digest_data.encode());
	Digest { logs: vec![log] }
}

pub fn make_secondary_plain_pre_digest(
	authority_index: tp_consensus_babe::AuthorityIndex,
	slot: tp_consensus_babe::Slot,
) -> Digest {
	let digest_data = tp_consensus_babe::digests::PreDigest::SecondaryPlain(
		tp_consensus_babe::digests::SecondaryPlainPreDigest {
			authority_index,
			slot,
		}
	);
	let log = DigestItem::PreRuntime(tp_consensus_babe::BABE_ENGINE_ID, digest_data.encode());
	Digest { logs: vec![log] }
}

pub fn make_secondary_vrf_pre_digest(
	authority_index: tp_consensus_babe::AuthorityIndex,
	slot: tp_consensus_babe::Slot,
	vrf_output: VRFOutput,
	vrf_proof: VRFProof,
) -> Digest {
	let digest_data = tp_consensus_babe::digests::PreDigest::SecondaryVRF(
		tp_consensus_babe::digests::SecondaryVRFPreDigest {
			authority_index,
			slot,
			vrf_output,
			vrf_proof,
		}
	);
	let log = DigestItem::PreRuntime(tp_consensus_babe::BABE_ENGINE_ID, digest_data.encode());
	Digest { logs: vec![log] }
}

pub fn make_vrf_output(
	slot: Slot,
	pair: &tp_consensus_babe::AuthorityPair
) -> (VRFOutput, VRFProof, [u8; 32]) {
	let pair = tet_core::sr25519::Pair::from_ref(pair).as_ref();
	let transcript = tp_consensus_babe::make_transcript(&Babe::randomness(), slot, 0);
	let vrf_inout = pair.vrf_sign(transcript);
	let vrf_randomness: tp_consensus_vrf::schnorrkel::Randomness = vrf_inout.0
		.make_bytes::<[u8; 32]>(&tp_consensus_babe::BABE_VRF_INOUT_CONTEXT);
	let vrf_output = VRFOutput(vrf_inout.0.to_output());
	let vrf_proof = VRFProof(vrf_inout.1);

	(vrf_output, vrf_proof, vrf_randomness)
}

pub fn new_test_ext(authorities_len: usize) -> tet_io::TestExternalities {
	new_test_ext_with_pairs(authorities_len).1
}

pub fn new_test_ext_with_pairs(authorities_len: usize) -> (Vec<AuthorityPair>, tet_io::TestExternalities) {
	let pairs = (0..authorities_len).map(|i| {
		AuthorityPair::from_seed(&U256::from(i).into())
	}).collect::<Vec<_>>();

	let public = pairs.iter().map(|p| p.public()).collect();

	(pairs, new_test_ext_raw_authorities(public))
}

pub fn new_test_ext_raw_authorities(authorities: Vec<AuthorityId>) -> tet_io::TestExternalities {
	let mut t = fabric_system::GenesisConfig::default()
		.build_storage::<Test>()
		.unwrap();

	let balances: Vec<_> = (0..authorities.len())
		.map(|i| (i as u64, 10_000_000))
		.collect();

	noble_balances::GenesisConfig::<Test> { balances }
		.assimilate_storage(&mut t)
		.unwrap();

	// stashes are the index.
	let session_keys: Vec<_> = authorities
		.iter()
		.enumerate()
		.map(|(i, k)| {
			(
				i as u64,
				i as u64,
				MockSessionKeys {
					babe_authority: AuthorityId::from(k.clone()),
				},
			)
		})
		.collect();

	// NOTE: this will initialize the babe authorities
	// through OneSessionHandler::on_genesis_session
	noble_session::GenesisConfig::<Test> { keys: session_keys }
		.assimilate_storage(&mut t)
		.unwrap();

	// controllers are the index + 1000
	let stakers: Vec<_> = (0..authorities.len())
		.map(|i| {
			(
				i as u64,
				i as u64 + 1000,
				10_000,
				noble_staking::StakerStatus::<u64>::Validator,
			)
		})
		.collect();

	let staking_config = noble_staking::GenesisConfig::<Test> {
		stakers,
		validator_count: 8,
		force_era: noble_staking::Forcing::ForceNew,
		minimum_validator_count: 0,
		invulnerables: vec![],
		..Default::default()
	};

	staking_config.assimilate_storage(&mut t).unwrap();

	t.into()
}

/// Creates an equivocation at the current block, by generating two headers.
pub fn generate_equivocation_proof(
	offender_authority_index: u32,
	offender_authority_pair: &AuthorityPair,
	slot: Slot,
) -> tp_consensus_babe::EquivocationProof<Header> {
	use tp_consensus_babe::digests::CompatibleDigestItem;

	let current_block = System::block_number();
	let current_slot = CurrentSlot::get();

	let make_header = || {
		let parent_hash = System::parent_hash();
		let pre_digest = make_secondary_plain_pre_digest(offender_authority_index, slot);
		System::initialize(&current_block, &parent_hash, &pre_digest, InitKind::Full);
		System::set_block_number(current_block);
		Timestamp::set_timestamp(current_block);
		System::finalize()
	};

	// sign the header prehash and sign it, adding it to the block as the seal
	// digest item
	let seal_header = |header: &mut Header| {
		let prehash = header.hash();
		let seal = <DigestItem as CompatibleDigestItem>::babe_seal(
			offender_authority_pair.sign(prehash.as_ref()),
		);
		header.digest_mut().push(seal);
	};

	// generate two headers at the current block
	let mut h1 = make_header();
	let mut h2 = make_header();

	seal_header(&mut h1);
	seal_header(&mut h2);

	// restore previous runtime state
	go_to_block(current_block, *current_slot);

	tp_consensus_babe::EquivocationProof {
		slot,
		offender: offender_authority_pair.public(),
		first_header: h1,
		second_header: h2,
	}
}
