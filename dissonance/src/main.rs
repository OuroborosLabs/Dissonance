use std::error::Error;
use futures::stream::StreamExt;
use libp2p::{
    noise,
    swarm::{NetworkBehaviour, SwarmEvent, dummy},
    tcp, yamux,
};
use tracing_subscriber::EnvFilter;

use dissonance::network;
use dissonance::NodeIdentity;

// Dummy behaviour
#[derive(NetworkBehaviour)]
struct DummyBehaviour {
    dummy: dummy::Behaviour,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();

    println!("Initialising node identity");
    let node_identity = NodeIdentity::get_identity()?;

    let lp2p_keypair = node_identity.to_lp2p_keypair()?;

    let mut swarm = libp2p::SwarmBuilder::with_existing_identity(lp2p_keypair)
        .with_tokio()
        .with_tcp(network::transport::tcp::build_tcp_transport(), noise::Config::new, yamux::Config::default,)?
        .with_behaviour(|_key| {
            Ok(DummyBehaviour {
            dummy: dummy::Behaviour,
             })
             })?
            .build();

    println!("Local peer ID: {}", swarm.local_peer_id());

    assert_eq!(swarm.local_peer_id(), &node_identity.peer_id());

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("Local node is listening on {address}");
                println!("Full address: {address}/p2p/{}", swarm.local_peer_id());
            }
            SwarmEvent::IncomingConnection { local_addr, send_back_addr, .. } => {
                println!("Incoming connection from {send_back_addr} on {local_addr}");
            }
            SwarmEvent::ConnectionEstablished { peer_id, endpoint, .. } => {
                println!("Connected to peer: {peer_id} via {endpoint:?}");
            }
            SwarmEvent::ConnectionClosed { peer_id, cause, .. } => {
                println!("Connection to {peer_id} closed: {cause:?}");
            }
            _ => {
                // Handle other events silently for now
            }
        }
    }
}