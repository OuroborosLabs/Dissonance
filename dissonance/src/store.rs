use std::{collections::HashMap, time::{Duration, SystemTime}};

use libp2p::{identify::Info, Multiaddr, PeerId, StreamProtocol};

#[derive(Debug)]
pub struct PeerInfo{
    pub last_seen: SystemTime,
    pub addresses: Vec<Multiaddr>,
    pub agent_version: Option<String>,
    pub protocols: Vec<StreamProtocol>,
    is_trusted: bool
}

impl PeerInfo{
    pub fn new() -> Self{
        PeerInfo { last_seen: SystemTime::now(), addresses: vec![], agent_version: None, protocols: vec![], is_trusted: false }
    }

    pub fn seen(&mut self){
        self.last_seen = SystemTime::now();
    }

    pub fn add_address(&mut self, address: Multiaddr){
        if !self.addresses.contains(&address){
            self.addresses.push(address);
        }
        self.seen();
    }
    pub fn add_identity(&mut self, info: Info){
        self.agent_version = Some(info.agent_version);
        self.protocols = info.protocols.clone();
        self.seen();
    }
}


#[derive(Debug, Default)]
pub struct PeerStore{
    known_peers: HashMap<PeerId, PeerInfo>,
}

impl PeerStore {
    pub fn new() -> Self{
        PeerStore { known_peers: HashMap::new() }
    }

    pub fn get_or_create(&mut self, peer_id: &PeerId) -> &mut PeerInfo{
        self.known_peers.entry(peer_id.clone()).or_insert_with(PeerInfo::new)
    }

    pub fn add_peer_address(&mut self, peer_id: &PeerId, address: Multiaddr){
        let peer_info = self.get_or_create(peer_id);
        peer_info.add_address(address);
    }

    pub fn add_peer_identity(&mut self, peer_id: &PeerId, info:Info){
        let peer_info = self.get_or_create(peer_id);
        peer_info.add_identity(info);
    }

    pub fn is_peer_trusted(&mut self, peer_id: &PeerId) -> bool{
        let peer_info = self.get_or_create(peer_id);
        peer_info.is_trusted
    }

    pub fn list_peers(&mut self) -> Vec<(&PeerId, &PeerInfo)>{
        self.known_peers.iter().collect()
    }

    pub fn insert_peer_info(&mut self, peer_id: PeerId, info: PeerInfo) {
        self.known_peers.insert(peer_id, info);
    }

    pub fn prune_stale(&mut self, max_age: Duration){
        let now = SystemTime::now();
        self.known_peers.retain(|_, info|{
            now.duration_since(info.last_seen).map(|age| age<max_age).unwrap_or(false)
        });
    }
}