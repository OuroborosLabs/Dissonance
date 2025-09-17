use libp2p::noise::{self, Config as NoiseConfig};
use libp2p::identity;

pub fn build_noise_config<'a>(local_keypair: &'a identity::Keypair) -> Result<NoiseConfig,noise::Error>{
    NoiseConfig::new(local_keypair)
}
