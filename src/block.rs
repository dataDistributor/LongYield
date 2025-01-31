use serde::{Serialize, Deserialize};
use sha3::{Digest, Keccak256};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub nonce: u64,
    pub hash: String,
}

impl Block {
    pub fn calculate_hash(&self) -> String {
        let data = serde_json::json!({
            "index": self.index,
            "timestamp": self.timestamp,
            "transactions": self.transactions,
            "previous_hash": self.previous_hash,
            "nonce": self.nonce,
        });
        let mut hasher = Keccak256::new();
        hasher.update(data.to_string().as_bytes());
        format!("{:x}", hasher.finalize())
    }
}