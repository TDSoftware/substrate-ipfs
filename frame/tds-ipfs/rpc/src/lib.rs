pub use pallet_tds_ipfs_runtime_api::TDSIpfsApi as TDSIpfsRuntimeApi;

use jsonrpsee::{
	core::{Error as JsonRpseeError, RpcResult},
	proc_macros::rpc,
	types::error::{CallError, ErrorObject},
};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, sp_std, traits::Block as BlockT};
use std::sync::Arc;

#[rpc(client, server)]
pub trait TDSIpfsApi<BlockHash> {
	#[method(name = "ipfs_getFileURL")]
	fn get_file_url_for_cid(&self, cid: &str, at: Option<BlockHash>) -> RpcResult<String>;
}

impl<C, Block> TDSIpfsApiServer<<Block as BlockT>::Hash> for TDSIpfsPallet<C, Block>
	where
		Block: BlockT,
		C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
		C::Api: TDSIpfsRuntimeApi<Block>,
{
	fn get_file_url_for_cid(&self, cid: &str, at: Option<<Block as BlockT>::Hash>) -> RpcResult<String> {
		let api = self.client.runtime_api();
		let cid_bytes = cid.as_bytes();
		let cid_vec = sp_std::vec::Vec::from(cid_bytes);

		let at = BlockId::hash(at.unwrap_or_else(||self.client.info().best_hash));
		let result = api.get_file_url_for_cid(&at,
											  cid_vec);

		match result {
			Ok(cid_address_raw) => {
				let cid_address = String::from_utf8(cid_address_raw).unwrap();
				RpcResult::Ok(cid_address)
			}
			Err(api_error) => {
				let error = runtime_error_into_rpc_err(api_error);
				RpcResult::Err(error)
			}
		}
	}
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

const RUNTIME_ERROR: i32 = 1;

/// Converts a runtime trap into an RPC error.
fn runtime_error_into_rpc_err(err: impl std::fmt::Debug) -> JsonRpseeError {
	CallError::Custom(ErrorObject::owned(
		RUNTIME_ERROR,
		"Runtime error",
		Some(format!("{:?}", err)),
	)).into()
}
