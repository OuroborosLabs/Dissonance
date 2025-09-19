use libp2p::{mdns::{tokio::Behaviour as MdnsBehaviour, Config as MdnsConfig}};

use crate::NodeIdentity;

pub fn get_mdns(identity: &NodeIdentity)  -> MdnsBehaviour{
    let mdns_config = MdnsConfig::default();
    let mdns_behaviour = MdnsBehaviour::new(mdns_config, identity.peer_id()).unwrap();
    mdns_behaviour
}