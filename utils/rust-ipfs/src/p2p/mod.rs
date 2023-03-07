//! P2P handling for IPFS nodes.
mod stream;

use std::convert::TryInto;
use std::num::{NonZeroU8, NonZeroUsize};

use crate::error::Error;
use crate::IpfsOptions;

use libp2p::identify::Info as IdentifyInfo;
use libp2p::identity::{Keypair, PublicKey};
use libp2p::kad::KademliaConfig;
use libp2p::ping::Config as PingConfig;
use libp2p::swarm::ConnectionLimits;
use libp2p::Swarm;
use libp2p::{Multiaddr, PeerId};
use tracing::Span;

pub(crate) mod addr;
mod behaviour;
pub use self::behaviour::BehaviourEvent;
pub use self::behaviour::IdentifyConfiguration;
pub use self::behaviour::KadStoreConfig;
pub use self::behaviour::{RateLimit, RelayConfig};
pub use self::stream::ProviderStream;
pub use self::stream::RecordStream;
pub use self::transport::TransportConfig;
pub(crate) mod gossipsub;
mod swarm;
mod transport;

pub use addr::{MultiaddrWithPeerId, MultiaddrWithoutPeerId};
pub use {behaviour::KadResult, swarm::Connection};

/// Type alias for [`libp2p::Swarm`] running the [`behaviour::Behaviour`] with the given [`IpfsTypes`].
pub type TSwarm = Swarm<behaviour::Behaviour>;

/// Abstraction of IdentifyInfo but includes PeerId
#[derive(Clone, Debug, Eq)]
pub struct PeerInfo {
    /// The peer id of the user
    pub peer_id: PeerId,

    /// The public key of the local peer.
    pub public_key: PublicKey,

    /// Application-specific version of the protocol family used by the peer,
    /// e.g. `ipfs/1.0.0` or `polkadot/1.0.0`.
    pub protocol_version: String,

    /// Name and version of the peer, similar to the `User-Agent` header in
    /// the HTTP protocol.
    pub agent_version: String,

    /// The addresses that the peer is listening on.
    pub listen_addrs: Vec<Multiaddr>,

    /// The list of protocols supported by the peer, e.g. `/ipfs/ping/1.0.0`.
    pub protocols: Vec<String>,

    /// Address observed by or for the remote.
    pub observed_addr: Option<Multiaddr>,
}

impl core::hash::Hash for PeerInfo {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.peer_id.hash(state);
        self.public_key.hash(state);
    }
}

impl PartialEq for PeerInfo {
    fn eq(&self, other: &Self) -> bool {
        self.peer_id == other.peer_id && self.public_key == other.public_key
    }
}

impl From<IdentifyInfo> for PeerInfo {
    fn from(info: IdentifyInfo) -> Self {
        let IdentifyInfo {
            public_key,
            protocol_version,
            agent_version,
            listen_addrs,
            protocols,
            observed_addr,
        } = info;
        let peer_id = public_key.clone().into();
        let observed_addr = Some(observed_addr);
        Self {
            peer_id,
            public_key,
            protocol_version,
            agent_version,
            listen_addrs,
            protocols,
            observed_addr,
        }
    }
}

/// Defines the configuration for an IPFS swarm.
pub struct SwarmOptions {
    /// The keypair for the PKI based identity of the local node.
    pub keypair: Keypair,
    /// The peer address of the local node created from the keypair.
    pub peer_id: PeerId,
    /// The peers to connect to on startup.
    pub bootstrap: Vec<Multiaddr>,
    /// Enables mdns for peer discovery and announcement when true.
    pub mdns: bool,
    /// enables ipv6 for mdns
    pub mdns_ipv6: bool,
    /// Relay Server
    pub relay_server: bool,
    /// Relay Server Configuration
    pub relay_server_config: Option<RelayConfig>,
    /// Kademlia Configuration
    pub kad_config: Option<KademliaConfig>,
    /// Ping Configuration
    pub ping_config: Option<PingConfig>,
    /// identify configuration
    pub identify_config: Option<IdentifyConfiguration>,
    /// Kad store config
    /// Note: Only supports MemoryStoreConfig at this time
    pub kad_store_config: Option<KadStoreConfig>,
    /// UPnP/PortMapping
    pub portmapping: bool,
    /// Keep alive
    pub keep_alive: bool,
    /// Relay client
    pub relay: bool,
    /// Enables dcutr
    pub dcutr: bool,
}

