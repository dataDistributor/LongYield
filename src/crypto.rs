use ed25519_dalek::{Keypair, Signature, Signer, Verifier};
use rand::rngs::OsRng; // Using OsRng from the rand crate

/// Signs the given transaction data with the provided keypair.
pub fn sign_transaction(keypair: &Keypair, transaction_data: &[u8]) -> Signature {
    keypair.sign(transaction_data)
}

/// Verifies the transaction signature using the provided public key.
pub fn verify_transaction(
    public_key: &ed25519_dalek::PublicKey,
    transaction_data: &[u8],
    signature: &Signature,
) -> bool {
    public_key.verify(transaction_data, signature).is_ok()
}

/// A simple wallet that holds an Ed25519 keypair.
pub struct Wallet {
    pub keypair: Keypair,
}

impl Wallet {
    pub fn new() -> Self {
        let mut csprng = OsRng {};
        let keypair = Keypair::generate(&mut csprng);
        Wallet { keypair }
    }
}
