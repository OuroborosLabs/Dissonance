use std::time::Duration;

use libp2p::{kad::{store::MemoryStore as KademliaStore, Behaviour as KademliaBehaviour, Config as KademliaConfig,
    Mode as KademliaMode
}};

use crate::NodeIdentity;

pub fn get_kademlia(identity: &NodeIdentity) -> KademliaBehaviour<KademliaStore>{

    let kad_store = KademliaStore::new(identity.peer_id());
    let mut kad_config = KademliaConfig::default();
    kad_config.set_query_timeout(Duration::from_secs(20));
    kad_config.set_replication_factor(20.try_into().unwrap());
    kad_config.set_max_packet_size(16*1024);

    let mut kademlia = KademliaBehaviour::with_config(identity.peer_id, kad_store, kad_config);
    kademlia.set_mode(Some(KademliaMode::Server));

    kademlia
}