// This file is part of Substrate.

// Copyright (C) 2020 Parity Technologies (UK) Ltd.
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

use serde::{Serialize, Deserialize};

use sc_finality_grandpa::FinalityProofProvider;
use sp_runtime::traits::{Block as BlockT, NumberFor};

#[derive(Serialize, Deserialize)]
pub struct EncodedFinalityProofs(pub sp_core::Bytes);

/// Local trait mainly to allow mocking in tests.
pub trait RpcFinalityProofProvider<Block: BlockT> {
	fn rpc_prove_finality(
		&self,
		block: NumberFor<Block>,
	) -> Result<Option<EncodedFinalityProofs>, sp_blockchain::Error>;
}

impl<B, Block> RpcFinalityProofProvider<Block> for FinalityProofProvider<B, Block>
where
	Block: BlockT,
	NumberFor<Block>: finality_grandpa::BlockNumberOps,
	B: sc_client_api::backend::Backend<Block> + Send + Sync + 'static,
{
	fn rpc_prove_finality(
		&self,
		block: NumberFor<Block>,
	) -> Result<Option<EncodedFinalityProofs>, sp_blockchain::Error> {
		self.prove_finality(block)
			.map(|x| x.map(|y| EncodedFinalityProofs(y.into())))
	}
}
