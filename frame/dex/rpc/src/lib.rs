// This file is part of Substrate.

// Copyright (C) 2019-2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! RPC interface for the transaction payment pallet.

use std::{convert::TryInto, marker::PhantomData, sync::Arc};

use codec::{Codec, Decode};
use jsonrpsee::{
	core::{async_trait, Error as JsonRpseeError, RpcResult},
	proc_macros::rpc,
	types::error::{CallError, ErrorCode, ErrorObject},
};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_core::Bytes;
use sp_rpc::number::NumberOrHex;
use sp_runtime::{
	generic::BlockId,
	traits::{Block as BlockT, MaybeDisplay},
};

pub use pallet_dex_rpc_runtime_api::DexApi as DexRuntimeApi;

#[rpc(client, server)]
pub trait DexApi<AssetId, Balance>
where
	Balance: Copy + TryFrom<NumberOrHex> + Into<NumberOrHex>,
{
	#[method(name = "dex_pairPrice")]
	fn pair_price(&self, asset1: AssetId, asset2: AssetId) -> RpcResult<Option<Balance>>;
}

/// Dex RPC methods.
pub struct Dex<Client, Block> {
	client: Arc<Client>,
	_marker: PhantomData<Block>,
}

impl<Client, Block> Dex<Client, Block> {
	/// Creates a new instance of the DEX RPC helper.
	pub fn new(client: Arc<Client>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

/// Error type of this RPC api.
pub enum Error {
	/// The call to runtime failed.
	RuntimeError,
}

impl From<Error> for i32 {
	fn from(e: Error) -> i32 {
		match e {
			Error::RuntimeError => 1,
		}
	}
}

#[async_trait]
impl<Client, Block, AssetId, Balance> DexApiServer<<Block as BlockT>::Hash, AssetId, Balance>
	for Dex<Client, Block>
where
	Block: BlockT,
	Client: ProvideRuntimeApi<Block> + HeaderBackend<Block> + Send + Sync + 'static,
	Client::Api: DexRuntimeApi<Block, AssetId, Balance>,
	AssetId: Codec,
	Balance: Codec + MaybeDisplay + Copy + TryFrom<NumberOrHex> + TryInto<NumberOrHex>,
{
	fn quote_price(&self, asset1: AssetId, asset2: AssetId) -> RpcResult<Option<Balance>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

		api.quote_price(&at, asset1, asset2).map_err(|e| {
			CallError::Custom(ErrorObject::owned(
				Error::RuntimeError.into(),
				"Unable to query price info.",
				Some(e.to_string()),
			))
			.into()
		})
	}
}
