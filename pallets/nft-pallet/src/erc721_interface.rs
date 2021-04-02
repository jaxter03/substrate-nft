use frame_support::dispatch::DispatchResult;

pub trait Erc721<AccountId> {

    type NftId;

    /// This method counts all assets of given Account Id.
	/// # Arguments
	///
	/// * `address` - Account id of Owner.
	///
	/// # Return
	///
	///  This function returns a count of all asset hold by given Account.
    fn balance_of(address: &AccountId) -> u64;

    /// This method finds Owner of given Asset.
	/// # Arguments
	///
	/// * `asset_id` - Account id of Owner.
	///
	/// # Return
	///
	///  This function returns a Owner of Given Asset.
    fn owner_of(asset: Self::NftId) -> AccountId;

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
    fn transfer_from(from: &AccountId, to: &AccountId, asset: Self::NftId) -> DispatchResult;

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
    fn mint_nft(owner: &AccountId, asset_id: Self::NftId) -> DispatchResult;

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
    fn burn_nft(owner: &AccountId, asset_id: Self::NftId) -> DispatchResult;

}