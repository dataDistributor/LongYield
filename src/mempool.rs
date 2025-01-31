use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Mempool {
    pub transactions: Vec<String>,
}

impl Mempool {
    pub fn new() -> Self {
        Mempool { transactions: vec![] }
    }
}