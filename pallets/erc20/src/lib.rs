#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

pub use pallet::*;
use frame_support::{
    codec::{Codec},
};
use frame_support::sp_runtime::sp_std::{fmt::Debug};

pub const DEFAULT_DECIMALS: u8 = 18;

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
		traits::{
			AtLeast32BitUnsigned, CheckedAdd, CheckedSub,
			MaybeSerializeDeserialize, Bounded,
		},
	};

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub balances: Vec<(T::AccountId, T::Balance)>,
		pub name: Vec<u8>,
		pub sym: Vec<u8>,
		pub decimals: Option<u8>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> GenesisConfig<T> {
		pub fn build_storage(&self) -> Result<sp_runtime::Storage, String> {
			<Self as GenesisBuild<T>>::build_storage(self)
		}

		pub fn assimilate_storage(&self, storage: &mut sp_runtime::Storage) -> Result<(), String> {
			<Self as GenesisBuild<T>>::assimilate_storage(self, storage)
		}
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { 
				balances: Default::default(),
				name: Vec::new(),
				sym: Vec::new(),
				decimals: None
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			let total_supply = self.balances
							.iter()
							.map(|(_, y)| *y)
							.fold(T::Balance::default(),|x, y| {
								x.checked_add(&y).expect("Genesis build failed: Total supply overflow")
							});

			TotalSupply::<T>::mutate(|x| *x = total_supply);

			// We can afford Vec<u8>.clone() in genesis build, because it called once and name is not long
			// Doing without O(n) allocation - is std mem swapping to Vec::new() - to move name and sym from genesis config
			Name::<T>::mutate(|x| *x = self.name.clone());
			Symbol::<T>::mutate(|x| *x = self.sym.clone());

			for (acc, bal) in &self.balances {
				BalanceOf::<T>::insert(acc, bal);
			}
		}
	}

	#[pallet::config]
	/// The module configuration trait.
	pub trait Config: 
		frame_system::Config +
	{
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Balance: Parameter + Member + AtLeast32BitUnsigned + Codec + Default + Copy +
					MaybeSerializeDeserialize + Debug;
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}


	#[pallet::type_value]
	pub fn DefaultDecimals() -> u8 { DEFAULT_DECIMALS }

	// pallet storages:
	#[pallet::storage]
	#[pallet::getter(fn get_total_supply)]
	/// Total supply
	pub(super) type TotalSupply<T: Config> = StorageValue<_, T::Balance, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_name)]
	/// Name byte vector
	pub type Name<T: Config> = StorageValue<_, Vec<u8>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_symbol)]
	/// Symbol byte vector
	pub type Symbol<T: Config> = StorageValue<_, Vec<u8>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_decimals)]
	/// Decimals
	pub type Decimals<T: Config> = StorageValue<_, u8, ValueQuery, DefaultDecimals>;

	#[pallet::storage]
	#[pallet::getter(fn get_balance)]
	/// Balance of an account
	pub(super) type BalanceOf<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		T::Balance,
		ValueQuery
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_allowance)]
	/// Allowance of an account given to another account
	pub(super) type AllowanceOf<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		T::AccountId,
		T::Balance,
		ValueQuery
	>;

	// Pallet Errors
	#[pallet::error]
	pub enum Error<T> {
		/// Error for if transfer amount exceeds balance
		TransferAmountExceedsBalance,
		/// Decreases allowance below zero error
		DecreasedAllowanceBelowZero,
		/// Error for balance overflow
		BalanceOverflow,
		/// Allowance insufficient error 
		InsufficientAllowance,
		/// Burn amount exceeds balacnde error
		BurnAmountExceedsBalance,
	}

	// Pallet events
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	#[pallet::metadata(T::AccountId = "AccountId", T::Balance = "Balance")]
	pub enum Event<T: Config> {
		/// \[From, To, Amount\]
		Transfer(T::AccountId, T::AccountId, T::Balance),
		/// \[From, To, Amount\]
		Approval(T::AccountId, T::AccountId, T::Balance),
	}

	#[deprecated(note = "use `Event` instead")]
	pub type RawEvent<T> = Event<T>;

	/// Calls:
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(2, 2))]
		pub fn transfer(origin: OriginFor<T>, to: T::AccountId, amount: T::Balance) -> DispatchResultWithPostInfo {
			let from = ensure_signed(origin)?;
			Self::_transfer(from, to, amount)?;
			Ok(().into())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1))]
		pub fn approve(origin: OriginFor<T>, spender: T::AccountId, amount: T::Balance) -> DispatchResultWithPostInfo {
			let owner = ensure_signed(origin)?;
			Self::_approve(owner, spender, amount)?;
			Ok(().into())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(3, 3))]
		pub fn transfer_from(
			origin: OriginFor<T>, 
			from: T::AccountId,
			to: T::AccountId, 
			amount: T::Balance
		) -> DispatchResultWithPostInfo {
			let spender = ensure_signed(origin)?;
			Self::_spend_allowance(from.clone(), spender, amount)?;
			Self::_transfer(from, to, amount)?;
			Ok(().into())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1))]
		pub fn increase_allowance(origin: OriginFor<T>, sender: T::AccountId, added_value: T::Balance) -> DispatchResultWithPostInfo {
			let owner = ensure_signed(origin)?;
			let amount = AllowanceOf::<T>::get(&owner, &sender)
													.checked_add(&added_value)
													.ok_or(Error::<T>::BalanceOverflow)?;

			Self::_approve(owner, sender, amount)?;
			Ok(().into())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1))]
		pub fn decrease_allowance(origin: OriginFor<T>, sender: T::AccountId, substracted_value: T::Balance) -> DispatchResultWithPostInfo {
			let owner = ensure_signed(origin)?;
			let current_allowance = AllowanceOf::<T>::get(&owner, &sender);
			ensure!(current_allowance >= substracted_value, Error::<T>::DecreasedAllowanceBelowZero);
			let amount = current_allowance.checked_sub(&substracted_value)
																.ok_or(Error::<T>::BalanceOverflow)?;

			Self::_approve(owner, sender, amount)?;
			Ok(().into())
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn _transfer(from: T::AccountId, to: T::AccountId, amount: T::Balance) -> DispatchResultWithPostInfo {			
			BalanceOf::<T>::try_mutate(&from, |from_bal| -> DispatchResultWithPostInfo {
				ensure!(*from_bal >= amount, Error::<T>::TransferAmountExceedsBalance);
				BalanceOf::<T>::try_mutate(&to, |to_bal| -> DispatchResultWithPostInfo {
					*from_bal = from_bal.checked_sub(&amount).ok_or(Error::<T>::BalanceOverflow)?;
					*to_bal = to_bal.checked_add(&amount).ok_or(Error::<T>::BalanceOverflow)?;
					Ok(().into())
				})?;
				Ok(().into())
			})?;
			Self::deposit_event(Event::Transfer(from, to, amount));
			Ok(().into())
		}

		pub fn _approve(owner: T::AccountId, spender: T::AccountId, amount: T::Balance) -> DispatchResultWithPostInfo {
			AllowanceOf::<T>::try_mutate(&owner, &spender, |bal| -> DispatchResultWithPostInfo {
				*bal = amount;
				Ok(().into())
			})?;
			Self::deposit_event(Event::Approval(owner, spender, amount));
			Ok(().into())
		}

		pub fn _spend_allowance(owner: T::AccountId, spender: T::AccountId, amount: T::Balance) -> DispatchResultWithPostInfo {
			let current_allowance = AllowanceOf::<T>::get(&owner, &spender);
			if current_allowance != T::Balance::max_value() {
				let new_allowance = current_allowance.checked_sub(&amount)
									.ok_or( Error::<T>::InsufficientAllowance)?;
				Self::_approve(owner, spender, new_allowance)?;
			}
			Ok(().into())
		}

		pub fn _mint(account: T::AccountId, amount: T::Balance) -> DispatchResultWithPostInfo {
			BalanceOf::<T>::try_mutate(&account, |balance| -> DispatchResultWithPostInfo {
				TotalSupply::<T>::try_mutate(|bal| -> DispatchResultWithPostInfo {
					*bal = bal.checked_add(&amount).ok_or(Error::<T>::BalanceOverflow)?;
					Ok(().into())
				})?;
				*balance = balance.checked_add(&amount).ok_or(Error::<T>::BalanceOverflow)?;
				Ok(().into())
			})?;
			Self::deposit_event(Event::Transfer(T::AccountId::default(), account, amount));
			Ok(().into())
		}

		pub fn _burn(account: T::AccountId, amount: T::Balance) -> DispatchResultWithPostInfo {
			BalanceOf::<T>::try_mutate(&account, |balance| -> DispatchResultWithPostInfo {
				ensure!(*balance >= amount, Error::<T>::BurnAmountExceedsBalance);
				TotalSupply::<T>::try_mutate(|bal| -> DispatchResultWithPostInfo {
					*bal = bal.checked_sub(&amount).ok_or(Error::<T>::BalanceOverflow)?;
					Ok(().into())
				})?;
				*balance = balance.checked_sub(&amount).ok_or(Error::<T>::BalanceOverflow)?;
				Ok(().into())
			})?;
			Self::deposit_event(Event::Transfer(account, T::AccountId::default(), amount));
			Ok(().into())
		}
	}
}