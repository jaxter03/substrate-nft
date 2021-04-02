#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::unnecessary_mut_passed)]

use sp_arithmetic::FixedU128;
use sp_core::H256;
use sp_std::vec::Vec;
use sp_core::H256;

sp_api::decl_runtime_apis!{
    pub trait NftStorageApi {

        pub fn get_balance_of (account: [u8; 32]) -> u64;

        pub fn get_owner_of (asset_id: H256) -> [u8; 32];

    }
}
