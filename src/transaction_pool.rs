use crate::blockchain::Transaction;
use std::sync::Mutex;

#[derive(Debug)]
pub struct TransactionPool {
    pub transactions: Mutex<Vec<Transaction>>,
}

impl TransactionPool {
    pub fn new() -> Self {
        TransactionPool {
            transactions: Mutex::new(Vec::new()),
        }
    }

    pub fn add_transaction(&self, tx: Transaction) {
        let mut transactions = self.transactions.lock().unwrap();
        transactions.push(tx);
    }

    pub fn get_transactions(&self) -> Vec<Transaction> {
        let transactions = self.transactions.lock().unwrap();
        transactions.clone()
    }

    pub fn clear(&self) {
        let mut transactions = self.transactions.lock().unwrap();
        transactions.clear();
    }
}
