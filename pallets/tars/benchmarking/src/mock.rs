// Copyright (C) 2019-2021 Calcu Network Technologies Ltd.
// This file is part of Calcu.

use crate::*;

pub use frame_support::{
    parameter_types,
    weights::{Weight, constants::RocksDbWeight},
    traits::{OnInitialize, OnFinalize, Get, TestRandomness}
};
pub use sp_core::{crypto::{AccountId32, Ss58Codec}, H256};
use sp_runtime::{
    testing::Header, ModuleId,
    traits::{BlakeTwo256, IdentityLookup},
    Perbill,
};
pub use murphy::{Replica, FileInfo, UsedInfo};
use tars::Works;
use balances::AccountData;
pub use std::{cell::RefCell, collections::HashMap, borrow::Borrow, iter::FromIterator};

pub type AccountId = AccountId32;
pub type Balance = u64;

thread_local! {
    static EXISTENTIAL_DEPOSIT: RefCell<u64> = RefCell::new(0);
}

pub struct ExistentialDeposit;
impl Get<u64> for ExistentialDeposit {
    fn get() -> u64 {
        EXISTENTIAL_DEPOSIT.with(|v| *v.borrow())
    }
}

parameter_types! {
	pub const BlockHashCount: u32 = 250;
	pub BlockWeights: frame_system::limits::BlockWeights =
		frame_system::limits::BlockWeights::simple_max(4 * 1024 * 1024);
}

impl system::Config for Test {
    type BaseCallFilter = ();
    type BlockWeights = BlockWeights;
    type BlockLength = ();
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = ();
    type BlockHashCount = BlockHashCount;
    type DbWeight = RocksDbWeight;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
}

impl balances::Config for Test {
    type Balance = Balance;
    type DustRemoval = ();
    type Event = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxLocks = ();
}

parameter_types! {
    /// Unit is pico
    pub const MurphyModuleId: ModuleId = ModuleId(*b"crmurphy");
    pub const FileDuration: BlockNumber = 1000;
    pub const FileReplica: u32 = 4;
    pub const FileBaseFee: Balance = 1000;
    pub const FileInitPrice: Balance = 1000; // Need align with FileDuration and FileBaseReplica
    pub const StorageReferenceRatio: (u128, u128) = (1, 2);
    pub const StorageIncreaseRatio: Perbill = Perbill::from_percent(1);
    pub const StorageDecreaseRatio: Perbill = Perbill::from_percent(1);
    pub const StakingRatio: Perbill = Perbill::from_percent(80);
    pub const TaxRatio: Perbill = Perbill::from_percent(10);
    pub const UsedTrashMaxSize: u128 = 2;
    pub const MaximumFileSize: u64 = 137_438_953_472; // 128G = 128 * 1024 * 1024 * 1024
    pub const RenewRewardRatio: Perbill = Perbill::from_percent(5);
}

impl murphy::Config for Test {
    type ModuleId = MurphyModuleId;
    type Currency = balances::Module<Self>;
    type CurrencyToBalance = ();
    type TarsInterface = Tars;
    type Event = ();
    /// File duration.
    type FileDuration = FileDuration;
    type FileReplica = FileReplica;
    type FileBaseFee = FileBaseFee;
    type FileInitPrice = FileInitPrice;
    type StorageReferenceRatio = StorageReferenceRatio;
    type StorageIncreaseRatio = StorageIncreaseRatio;
    type StorageDecreaseRatio = StorageDecreaseRatio;
    type StakingRatio = StakingRatio;
    type RenewRewardRatio = RenewRewardRatio;
    type TaxRatio = TaxRatio;
    type UsedTrashMaxSize = UsedTrashMaxSize;
    type MaximumFileSize = MaximumFileSize;
    type WeightInfo = murphy::weight::WeightInfo<Test>;
}

pub struct TestWorksInterface;

impl Works<AccountId> for TestWorksInterface {
    fn report_works(_: BTreeMap<AccountId, u128>, _: u128) {}
}

parameter_types! {
    pub const PunishmentSlots: u32 = 1;
    pub const MaxGroupSize: u32 = 100;
}

impl tars::Config for Test {
    type Currency = balances::Module<Self>;
    type Event = ();
    type PunishmentSlots = PunishmentSlots;
    type Works = TestWorksInterface;
    type MurphyInterface = Murphy;
    type MaxGroupSize = MaxGroupSize;
    type WeightInfo = tars::weight::WeightInfo<Test>;
}

impl crate::Config for Test {}

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: frame_system::{Module, Call, Config, Storage, Event<T>},
		Balances: balances::{Module, Call, Storage, Config<T>, Event<T>},
		Tars: tars::{Module, Call, Storage, Event<T>, Config},
		Murphy: murphy::{Module, Call, Storage, Event<T>, Config},
	}
);

pub struct ExtBuilder { }

impl Default for ExtBuilder {
    fn default() -> Self {
        Self { }
    }
}

impl ExtBuilder {
    pub fn build(self) -> sp_io::TestExternalities {
        let t = system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();

        t.into()
    }
}
