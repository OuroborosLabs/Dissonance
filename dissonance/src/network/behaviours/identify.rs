use std::time::Duration;

use libp2p::identify::{Behaviour as IdentifyBehaviour, Config as IdentifyConfig};

use crate::NodeIdentity;

pub fn create_identify(identity: &NodeIdentity) -> IdentifyBehaviour{

    let keypair = identity.to_lp2p_keypair().unwrap();
    let identify_config = IdentifyConfig::new("/basic-p2p/1.0.0".to_string(), keypair.public())
    .with_agent_version("basic-p2p-node/0.1.0".to_string())
    .with_push_listen_addr_updates(true)
    .with_interval(Duration::from_secs(30));

    let identify = IdentifyBehaviour::new(identify_config);
    identify
}