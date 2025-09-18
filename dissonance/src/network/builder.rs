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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NodeIdentity;

    #[test]
    fn test_dummy_behaviour_implements_network_behaviour() {
        fn is_network_behaviour<T: libp2p::swarm::NetworkBehaviour>() {}
        is_network_behaviour::<DummyBehaviour>();
    }

    #[test]
    fn test_build_swarm_success() {
        let identity = NodeIdentity::get_identity().expect("Could not generate identity");
        let swarm_result = build_swarm(&identity);
        assert!(swarm_result.is_ok(), "Failed to build swarm");

        let swarm = swarm_result.unwrap();
        let local_peer_id = *swarm.local_peer_id();
        assert_eq!(local_peer_id, identity.peer_id, "Mismatch in peer id");
    }

    #[test]
    fn test_swarm_has_dummy_behaviour() {
        let identity = NodeIdentity::get_identity().unwrap();
        let swarm = build_swarm(&identity).unwrap();

        let behaviour_any = swarm.behaviour();
        let behaviour: &DummyBehaviour = behaviour_any;
        let _ = &behaviour.dummy;
    }
}
