use std::error::Error;
use futures::stream::StreamExt;
use libp2p::{
    swarm::SwarmEvent,
};
use tracing_subscriber::EnvFilter;

use dissonance::network::builder::{build_swarm, DissonanceEvent};
use dissonance::NodeIdentity;

use libp2p::kad::Event as KademliaEvent;
// Dummy behaviour

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();

    println!("Initialising node identity");
    let node_identity = NodeIdentity::get_identity()?;

    let mut swarm = build_swarm(&node_identity)?;
    println!("Local peer ID: {}", swarm.local_peer_id());

    assert_eq!(swarm.local_peer_id(), &node_identity.peer_id());

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    loop {
        match swarm.select_next_some().await {

            SwarmEvent::Behaviour(DissonanceEvent::Kademlia(event)) => match event {
                KademliaEvent::RoutingUpdated{peer,is_new_peer,addresses,bucket_range,old_peer}=>{
                    println!("[KAD] Routing table updated with the following peer details: {}",peer);
                    // FUTURE: 
                    // - If `is_new_peer`, persist this peer in your local disk-backed store 
                    //   so the node remembers it after restart (important for bootstrap performance).
                    // - Use `addresses` to update your local peer-address book (with timestamp).
                    // - Could check peer reputation/behavior and decide whether to keep it in the routing table.
                    // - If `old_peer` is Some, remove/replace its state in the peer store.
                },
                KademliaEvent::InboundRequest{request}=>{
                    println!("[KAD] Inbound request on DHT");
                    // FUTURE:
                    // - Handle `GetRecord` or `PutRecord` requests.
                    // - You might filter what keys you allow others to store (anti-spam / DoS protection).
                    // - Optionally encrypt data stored on DHT if privacy is a concern (e.g. store ciphertext only).
                    // - Consider rate limiting or proof-of-work for writes to mitigate Sybil spam.
                    },
                KademliaEvent::OutboundQueryProgressed{id,result,stats,step}=>{
                    println!("[KAD] Query {} progressed {:?}",id,result);
                    // FUTURE:
                    // - Use `result` to know whether a peer lookup or record lookup was successful.
                    // - If this was a bootstrap query, check `stats` to decide whether to launch more queries.
                    // - If looking up a peer for message delivery, this is where you connect/send message.
                    // - Optionally log query performance to tune parallelism or timeouts.
                },
                KademliaEvent::UnroutablePeer { peer } => {
                    println!("[KAD] Unroutable peer detected: {}", peer);
                    // FUTURE: Could log metrics or attempt to refresh this peer's record.
                    // Maybe schedule a re-bootstrap or remove it from the routing table if repeated.
                },
                KademliaEvent::RoutablePeer { peer, address } => {
                    println!("[KAD] Routable peer {} detected with address {:?}", peer, address);
                    // FUTURE: This is a good place to store peer information in a local peer store.
                    // Can also trigger any queued messages for this peer since it's reachable now.
                },
                KademliaEvent::PendingRoutablePeer { peer, address } => {
                    println!("[KAD] Pending routable peer {} with address {:?}", peer, address);
                    // FUTURE: This is when the peer is found but not yet fully confirmed.
                    // You could attempt a direct connection here, or verify Noise handshake before trusting it.
                },
                KademliaEvent::ModeChanged { new_mode } => {
                    println!("[KAD] mode changed to {:?}", new_mode);
                    // FUTURE: Mode can be client or server. 
                    // If switched to client mode (e.g. behind NAT), maybe trigger bootstrap more often.
                    // If switched to server mode, you might allow other peers to store records on this node.
                },
            },
            
            SwarmEvent::Behaviour(DissonanceEvent::Identify(event)) => match event{
                libp2p::identify::Event::Received { connection_id, peer_id, info } => {
                    println!("[IDENTIFY] Received identity info from peer: {} on connection {:?}", peer_id, connection_id);
                    // FUTURE:
                    // - Store peer's `info` (agent version, supported protocols, listen addresses)
                    //   in your local peer database to help future connections.
                    // - Verify the info (e.g., supported protocols match what you expect).
                    // - Could enforce minimum supported protocol versions here (disconnect otherwise).
                    // - Might use peer's public key for TOFU (Trust On First Use) logic.
                },
                libp2p::identify::Event::Sent { connection_id, peer_id } => {
                    println!("[IDENTIFY] Sent our identity info to peer: {} on connection {:?}", peer_id, connection_id);
                    // FUTURE:
                    // - Log which peers you have identified to — could track handshake success rate.
                    // - This is useful to know when you can safely send encrypted messages to this peer.
                },
                libp2p::identify::Event::Pushed { connection_id, peer_id, info } => {
                    println!("[IDENTIFY] Received unsolicited identity push from peer: {} on connection {:?}", peer_id, connection_id);
                    // FUTURE:
                    // - Treat this as an update: refresh your stored info about this peer.
                    // - Use this to detect network changes (peer changed IP, protocol version, etc.).
                    // - If `info` looks suspicious (e.g., protocol downgrade attack), trigger security alert.
                },
                libp2p::identify::Event::Error { connection_id, peer_id, error } => {
                    println!("[IDENTIFY] Error with peer {} on connection {:?}: {:?}", peer_id, connection_id, error);
                    // FUTURE:
                    // - Log or count errors for peer reputation system (e.g., disconnect on repeated failures).
                    // - You may want to retry identification after a delay.
                    // - Could trigger peer ban if error indicates malicious behaviour.
                },
            }
                        
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("Local node is listening on {address}");
                println!("Full address: {address}/p2p/{}", swarm.local_peer_id());
            },
            SwarmEvent::IncomingConnection { local_addr, send_back_addr, .. } => {
                println!("Incoming connection from {send_back_addr} on {local_addr}");
            },
            SwarmEvent::ConnectionEstablished { peer_id, endpoint, .. } => {
                println!("Connected to peer: {peer_id} via {endpoint:?}");
            },
            SwarmEvent::ConnectionClosed { peer_id, cause, .. } => {
                println!("Connection to {peer_id} closed: {cause:?}");
            },
            _ => {
                //Handle silently
            }
                

    }
}
}