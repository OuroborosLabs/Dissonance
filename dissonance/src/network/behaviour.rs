use libp2p::{swarm::{NetworkBehaviour}};

use crate::network::behaviours::{identify::create_identify, kademlia::get_kademlia, mdns::get_mdns};
use super::NodeIdentity;
use libp2p::{{kad::{Event as KademliaEvent, Behaviour as KademliaBehaviour, store::MemoryStore}}, identify::{Behaviour as IdentifyBehaviour, Event as IdentifyEvent}, mdns::{tokio::Behaviour as MdnsBehaviour, Event as MdnsEvent}};

#[derive(NetworkBehaviour)]
#[behaviour(to_swarm="DissonanceEvent")]
pub struct DissonanceBehaviour {
    kademlia: KademliaBehaviour<MemoryStore>,
    identify: IdentifyBehaviour,
    mdns: MdnsBehaviour
}

impl DissonanceBehaviour {
    pub fn new(identity: &NodeIdentity) -> Self{
        DissonanceBehaviour { kademlia: get_kademlia(identity), identify: create_identify(identity), mdns: get_mdns(identity) }
    }

    pub fn add_kademlia_address(&mut self, peer:&libp2p::PeerId, addr: libp2p::Multiaddr){
        self.kademlia.add_address(peer, addr);
    }

    pub fn bootstrap_kad(&mut self) -> Result<libp2p::kad::QueryId, libp2p::kad::store::Error>{
        // self.kademlia.bootstrap()
        todo!()
    }
}

pub enum DissonanceEvent {
    Kademlia(KademliaEvent),
    Identify(IdentifyEvent),
    Mdns(MdnsEvent)
}

impl From<KademliaEvent> for DissonanceEvent {
    fn from(value: KademliaEvent) -> Self {
        DissonanceEvent::Kademlia(value)
    }    
}

impl From<IdentifyEvent> for DissonanceEvent {
    fn from(value: IdentifyEvent) -> Self {
        DissonanceEvent::Identify(value)
    }
}

impl From<MdnsEvent> for DissonanceEvent {    
    fn from(value: MdnsEvent) -> Self {
        DissonanceEvent::Mdns(value)
    }
}

