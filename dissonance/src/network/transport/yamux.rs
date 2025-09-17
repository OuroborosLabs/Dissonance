use libp2p::yamux::Config;

pub fn build_yamux_config() -> Config{
    let mut yamux_config = Config::default();
    yamux_config.set_max_num_streams(256);

    yamux_config   
}