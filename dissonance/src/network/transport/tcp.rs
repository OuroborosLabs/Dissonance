use libp2p::tcp::{Config as TcpConfig};


pub fn build_tcp_config() -> TcpConfig{
    TcpConfig::new().nodelay(true)
}