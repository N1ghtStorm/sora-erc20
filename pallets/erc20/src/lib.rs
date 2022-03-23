#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]


#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

pub use pallet::*;
use frame_support::{
    codec::{Decode, Encode, Codec},
    sp_runtime::RuntimeDebug,
};
use frame_support::sp_runtime::sp_std::{fmt::Debug};

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
		dispatch::{DispatchResultWithPostInfo},
		pallet_prelude::*,
	};
	use frame_system::pallet_prelude::*;
	use frame_support::dispatch::Vec;
	use super::*;
	use frame_support::sp_runtime::{
		RuntimeDebug, DispatchResult, DispatchError,
		traits::{
			Zero, AtLeast32BitUnsigned, StaticLookup, CheckedAdd, CheckedSub,
			MaybeSerializeDeserialize, Saturating, Bounded, StoredMapError,
		},
	};

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
        // pub creator: T::AccountId,
		a: PhantomData<T>
    }

	#[cfg(feature = "std")]
    impl<T: Config> GenesisConfig<T> {
        /// Direct implementation of `GenesisBuild::build_storage`.
        ///
        /// Kept in order not to break dependency.
        pub fn build_storage(&self) -> Result<sp_runtime::Storage, String> {
            <Self as GenesisBuild<T>>::build_storage(self)
        }

        /// Direct implementation of `GenesisBuild::assimilate_storage`.
        ///
        /// Kept in order not to break dependency.
        pub fn assimilate_storage(&self, storage: &mut sp_runtime::Storage) -> Result<(), String> {
            <Self as GenesisBuild<T>>::assimilate_storage(self, storage)
        }
    }

    #[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { 
				// creator: Default::default() 
				a: Default::default() 
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
            // <CreatorRegistry<T>>::insert(&self.creator, ());
		}
	}


    #[pallet::config]
	/// The module configuration trait.
	pub trait Config: 
		frame_system::Config +
	{
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		// type Balance: Parameter + Member + AtLeast32BitUnsigned + Codec + Default + Copy +
		// 			MaybeSerializeDeserialize + Debug;
		// type Balance: Debug + Default + Copy;
		type Balance: Parameter + AtLeast32BitUnsigned + Codec + Default + Copy + Debug;
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

	#[pallet::storage]
	#[pallet::getter(fn get_allowance)]
	pub(super) type AllowanceOf<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		T::AccountId,
		T::Balance,
		ValueQuery
	>;

	#[pallet::error]
	pub enum Error<T> {
		TransferAmountExceedsBalance,
		DecreasedAllowanceBelowZero,
		BalanceOverflow,
		InsufficientAllowance,
    }

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	#[pallet::metadata(T::AccountId = "AccountId", T::Balance = "Balance")]
	pub enum Event<T: Config> {
		Transfer(T::AccountId, T::AccountId, T::Balance),
		Approval(T::AccountId, T::AccountId, T::Balance),
    }
	
	#[deprecated(note = "use `Event` instead")]
	pub type RawEvent<T> = Event<T>;

	/// Calls:
    #[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1))]
        pub fn transfer(origin: OriginFor<T>, to: T::AccountId, amount: T::Balance) -> DispatchResultWithPostInfo {
			let from = ensure_signed(origin)?;
			Self::transfer_impl(from, to, amount)?;
            Ok(().into())
        }

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1))]
        pub fn approve(origin: OriginFor<T>, spender: T::AccountId, amount: T::Balance) -> DispatchResultWithPostInfo {
			let owner = ensure_signed(origin)?;
			Self::approve_impl(owner, spender, amount)?;
            Ok(().into())
        }

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1))]
        pub fn transfer_from(
			origin: OriginFor<T>, 
			from: T::AccountId,
			to: T::AccountId, 
			amount: T::Balance
		) -> DispatchResultWithPostInfo {
			let spender = ensure_signed(origin)?;
			Self::spend_allowance_impl(from.clone(), spender, amount)?;
			Self::transfer_impl(from, to, amount)?;
            Ok(().into())
        }

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1))]
        pub fn increase_allowance(origin: OriginFor<T>, sender: T::AccountId, added_value: T::Balance) -> DispatchResultWithPostInfo {
			let owner = ensure_signed(origin)?;
			let amount = AllowanceOf::<T>::get(&owner, &sender)
													.checked_add(&added_value)
													.ok_or(Error::<T>::BalanceOverflow)?;

			Self::approve_impl(owner, sender, amount)?;
            Ok(().into())
        }

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1))]
        pub fn decrease_allowance(origin: OriginFor<T>, sender: T::AccountId, substracted_value: T::Balance) -> DispatchResultWithPostInfo {
			let owner = ensure_signed(origin)?;
			let current_allowance = AllowanceOf::<T>::get(&owner, &sender);
			ensure!(current_allowance >= substracted_value, Error::<T>::DecreasedAllowanceBelowZero);
			let amount = current_allowance.checked_sub(&substracted_value)
																.ok_or(Error::<T>::BalanceOverflow)?;

			Self::approve_impl(owner, sender, amount)?;
            Ok(().into())
        }
    }

	impl<T: Config> Pallet<T> {
		pub fn transfer_impl(from: T::AccountId, to: T::AccountId, amount: T::Balance) -> DispatchResultWithPostInfo {			
			BalanceOf::<T>::try_mutate(&from, |from_bal| -> DispatchResultWithPostInfo {
				ensure!(*from_bal >= amount, Error::<T>::TransferAmountExceedsBalance);
				BalanceOf::<T>::try_mutate(&to, |to_bal| -> DispatchResultWithPostInfo {
					from_bal.checked_sub(&amount);
					to_bal.checked_add(&amount);
					Ok(().into())
				})?;
				Ok(().into())
			})?;
			Self::deposit_event(Event::Transfer(from, to, amount));
			Ok(().into())
		}

		pub fn approve_impl(owner: T::AccountId, spender: T::AccountId, amount: T::Balance) -> DispatchResultWithPostInfo {
			AllowanceOf::<T>::try_mutate(&owner, &spender, |bal| -> DispatchResultWithPostInfo {
				*bal = amount;
				Ok(().into())
			})?;
			Self::deposit_event(Event::Approval(owner, spender, amount));
			Ok(().into())
		}

		pub fn spend_allowance_impl(owner: T::AccountId, spender: T::AccountId, amount: T::Balance) -> DispatchResultWithPostInfo {
			let current_allowance = AllowanceOf::<T>::get(&owner, &spender);
			if current_allowance != T::Balance::max_value() {
				let new_allowance = current_allowance.checked_sub(&amount)
									.ok_or( Error::<T>::InsufficientAllowance)?;
				Self::approve_impl(owner, spender, new_allowance)?;
			}
			Ok(().into())
		}
    }
}