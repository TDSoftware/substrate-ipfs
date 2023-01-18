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


pub mod ipfs_api {
    use futures::{Future, FutureExt};
    use rust_ipfs::{Ipfs, IpfsOptions, TestTypes, UninitializedIpfs, PublicKey, p2p::PeerInfo};
    use tracing::log;
	use tracing_subscriber;

	pub async fn start_node() -> anyhow::Result<()> {
		log::info!("IPFS_API start_node");

		tracing_subscriber::fmt::init();

		// // Initialize the repo and start a daemon
		let opts = rust_ipfs::IpfsOptions::default();
		let ipfs: rust_ipfs::Ipfs<TestTypes> = UninitializedIpfs::new(opts).spawn_start().await?;

		//let ipfs_future = UninitializedIpfs::<TestTypes>::new(opts).spawn_start();


		tokio::time::sleep(std::time::Duration::from_secs(1)).await;

		let PeerInfo { public_key: key, listen_addrs: addresses, ..} = ipfs.identity(None).await?;

		if let PublicKey::Ed25519(publickey) = &key {
			log::info!("Public Key: {:?}", publickey.encode());
		}

		println!("PeerID: {}", key.to_peer_id());

		for address in addresses {
			log::info!("Listening on: {}", address);
		}

		ipfs.exit_daemon().await;

		Ok(())
	}
}
