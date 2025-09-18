use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature, Signer, Verifier, SECRET_KEY_LENGTH};
use rand::rngs::OsRng;
use sha2::{Digest, Sha256};

pub struct CryptoKeyPair {
    pub keypair: Keypair,
}

impl CryptoKeyPair {
    pub fn generate() -> Self {
        let mut csprng = OsRng;
        let keypair = Keypair::generate(&mut csprng);
        Self { keypair }
    }

    pub fn from_secret_key(secret_key: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        if secret_key.len() != SECRET_KEY_LENGTH {
            return Err("Invalid secret key length".into());
        }
        let mut secret_array = [0u8; SECRET_KEY_LENGTH];
        secret_array.copy_from_slice(secret_key);
        let secret = SecretKey::from_bytes(&secret_array)?;
        let public = PublicKey::from(&secret);
        let keypair = Keypair { secret, public };
        Ok(Self { keypair })
    }

    pub fn public_key(&self) -> Vec<u8> {
        self.keypair.public.to_bytes().to_vec()
    }

    pub fn secret_key(&self) -> Vec<u8> {
        self.keypair.secret.to_bytes().to_vec()
    }

    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        let signature = self.keypair.sign(message);
        signature.to_bytes().to_vec()
    }
}

pub fn verify_signature(
    public_key: &[u8],
    message: &[u8],
    signature: &[u8],
) -> Result<bool, Box<dyn std::error::Error>> {
    if public_key.len() != 32 {
        return Err("Invalid public key length".into());
    }
    if signature.len() != 64 {
        return Err("Invalid signature length".into());
    }

    let mut pk_array = [0u8; 32];
    pk_array.copy_from_slice(public_key);
    let public_key = PublicKey::from_bytes(&pk_array)?;

    let mut sig_array = [0u8; 64];
    sig_array.copy_from_slice(signature);
    let signature = Signature::from_bytes(&sig_array)?;

    match public_key.verify(message, &signature) {
        Ok(()) => Ok(true),
        Err(_) => Ok(false),
    }
}

pub fn hash_credential(credential_data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(credential_data);
    hasher.finalize().to_vec()
}