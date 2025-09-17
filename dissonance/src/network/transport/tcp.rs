use libp2p::tcp::Config as TcpConfig;


pub fn build_tcp_transport() -> TcpConfig{

    TcpConfig::default().nodelay(true)
}