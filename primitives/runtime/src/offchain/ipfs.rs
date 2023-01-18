// This file is part of Substrate.

// Copyright (C) 2017-2022 Parity Technologies (UK) Ltd.
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

pub mod ipfs_core {

	pub fn test() {
		log::info!("IPFS_API");
		sp_io::offchain::ipfs_start_node();

	}

	pub async fn test_two() {

	}

	pub struct StartNodeRequest {

	}

	impl StartNodeRequest {
		/// Creates amd starts a specified request for the IPFS node.
		pub fn new() {
			let id =
				sp_io::offchain::ipfs_start_node();

			// Ok(PendingRequest { id, request })
		}
	}
}


