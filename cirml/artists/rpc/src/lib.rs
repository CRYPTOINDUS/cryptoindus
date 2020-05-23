use std::sync::Arc;

use codec::Codec;
use jsonrpc_core::{Error, ErrorCode, Result};
use jsonrpc_derive::rpc;

use sp_blockchain::HeaderBackend;
use sp_runtime::{
    generic::BlockId,
    traits::{Block as BlockT, Header as HeaderT},
};

use ci_primitives::ArtistId;

pub struct Artists<C, B> {
    client: Arc<C>,
    _marker: std::marker::PhantomData<B>,
}

impl<C, B> Artists<C, B> {
    /// Create new `Contracts` with the given reference to the client.
    pub fn new(client: Arc<C>) -> Self {
        Artists {
            client,
            _marker: Default::default(),
        }
    }
}

#[rpc]
pub trait ArtistsApi<BlockHash, AccountId> {
    #[rpc(name = "artists")]
    fn artists(&self, at: Option<BlockHash>) -> Result<(ArtistId, AccountId)>;
}

impl<C, Block, AccountId> ArtistsApi<<Block as BlockT>::Hash, AccountId> for Artists<C, Block>
where
    C: sp_api::ProvideRuntimeApi<Block>,
    C: HeaderBackend<Block>,
    C: Send + Sync + 'static,
    C::Api: cirml_artists_runtime_api::ArtistsApi<Block, AccountId>,
    Block: BlockT,
    AccountId: Clone + std::fmt::Display + Codec,
{
    fn artists(&self, at: Option<<Block as BlockT>::Hash>) -> Result<(u32, AccountId)> {
        unimplemented!()
    }
}
