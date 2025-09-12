use std::error::Error;
use futures::stream::StreamExt;
use libp2p::{
    noise,
    swarm::{NetworkBehaviour, SwarmEvent, dummy},
    tcp, yamux,
};
use tracing_subscriber::EnvFilter;

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

    let mut swarm = libp2p::SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_behaviour(|_key| {
            Ok(DummyBehaviour {
                dummy: dummy::Behaviour,
            })
        })?
        .build();

    // Log the peer ID
    println!("Local peer ID: {}", swarm.local_peer_id());

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("Local node is listening on {address}");
            }
            _ => {}
        }
    }
}