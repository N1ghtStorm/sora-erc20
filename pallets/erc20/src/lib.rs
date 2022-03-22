#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]


#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

// use sp_std::{prelude::*};
pub use pallet::*;
// use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get};
use frame_system::ensure_signed;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
		dispatch::{DispatchResultWithPostInfo},
		pallet_prelude::*,
	};
	use frame_system::pallet_prelude::*;
	use frame_support::dispatch::Vec;
	use super::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);


    #[pallet::config]
	/// The module configuration trait.
	pub trait Config: 
		frame_system::Config +
		pallet_balances::Config +
	{
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

    #[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	// pallet storages:
	#[pallet::storage]
	#[pallet::getter(fn get_total_supply)]
	/// Total supply
	pub(super) type TotalSupply<T: Config> = StorageValue<_, T::Balance, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_name)]
	pub type Name<T: Config> = StorageValue<_, Vec<u8>>;

	#[pallet::storage]
	#[pallet::getter(fn get_symbol)]
	pub type Symbol<T: Config> = StorageValue<_, Vec<u8>>;

	#[pallet::storage]
	#[pallet::getter(fn get_decimals)]
	pub type Decimals<T: Config> = StorageValue<_, u8>;

	#[pallet::storage]
	#[pallet::getter(fn get_balance)]
	pub(super) type BalanceOf<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		T::Balance,
		ValueQuery
	>;

	// #[pallet::storage]
	// #[pallet::getter(fn get_allowance)]
	// pub(super) type AllowanceOf<T> = StorageMap<
	// 	_,
	// 	Blake2_128Concat,
	// 	T::AccountId,
	// 	T::Balance,
	// 	ValueQuery
	// >;

	#[pallet::error]
	pub enum Error<T> {

    }

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	#[pallet::metadata(T::AccountId = "AccountId", T::Balance = "Balance")]
	pub enum Event<T: Config> {

    }
	
	#[deprecated(note = "use `Event` instead")]
	pub type RawEvent<T> = Event<T>;

	/// Calls:
    #[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1))]
        pub fn transfer(origin: OriginFor<T>, to: T::AccountId, amount: T::Balance) -> DispatchResultWithPostInfo {
			todo!();
            Ok(().into())
        }

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1))]
        pub fn approve(origin: OriginFor<T>, sender: T::AccountId, amount: T::Balance) -> DispatchResultWithPostInfo {
			todo!();
            Ok(().into())
        }

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1))]
        pub fn transfer_from(
			origin: OriginFor<T>, 
			from: T::AccountId,
			to: T::AccountId, 
			amount: T::Balance
		) -> DispatchResultWithPostInfo {
			todo!();
            Ok(().into())
        }
    }
}