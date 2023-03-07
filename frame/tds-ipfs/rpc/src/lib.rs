pub use pallet_tds_ipfs_runtime_api::TDSIpfsApi as TDSIpfsRuntimeApi;

use jsonrpsee::{
	core::{Error as JsonRpseeError, RpcResult},
	proc_macros::rpc,
	types::error::{CallError, ErrorObject},
};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use std::sync::Arc;

#[rpc(client, server)]
pub trait TDSIpfsApi<BlockHash> {
	#[method(name = "ipfs_getFileURL")]
	fn get_value(&self, at: Option<BlockHash>) -> RpcResult<u32>;
}

/// A struct that implements the `TemplateApi`.
pub struct TDSIpfsPallet<C, Block> {
	// If you have more generics, no need to TemplatePallet<C, M, N, P, ...>
	// just use a tuple like TemplatePallet<C, (M, N, P, ...)>
	client: Arc<C>,
	_marker: std::marker::PhantomData<Block>,
}

impl<C, Block> TDSIpfsPallet<C, Block> {
	/// Create new `TemplatePallet` instance with the given reference to the client.
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

impl<C, Block> TDSIpfsApiServer<<Block as BlockT>::Hash> for TDSIpfsPallet<C, Block>
	where
		Block: BlockT,
		C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
		C::Api: TDSIpfsRuntimeApi<Block>,
{
	fn get_value(&self, at: Option<<Block as BlockT>::Hash>) -> RpcResult<u32> {
		// let api = self.client.runtime_api();
		// let at = BlockId::hash(at.unwrap_or_else(||self.client.info().best_hash));
		//
		// api.get_value(&at).map_err(runtime_error_into_rpc_err)
		Ok(666) // TODO: Implement
	}
}

const RUNTIME_ERROR: i32 = 1;

/// Converts a runtime trap into an RPC error.
fn runtime_error_into_rpc_err(err: impl std::fmt::Debug) -> JsonRpseeError {
	CallError::Custom(ErrorObject::owned(
		RUNTIME_ERROR,
		"Runtime error",
		Some(format!("{:?}", err)),
	)).into()
}
