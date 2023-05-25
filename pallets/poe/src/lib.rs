#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::{pallet_prelude::*, Config};
	use sp_std::prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The maximum length of claim that can be added.
		#[pallet::constant] // Declaring variable type as constant.
		type  MaxClaimLength: Get<u32>; // Currently only accepting u32 type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	//存储项
	#[pallet::storage]
	pub type Proofs<T: Config> = StorageMap<
	_,
	Blake2_128Concat,
	BoundedVec<u8, T::MaxClaimLength>, // 不能直接使用vec 类型，需要使用长度受限的vec 类型
	(T::AccountId, T::BlockNumber),
	>;


	/// Defining an event.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ClaimCreated(T::AccountId, Vec<u8>), 
		ClaimRevoked(T::AccountId, Vec<u8>), 
	}

	/// Error message.
	#[pallet::error]
	pub enum Error<T> {
		ProofAlreadyExist,
		ClaimTooLong,
		ClaimNotExist,
		NotClaimOwner,
	}

	/// 保留函数
	#[pallet::hooks]
	impl <T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	/// 可调用函数
	#[pallet::call]
	impl <T: Config> Pallet<T> {
		#[pallet::weight(0)] // Specifying weight 0. claim created.
		pub fn create_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo {
			// 校验发送方
			let sender = ensure_signed(origin) ?;

			// Did it exceed the maximum length
			let bounded_claim = BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone())
				.map_err(|_| Error::<T>::ClaimTooLong)?;

			//check key is exist.
			ensure!(!Proofs::<T>::contains_key(&bounded_claim), Error::<T>::ProofAlreadyExist);

			Proofs::<T>::insert(
				&bounded_claim,
				(sender.clone(), frame_system::Pallet::<T>::block_number()),
			);

			Self::deposit_event(Event::ClaimCreated(sender, claim));

            Ok(().into())
		}

		//claim revoke
		#[pallet::weight(0)]
		pub fn revoke_claim(origin: OriginFor<T>, claim: Vec<u8>) ->DispatchResultWithPostInfo {
			let sender  = ensure_signed(origion)?;

			let bounded_claim = 
			BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone()).map_err(|_| Error::<T>::ClaimTooLong)?;
			let (owner, _) = Proofs::<T>::get(&bounded_claim).ok_or(Error::<T>::ClaimNotExist)?;

			ensure!(owner == sender, Error::<T>::NotClaimOwner);
			Proofs::<T>::remove(&bounded_claim);
			Self::deposit_event(Event::ClaimRevoked(sender, claim));

			Ok(().into())
		}
	}

	
}
