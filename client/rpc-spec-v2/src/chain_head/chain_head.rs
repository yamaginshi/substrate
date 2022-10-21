// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! API implementation for `chainHead`.

use crate::{
	chain_head::{api::ChainHeadApiServer, subscription::SubscriptionManagement},
	SubscriptionTaskExecutor,
};
use jsonrpsee::{
	core::{async_trait, RpcResult},
	types::SubscriptionResult,
	SubscriptionSink,
};
use sc_client_api::{
	Backend, BlockBackend, BlockchainEvents, ExecutorProvider, StorageKey, StorageProvider,
};
use sp_api::CallApiAt;
use sp_blockchain::HeaderBackend;
use sp_core::Bytes;
use sp_runtime::traits::{Block as BlockT, Header};
use std::{marker::PhantomData, sync::Arc};

/// An API for chain head RPC calls.
pub struct ChainHead<BE, Block: BlockT, Client> {
	/// Substrate client.
	client: Arc<Client>,
	/// Executor to spawn subscriptions.
	executor: SubscriptionTaskExecutor,
	/// Keep track of the pinned blocks for each subscription.
	subscriptions: Arc<SubscriptionManagement<Block>>,
	/// Phantom member to pin the block type.
	_phantom: PhantomData<(Block, BE)>,
}

impl<BE, Block: BlockT, Client> ChainHead<BE, Block, Client> {
	/// Create a new [`ChainHead`].
	pub fn new(client: Arc<Client>, executor: SubscriptionTaskExecutor) -> Self {
		Self {
			client,
			executor,
			subscriptions: Arc::new(SubscriptionManagement::new()),
			_phantom: PhantomData,
		}
	}
}

#[async_trait]
impl<BE, Block, Client> ChainHeadApiServer<Block::Hash> for ChainHead<BE, Block, Client>
where
	Block: BlockT + 'static,
	Block::Header: Unpin,
	BE: Backend<Block> + 'static,
	Client: BlockBackend<Block>
		+ ExecutorProvider<Block>
		+ HeaderBackend<Block>
		+ BlockchainEvents<Block>
		+ CallApiAt<Block>
		+ StorageProvider<Block, BE>
		+ 'static,
{
	fn chain_head_unstable_follow(
		&self,
		mut _sink: SubscriptionSink,
		_runtime_updates: bool,
	) -> SubscriptionResult {
		Ok(())
	}

	fn chain_head_unstable_body(
		&self,
		mut _sink: SubscriptionSink,
		_follow_subscription: String,
		_hash: Block::Hash,
		_network_config: Option<()>,
	) -> SubscriptionResult {
		Ok(())
	}

	fn chain_head_unstable_header(
		&self,
		_follow_subscription: String,
		_hash: Block::Hash,
	) -> RpcResult<Option<String>> {
		Ok(None)
	}

	fn chain_head_unstable_storage(
		&self,
		mut _sink: SubscriptionSink,
		_follow_subscription: String,
		_hash: Block::Hash,
		_key: StorageKey,
		_child_key: Option<StorageKey>,
		_network_config: Option<()>,
	) -> SubscriptionResult {
		Ok(())
	}

	fn chain_head_unstable_call(
		&self,
		mut _sink: SubscriptionSink,
		_follow_subscription: String,
		_hash: Block::Hash,
		_function: String,
		_call_parameters: Bytes,
		_network_config: Option<()>,
	) -> SubscriptionResult {
		Ok(())
	}

	fn chain_head_unstable_unpin(
		&self,
		_follow_subscription: String,
		_hash: Block::Hash,
	) -> RpcResult<()> {
		Ok(())
	}
}
