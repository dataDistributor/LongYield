use chrono::Utc;

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub transactions: Vec<String>,
    pub previous_hash: String,
    pub hash: String,
}

pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block {
            index: 0,
            timestamp: Utc::now().timestamp(),
            transactions: vec![],
            previous_hash: "genesis".into(),
            hash: "genesis_hash".into(),
        };

        Blockchain {
            chain: vec![genesis_block],
        }
    }

    pub fn mine_block(&mut self, transactions: Vec<String>) {
        let last_block = self.chain.last().unwrap();
        let new_block = Block {
            index: last_block.index + 1,
            timestamp: Utc::now().timestamp(),
            transactions,
            previous_hash: last_block.hash.clone(),
            hash: format!("block_{}", last_block.index + 1),
        };
        
        self.chain.push(new_block);
    }
}