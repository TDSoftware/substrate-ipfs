use clap::Parser;
use futures::{channel::mpsc, pin_mut, FutureExt};
use libipld::ipld;
use libp2p::{futures::StreamExt, swarm::SwarmEvent};
use rust_ipfs::{BehaviourEvent, Ipfs, IpfsOptions, Protocol, TestTypes, UninitializedIpfs};
use rustyline_async::{Readline, ReadlineError};
use std::{io::Write, sync::Arc};
use tokio::sync::Notify;

#[derive(Debug, Parser)]
#[clap(name = "pubsub")]
struct Opt {
    #[clap(long)]
    disable_bootstrap: bool,
    #[clap(long)]
    disable_mdns: bool,
    #[clap(long)]
    disable_relay: bool,
    #[clap(long)]
    disable_upnp: bool,
    #[clap(long)]
    topic: Option<String>,
    #[clap(long)]
    stdout_log: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opt = Opt::parse();
    if opt.stdout_log {
        tracing_subscriber::fmt::init();
    }

    let topic = opt.topic.unwrap_or_else(|| String::from("ipfs-chat"));

    // Initialize the repo and start a daemon
    let opts = IpfsOptions {
        // Used to discover peers locally
        mdns: !opt.disable_mdns,
        // Used, along with relay [client] for hole punching
        dcutr: !opt.disable_relay,
        // Used to connect to relays
        relay: !opt.disable_relay,
        // used to attempt port forwarding
        port_mapping: !opt.disable_upnp,
        ..Default::default()
    };

    let (tx, mut rx) = mpsc::unbounded();

    let ipfs: Ipfs<TestTypes> = UninitializedIpfs::with_opt(opts)
        .swarm_events({
            move |_, event| {
                if let SwarmEvent::Behaviour(BehaviourEvent::Autonat(
                    libp2p::autonat::Event::StatusChanged { new, .. },
                )) = event
                {
                    match new {
                        libp2p::autonat::NatStatus::Public(_) => {
                            let _ = tx.unbounded_send(true);
                        }
                        libp2p::autonat::NatStatus::Private
                        | libp2p::autonat::NatStatus::Unknown => {
                            let _ = tx.unbounded_send(false);
                        }
                    }
                }
            }
        })
        .start()
        .await?;

    ipfs.default_bootstrap().await?;

    let identity = ipfs.identity(None).await?;
    let peer_id = identity.peer_id;
    let (mut rl, mut stdout) = Readline::new(format!("{peer_id} >"))?;

    if !opt.disable_bootstrap {
        tokio::spawn({
            let ipfs = ipfs.clone();
            async move { if let Err(_e) = ipfs.bootstrap().await {} }
        });
    }

    let cancel = Arc::new(Notify::new());
    if !opt.disable_relay {
        //Until autorelay is implemented and/or functions to use relay more directly, we will manually listen to the relays (using libp2p bootstrap, though you can add your own)
        tokio::spawn({
            let ipfs = ipfs.clone();
            let mut stdout = stdout.clone();
            let cancel = cancel.clone();
            async move {
                let mut listening_addrs = vec![];
                let mut relay_used = false;
                loop {
                    let flag = tokio::select! {
                        flag = rx.next() => {
                            flag.unwrap_or_default()
                        },
                        _ = cancel.notified() => break
                    };

                    match flag {
                        true => {
                            if relay_used {
                                writeln!(stdout, "Disabling Relay...")?;
                                for addr in listening_addrs.drain(..) {
                                    if let Err(_e) = ipfs.remove_listening_address(addr).await {}
                                }
                                relay_used = false;
                            }
                        }
                        false => {
                            if !relay_used {
                                writeln!(stdout, "Enabling Relay...")?;
                                for addr in ipfs.get_bootstraps().await? {
                                    let circuit = addr.with(Protocol::P2pCircuit);
                                    if let Ok(addr) =
                                        ipfs.add_listening_address(circuit.clone()).await
                                    {
                                        listening_addrs.push(addr)
                                    }
                                }
                                relay_used = !listening_addrs.is_empty();
                            }
                        }
                    }
                }

                Ok::<_, anyhow::Error>(())
            }
        });
    }

    let stream = ipfs.pubsub_subscribe(topic.to_string()).await?;
    pin_mut!(stream);

    tokio::spawn(topic_discovery(ipfs.clone(), topic.clone()));

    tokio::task::yield_now().await;

    loop {
        tokio::select! {
            data = stream.next() => {
                if let Some(msg) = data {
                    writeln!(stdout, "{}: {}", msg.source.expect("Message should contain a source peer_id"), String::from_utf8_lossy(&msg.data))?;
                }
            }
            line = rl.readline().fuse() => match line {
                Ok(line) => {
                    if let Err(e) = ipfs.pubsub_publish(topic.clone(), line.as_bytes().to_vec()).await {
                        writeln!(stdout, "Error publishing message: {e}")?;
                        continue;
                    }
                    writeln!(stdout, "{peer_id}: {line}")?;
                }
                Err(ReadlineError::Eof) => {
                    cancel.notify_one();
                    break
                },
                Err(ReadlineError::Interrupted) => {
                    cancel.notify_one();
                    break
                },
                Err(e) => {
                    writeln!(stdout, "Error: {e}")?;
                    writeln!(stdout, "Exiting...")?;
                    break
                },
            }
        }
    }
    // Exit
    ipfs.exit_daemon().await;
    Ok(())
}

//Note: This is temporary as a similar implementation will be used internally in the future
async fn topic_discovery(ipfs: Ipfs<TestTypes>, topic: String) -> anyhow::Result<()> {
    let cid = ipfs.put_dag(ipld!(topic)).await?;
    ipfs.provide(cid).await?;
    loop {
        let mut stream = ipfs.get_providers(cid).await?.boxed();
        while let Some(_providers) = stream.next().await {}
    }
}
