
use std::sync::Arc;

use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;
use sp_api::ProvideRuntimeApi;
use sp_arithmetic::FixedU128;
use sp_blockchain::HeaderBackend;
use sp_core::H256;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use sp_std::vec::Vec;

#[rpc]
pub trait NftStorageApi<BlockHash> {
    #[rpc(name = "get_balance")]
    fn get_balance_of(&self, at: Option<BlockHash>, account: [u8; 32]) -> Result<u64>;

    #[rpc(name = "get_owner")]
    fn get_owner_of(&self, at: Option<BlockHash>, asset_id: H256) -> Result<[u8; 32]>;
}

pub struct NftStorage<C, M> {
    client: Arc<C>,
    _marker: std::marker::PhantomData<M>,
}

impl<C, M> NftStorage<C, M> {
    /// Create new `DexStorage` instance with the given reference to the client.
    pub fn new(client: Arc<C>) -> Self {
        Self {
            client,
            _marker: Default::default(),
        }
    }
}

impl<C, Block> NftStorageApi<<Block as BlockT>::Hash> for NftStorage<C, Block>
    where
        Block: BlockT,
        C: ProvideRuntimeApi<Block>,
        C: HeaderBackend<Block>,
        C::Api: NftStorageApi<Block>,
{
    fn get_balance_of(&self, _at: Option<<Block as BlockT>::Hash>, account: [u8; 32]) -> Result<u64> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(
            self.client.info().best_hash);
        let runtime_api_result = api.get_balance_of(&at, trading_pair);
        let temp = match runtime_api_result {
            Ok(x) => match x {
                Ok(z) => Ok(z),
                Err(x) => Err(x),
            }
            Err(_) => Err(ErrorRpc::ServerErrorWhileCallingAPI), // change
        };
        temp.map_err(|e| ErrorConvert::covert_to_rpc_error(e))
    }

    fn get_owner_of(&self, at: Option<<Block as BlockT>::Hash>, asset_id: H256) -> Result<[u8; 32]> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(
            self.client.info().best_hash);
        let runtime_api_result = api.get_owner_of(&at, asset_id);
        let temp = match runtime_api_result {
            Ok(x) => match x {
                Ok(z) => Ok(z),
                Err(x) => Err(x),
            }
            Err(_) => Err(ErrorRpc::ServerErrorWhileCallingAPI),
        };
        temp.map_err(|e| ErrorConvert::covert_to_rpc_error(e))
    }
}
