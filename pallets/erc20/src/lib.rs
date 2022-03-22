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
	/// Id of last trade everud request
	pub(super) type TotalSupply<T: Config> = StorageValue<_, T::Balance, ValueQuery>;

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

    }
}