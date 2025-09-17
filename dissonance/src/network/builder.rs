use libp2p::swarm::{dummy, NetworkBehaviour, Swarm};

use crate::network::transport::{
    noise::build_noise_config,
    tcp::build_tcp_config,
    yamux::build_yamux_config
};

use super::NodeIdentity;
#[derive(NetworkBehaviour)]
pub struct DummyBehaviour {
    dummy: dummy::Behaviour,
}

pub fn build_swarm(identity: &NodeIdentity) -> anyhow::Result<Swarm<DummyBehaviour>>{

    let lp2p_keypair = identity.to_lp2p_keypair()?;
    let swarm = libp2p::SwarmBuilder::with_existing_identity(lp2p_keypair)
    .with_tokio()
    .with_tcp(build_tcp_config(), build_noise_config, build_yamux_config,)?
    .with_behaviour(|_key| {
        Ok(DummyBehaviour {
        dummy: dummy::Behaviour,
         })
         })?
        .build();

    Ok(swarm)
}