use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: u64,
    // The digital signature (if signed)
    pub signature: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: DateTime<Utc>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(index: u64, previous_hash: String, transactions: Vec<Transaction>) -> Self {
        let timestamp = Utc::now();
        let nonce = 0;
        let hash = Block::calculate_hash(index, timestamp, &previous_hash, nonce, &transactions);
        Block {
            index,
            timestamp,
            previous_hash,
            hash,
            nonce,
            transactions,
        }
    }

    pub fn calculate_hash(
        index: u64,
        timestamp: DateTime<Utc>,
        previous_hash: &str,
        nonce: u64,
        transactions: &Vec<Transaction>,
    ) -> String {
        let tx_data = serde_json::to_string(transactions).unwrap();
        let data = format!("{}{}{}{}{}", index, timestamp, previous_hash, nonce, tx_data);
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    /// Creates a new blockchain with a genesis block.
    pub fn new() -> Self {
        let genesis_block = Block::new(0, "0".to_string(), vec![]);
        Blockchain {
            chain: vec![genesis_block],
        }
    }

    pub fn add_block(&mut self, block: Block) {
        self.chain.push(block);
    }

    /// Verifies the integrity of the blockchain.
    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];
            if current.previous_hash != previous.hash {
                return false;
            }
            let calculated_hash = Block::calculate_hash(
                current.index,
                current.timestamp,
                &current.previous_hash,
                current.nonce,
                &current.transactions,
            );
            if current.hash != calculated_hash {
                return false;
            }
        }
        true
    }
}
