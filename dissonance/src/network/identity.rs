use ed25519_dalek::{SigningKey,VerifyingKey, SECRET_KEY_LENGTH, PUBLIC_KEY_LENGTH, KEYPAIR_LENGTH};
use libp2p::{identity, PeerId};
use rand::TryRngCore;
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub struct NodeIdentity{
    pub signing_key: SigningKey,
    pub verifying_key: VerifyingKey,
    pub peer_id: PeerId
}

#[derive(Serialize, Deserialize)]
struct StoredNodeIdentity{
    private_key_bytes: [u8; SECRET_KEY_LENGTH],
}

impl NodeIdentity{

    pub fn get_identity() -> Result<Self>{
        let identity_path = Self::get_identity_path()?;

        if identity_path.exists(){
            println!("loading existing identity from keypair: {}", identity_path.display());
            Self::load_from_file(&identity_path)
        }else{
            println!("Generating new identity!");
            let identity = Self::load_or_generate()?;
            identity.save_to_file(&identity_path)?;
            println!("Generated new identity and stored at: {}", identity_path.display());
            Ok(identity)
        }
    }
    pub fn generate_ephemeral() -> Result<Self> {
        println!("⚠️ Generating ephemeral (in-memory) identity, not persisted to disk!");
        let mut secret_bytes = [0u8; SECRET_KEY_LENGTH];
        rand::rngs::OsRng.try_fill_bytes(&mut secret_bytes)?;
        let signing_key: SigningKey = SigningKey::from_bytes(&secret_bytes);
        let verifying_key = signing_key.verifying_key();

        let lp2p_pub = identity::ed25519::PublicKey::try_from_bytes(&verifying_key.to_bytes())
            .context("Failed to create libp2p public key for ephemeral identity")?;
        let peer_id = PeerId::from_public_key(&identity::PublicKey::from(lp2p_pub));

        println!("Created ephemeral Node identity: {}", peer_id);
        Ok(NodeIdentity {
            signing_key,
            verifying_key,
            peer_id,
        })
    }
    fn load_or_generate() -> Result<Self>{
        let mut secret_bytes = [0u8; SECRET_KEY_LENGTH];
        rand::rngs::OsRng.try_fill_bytes(&mut secret_bytes)?;
        let signing_key: SigningKey = SigningKey::from_bytes(&secret_bytes);
        let verifying_key = signing_key.verifying_key();
        let lp2p_pub = identity::ed25519::PublicKey::try_from_bytes(&verifying_key.to_bytes()).context("Failed to cerate libp2p public key")?;
        let peer_id = PeerId::from_public_key(&identity::PublicKey::from(lp2p_pub));
        println!("Created Node identity: {}", peer_id);        
        Ok(NodeIdentity { signing_key:signing_key, verifying_key: verifying_key, peer_id: peer_id })
    }

    fn save_to_file(&self, path: &Path) -> Result<()>{

        if let Some(parent) = path.parent(){
            fs::create_dir_all(parent).context("Failed to create identity directory")?;
        }
        
        let stored = StoredNodeIdentity{
            private_key_bytes : self.signing_key.to_bytes(),
        };

        let content = serde_json::to_string_pretty(&stored).context("failed to serialize identity")?;

        fs::write(path, content).context("Failed to create identity file")?;
        println!("Node identity save to: {}", path.display());
        Ok(())
    }

    fn get_identity_path() -> Result<PathBuf>{
        let cf_dir = dirs::config_dir().context("Could not determine config directory")?;
        Ok(cf_dir.join("dsn-chat").join("node-identity.json"))
    }

    fn load_from_file(path: &Path) -> Result<Self>{
        let content = fs::read_to_string(path).context("Failed to read identity file")?;
        let stored :StoredNodeIdentity = serde_json::from_str(&content).context("Failed to parse identity")?;

        let signing_key = SigningKey::from_bytes(&stored.private_key_bytes);
        let verifying_key = signing_key.verifying_key();

        let lp2p_pub = identity::ed25519::PublicKey::try_from_bytes(&verifying_key.to_bytes()).context("Could not create lp2p public key")?;
        let peer_id = PeerId::from_public_key(&identity::PublicKey::from(lp2p_pub));

        println!("Created node identity: {}", peer_id);
        Ok(NodeIdentity { signing_key:signing_key, verifying_key: verifying_key, peer_id: peer_id })
    }

    pub fn peer_id(&self) -> PeerId{
        self.peer_id
    }

    pub fn pub_key_bytes(&self) -> [u8; PUBLIC_KEY_LENGTH]{
        self.verifying_key.to_bytes()
    }

    pub fn to_lp2p_keypair(&self) -> Result<identity::Keypair>{
        let mut keypair_bytes = [0u8; KEYPAIR_LENGTH];
        keypair_bytes[0..32].copy_from_slice(&self.signing_key.to_bytes());
        keypair_bytes[32..64].copy_from_slice(&self.verifying_key.to_bytes());
        let ed25519_keypair = identity::ed25519::Keypair::try_from_bytes(&mut keypair_bytes).context("Failed to create lp2p keypair")?;
        Ok(identity::Keypair::from(ed25519_keypair))
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn test_identity_path(temp: &tempfile::TempDir) -> PathBuf {
        temp.path().join("node-identity.json")
    }

    #[test]
    fn test_generate_and_serialize_identity() {
        let identity = NodeIdentity::load_or_generate().expect("Failed to create identity");
        let stored = StoredNodeIdentity {
            private_key_bytes: identity.signing_key.to_bytes(),
        };
        let json = serde_json::to_string_pretty(&stored).expect("Failed to serialize identity");
        assert!(json.contains("private_key_bytes"));
        assert_eq!(stored.private_key_bytes.len(), SECRET_KEY_LENGTH);
    }

    #[test]
    fn test_save_and_load_identity_file() {
        let temp = tempdir().unwrap();
        let path = test_identity_path(&temp);

        let identity = NodeIdentity::load_or_generate().expect("Failed to generate identity");
        identity.save_to_file(&path).expect("Failed to save identity");

        assert!(path.exists());

        let loaded = NodeIdentity::load_from_file(&path).expect("Failed to load identity");
        assert_eq!(identity.peer_id, loaded.peer_id);
        assert_eq!(identity.signing_key.to_bytes(), loaded.signing_key.to_bytes());
        assert_eq!(identity.verifying_key.to_bytes(), loaded.verifying_key.to_bytes());
    }

    #[test]
    fn test_pub_key_bytes() {
        let identity = NodeIdentity::load_or_generate().unwrap();
        let pub_bytes = identity.pub_key_bytes();
        assert_eq!(pub_bytes.len(), PUBLIC_KEY_LENGTH);
        assert_eq!(pub_bytes, identity.verifying_key.to_bytes());
    }

    #[test]
    fn test_to_lp2p_keypair() {
        let identity = NodeIdentity::load_or_generate().unwrap();
        let keypair = identity.to_lp2p_keypair().expect("Failed to convert to libp2p keypair");
        let peer_id = PeerId::from_public_key(&keypair.public());
        assert_eq!(peer_id, identity.peer_id);
    }
}
