use ring::signature::{Ed25519KeyPair, KeyPair};
use rand::RngCore;
use hex;

#[derive(Debug)]
pub struct Wallet {
    keypair: Ed25519KeyPair,
}

impl Wallet {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut seed = [0u8; 32];
        rng.fill_bytes(&mut seed);
        let keypair = Ed25519KeyPair::from_seed_unchecked(&seed).unwrap();
        Wallet { keypair }
    }

    pub fn address(&self) -> String {
        hex::encode(self.keypair.public_key().as_ref())
    }
}