#![cfg_attr(not(feature = "std"), no_std)]

use frame_system::ensure_signed;
pub use crate::erc721_interface::Erc721;
use frame_support::{
	decl_error, decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure,
	traits::{EnsureOrigin, Get},
	Hashable,
};
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;
mod erc721_interface;
use sp_std::{vec::Vec, vec};


pub trait Trait: frame_system::Trait + chainbridge::Trait {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}


decl_storage! {

	trait Store for Module<T: Trait> as TemplateModule {
		AssetOwner get(fn asset_owner): map hasher(blake2_128_concat) T::Hash => T::AccountId;
		OwnerAssets get(fn owner_assets): map hasher(blake2_128_concat) T::AccountId => Vec<T::Hash>;
	}
}

decl_event!(
	pub enum Event<T> where
	    AccountId = <T as frame_system::Trait>::AccountId,
	    Hash = <T as frame_system::Trait>::Hash,

	{
		Transfered(AccountId, AccountId, Hash),
		Minted(AccountId, Hash),
		Burnt(AccountId, Hash),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Trait> {

		/// AssetIsAlreadyPresent
		AssetisAlreadyPresent,
		/// AssetNotFound
		AssetNotFound,
		/// NotAnOwner
		NotAnOwner,
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		/// This method registers new NFT in the system.
        /// # Arguments
        ///
        /// * `origin` - This contains the detail of Origin from where Transaction originated.
        ///
        /// * `asset_id` - NFT Hash.
        ///
        /// # Return
        ///
        ///  This function returns a status that, new NFT Asset is successfully registered or not.
		#[weight = 10_000]
		pub fn mint_nft(origin, asset_id: T::Hash) -> DispatchResult {
		    let who = ensure_signed(origin)?;
		    <Self as Erc721<_>>::mint_nft(&who, asset_id)?;
		    Ok(())
		}

		/// This method burns NFT from the system.
        /// # Arguments
        ///
        /// * `origin` - This contains the detail of Origin from where Transaction originated.
        ///
        /// * `asset_id` - Hash of NFT Asset.
        ///
        /// # Return
        ///
        ///  This function returns a status that, new NFT Asset is burnt or not.
		#[weight = 10_000]
		pub fn burn_nft(origin, asset_id: T::Hash) -> DispatchResult {
		    let who = ensure_signed(origin)?;
		    ensure!(<AssetOwner<T>>::contains_key(asset_id), Error::<T>::AssetNotFound);
		    ensure!(<AssetOwner<T>>::get(asset_id) == who, Error::<T>::NotAnOwner);
		    <Self as Erc721<_>>::burn_nft(&who, asset_id)?;
		    Ok(())

		}

		/// This method transfers ownership from one user to another.
        /// # Arguments
        ///
        /// * `origin` - This contains the detail of Origin from where Transaction originated.
        ///
        /// * `new-owner` - Account Id of New Owner to whom ownership is to be transferred.
        ///
        /// * `asset_id` - Hash of NFT Asset.
        ///
        /// # Return
        ///
        ///  This function returns a status that, new NFT Asset is transferred or not.
		#[weight = 10_000]
		fn transfer_nft(origin, new_owner: T::AccountId, asset_id: T::Hash) -> DispatchResult {
		    let who = ensure_signed(origin)?;
		    ensure!(<AssetOwner<T>>::contains_key(asset_id), Error::<T>::AssetNotFound);
		    ensure!(<AssetOwner<T>>::get(asset_id) == who, Error::<T>::NotAnOwner);
		    Self::transfer_from(&who, &new_owner, asset_id)?;
		    Ok(())
		}

		/// This method transfers ownership from one user to another.
        /// # Arguments
        ///
        /// * `origin` - This contains the detail of Origin from where Transaction originated.
        ///
        /// * `dest-id` - Destination Chain id to where asset is to be transferred.
        ///
        /// * `recipient` - Account Id of new Owner on other chain
        ///
        /// * `asset_id` - Hash of NFT Asset.
        ///
        /// # Return
        ///
        ///  This function returns a status that, new NFT Asset is transferred or not.
		#[weight = 10_000]
		fn transfer_other_chain(origin, dest_id: ChainId, recipient: Vec<u8>, #[compact] asset_id: T::Hash) -> DispatchResult{
		    let withdrawer = ensure_signed(origin)?;
		    Self::burn_nft(&withdrawer, asset_id);
		    chainbridge::Module::<T>::transfer_nongingible(dest_id, asset_id, recipient)?;
		    Ok(())
		}

	}
}

impl<T: Trait> Erc721 <T::AccountId> for Module<T> {
	type NftId = T::Hash;

	/// This method counts all assets of given Account Id.
	/// # Arguments
	///
	/// * `address` - Account id of Owner.
	///
	/// # Return
	///
	///  This function returns a count of all asset hold by given Account.
	fn balance_of(address: &T::AccountId) -> u64 {
		let vector_of_assets: Vec<T::Hash> = <OwnerAssets<T>>::get(address);
		vector_of_assets.len() as u64
	}

	/// This method finds Owner of given Asset.
	/// # Arguments
	///
	/// * `asset_id` - Account id of Owner.
	///
	/// # Return
	///
	///  This function returns a Owner of Given Asset.
	fn owner_of(asset_id: Self::NftId) -> T::AccountId {
		<AssetOwner<T>>::get(&asset_id)
	}

	/// This method transfers asset from one account to another.
    /// # Arguments
    ///
    /// * `who` - Account id of Owner.
    ///
    /// * `to` - Account id of new Owner.
    ///
    /// * `asset` - Assert Id of NFT
    ///
    /// # Return
    ///
    ///  This function returns a status that, new NFT Asset is transferred or not.
	fn transfer_from(who: &T::AccountId, to: &T::AccountId, asset: Self::NftId) -> DispatchResult {
		<OwnerAssets<T>>::try_mutate(&who, |vec_of_assets_old| {
			<OwnerAssets<T>>::try_mutate(&to, |vec_of_assets_new| {
				let index = vec_of_assets_old.iter().position(|&element| element == asset).ok_or(Error::<T>::AssetNotFound)?;
				vec_of_assets_old.remove(index);
				vec_of_assets_new.push(asset);
				<AssetOwner<T>>::mutate(&asset, |owner| *owner = to.clone());
				Ok(())
			})
		})
	}

	/// This method registers new NFT in the system.
	/// # Arguments
	///
	/// * `who` - This contains the detail of Origin from where Transaction originated.
	///
	/// * `asset_id` - NFT Hash.
	///
	/// # Return
	///
	///  This function returns a status that, new NFT Asset is successfully registered or not.
	fn mint_nft(who: &T::AccountId, asset_id: Self::NftId) -> DispatchResult {
		if <OwnerAssets<T>>::contains_key(who) {
			<OwnerAssets<T>>::mutate(who, |vector_of_assets| {
				vector_of_assets.push(asset_id);
			});
		} else {
			let array_of_assets = vec![asset_id];
			<OwnerAssets<T>>::insert(who, array_of_assets);
		}
		<AssetOwner<T>>::insert(&asset_id, who);
		Ok(())
	}

	/// This method burns NFT from the system.
	/// # Arguments
	///
	/// * `owner` - This contains the detail of Origin from where Transaction originated.
	///
	/// * `asset_id` - Hash of NFT Asset.
	///
	/// # Return
	///
	///  This function returns a status that, new NFT Asset is burnt or not.
	fn burn_nft(owner: &T::AccountId, asset_id: Self::NftId) -> DispatchResult {
		<OwnerAssets<T>>::try_mutate(owner, |vec_of_assets| {
			let index = vec_of_assets.iter().position(|&element| element == asset_id).ok_or(Error::<T>::AssetNotFound)?;
			vec_of_assets.remove(index);
			<AssetOwner<T>>::remove(&asset_id);
			Ok(())
		})
	}
}

/// Helper Functions for Runtime Api.
impl<T: Trait> Module<T>{

	/// This method counts all assets of given Account Id.
	/// # Arguments
	///
	/// * `address` - Account id of Owner.
	///
	/// # Return
	///
	///  This function returns a count of all asset hold by given Account.
	pub fn get_balance_of(account: T::AccountId) -> u64 { Self::balance_of(&account) }

	/// This method finds Owner of given Asset.
    /// # Arguments
    ///
    /// * `asset_id` - Account id of Owner.
    ///
    /// # Return
    ///
    ///  This function returns a Owner of Given Asset.
	pub fn get_owner_of(asset_id: T::Hash) -> T::AccountId{
		Self::owner_of(asset_id)
	}

}
