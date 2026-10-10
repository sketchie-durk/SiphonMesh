pub mod protocol;
pub mod network;
pub mod storage;

use libp2p::{
    mdns,
    swarm::{NetworkBehaviour, SwarmEvent},
    PeerId,
};
use futures::StreamExt;
use anyhow::Result;

#[derive(NetworkBehaviour)]
pub struct SiphonBehaviour {
    pub mdns: mdns::async_mdns::Behaviour,
    pub kademlia: libp2p::kad::Behaviour<libp2p::kad::store::MemoryStore>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    let mut swarm = libp2p::SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            libp2p::tcp::tcp::Config::default(),
            libp2p::noise::NoiseConfig::xx(),
            libp2p::yamux::YamuxConfig::default,
        )?
        .with_behaviour(|key| {
            SiphonBehaviour {
                mdns: mdns::async_mdns::Behaviour::new(libp2p::mdns::Config::default(), key.public().to_peer_id()),
                kademlia: libp2p::kad::Behaviour::new(libp2p::kad::KademliaConfig::default(), libp2p::kad::store::MemoryStore::new(key.public().to_peer_id())),
            }
        })?
        .build();

    swarm.listen("/ip4/0.0.0.0/tcp/0")?;
    
    println!("⚡ SiphonMesh Node Operational. Searching for peers...");

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::Behaviour(SiphonEvent::Mdns(mdns::Event::Discovered(list))) => {
                for peer in list {
                    println!("📡 Peer discovered: {}", peer);
                }
            }
            _ => {}
        }
    }
}

#[derive(Debug)]
pub enum SiphonEvent {
    Mdns(mdns::Event),
}

impl From<mdns::Event> for SiphonEvent {
    fn from(event: mdns::Event) -> Self {
        SiphonEvent::Mdns(event)
    }
}