impl From<&IpfsOptions> for SwarmOptions {
    fn from(options: &IpfsOptions) -> Self {
        let keypair = options.keypair.clone();
        let peer_id = keypair.public().to_peer_id();
        let bootstrap = options.bootstrap.clone();
        let mdns = options.mdns;
        let mdns_ipv6 = options.mdns_ipv6;
        let dcutr = options.dcutr;
        let relay_server = options.relay_server;
        let relay_server_config = options.relay_server_config.clone();
        let relay = options.relay;
        let kad_config = options.kad_configuration.clone();
        let ping_config = options.ping_configuration.clone();
        let kad_store_config = options.kad_store_config.clone();

        let keep_alive = options.keep_alive;
        let identify_config = options.identify_configuration.clone();
        let portmapping = options.port_mapping;

        SwarmOptions {
            keypair,
            peer_id,
            bootstrap,
            mdns,
            mdns_ipv6,
            relay_server,
            relay_server_config,
            relay,
            dcutr,
            kad_config,
            kad_store_config,
            ping_config,
            keep_alive,
            identify_config,
            portmapping,
        }
    }
}

#[derive(Clone)]
pub struct SwarmConfig {
    pub connection: ConnectionLimits,
    pub dial_concurrency_factor: NonZeroU8,
    pub notify_handler_buffer_size: NonZeroUsize,
    pub connection_event_buffer_size: usize,
    pub max_inbound_stream: usize,
}

impl Default for SwarmConfig {
    fn default() -> Self {
        Self {
            connection: ConnectionLimits::default(),
            dial_concurrency_factor: 8.try_into().expect("8 > 0"),
            notify_handler_buffer_size: 256.try_into().expect("256 > 0"),
            connection_event_buffer_size: 256,
            max_inbound_stream: 128,
        }
    }
}

/// Creates a new IPFS swarm.
pub async fn create_swarm(
    options: SwarmOptions,
    swarm_config: SwarmConfig,
    transport_config: TransportConfig,
    span: Span,
) -> Result<TSwarm, Error> {
    let peer_id = options.peer_id;

    let keypair = options.keypair.clone();

    // Create a Kademlia behaviour
    let (behaviour, relay_transport) = behaviour::build_behaviour(options).await?;

    // Set up an encrypted TCP transport over the Yamux and Mplex protocol. If relay transport is supplied, that will be apart
    let transport = transport::build_transport(keypair, relay_transport, transport_config)?;

    // Create a Swarm
    let swarm = libp2p::swarm::SwarmBuilder::with_executor(
        transport,
        behaviour,
        peer_id,
        SpannedExecutor(span),
    )
    .connection_limits(swarm_config.connection)
    .notify_handler_buffer_size(swarm_config.notify_handler_buffer_size)
    .connection_event_buffer_size(swarm_config.connection_event_buffer_size)
    .dial_concurrency_factor(swarm_config.dial_concurrency_factor)
    .max_negotiating_inbound_streams(swarm_config.max_inbound_stream)
    .build();

    Ok(swarm)
}

struct SpannedExecutor(Span);

impl libp2p::swarm::Executor for SpannedExecutor {
    fn exec(
        &self,
        future: std::pin::Pin<Box<dyn std::future::Future<Output = ()> + 'static + Send>>,
    ) {
        use tracing_futures::Instrument;
        tokio::task::spawn(future.instrument(self.0.clone()));
    }
}
