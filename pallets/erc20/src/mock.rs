#![allow(clippy::from_over_into)]

use sp_core::H256;
use frame_support::parameter_types;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup}, testing::Header,
};
use crate as pallet_erc20;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Module, Call, Config, Storage, Event<T>},
        PalletErc20: pallet_erc20::{Module, Call, Storage, Event<T>}
	}
);

type Balance = u64;
type AccountId = u64;

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl frame_system::Config for Test {
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
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
}

impl pallet_erc20::Config for Test {
    type Event = Event;
    type Balance = Balance;
}

pub const BALANCES: [(AccountId, Balance); 4] = [(1, 500_000), (2, 300_000), (3, 1000), (4, 0)];
pub fn get_test_total_supply() -> Balance {
    BALANCES.iter().map(|(_, y)| y).sum()
}

pub fn get_test_token_name() -> Vec<u8> { String::from("SoraTestToken").as_bytes().to_vec() } 
pub fn get_test_token_sym() -> Vec<u8> { String::from("STT").as_bytes().to_vec() } 

/// Build genesis storage
pub fn new_test_ext() -> frame_support::sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();

    pallet_erc20::GenesisConfig::<Test> {
        balances: BALANCES.iter().map(|(x, y)| (*x, *y)).collect(),
        name: get_test_token_name() ,
        sym: get_test_token_sym(),
        decimals: None
    }
    .assimilate_storage(&mut t)
    .unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}

// get and cut last event
#[allow(clippy::result_unit_err)] 
pub fn last_event() -> Result<Event, ()> {
	match System::events().pop() {
		Some(ev) => Ok(ev.event),
		None => Err(())
	}
}