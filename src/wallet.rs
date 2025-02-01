use crate::blockchain::{Blockchain, Transaction};

/// Computes the balance for the given address by scanning the blockchain.
pub fn get_balance(blockchain: &Blockchain, address: &str) -> i64 {
    let mut balance: i64 = 0;
    for block in &blockchain.chain {
        for tx in &block.transactions {
            if tx.from == address {
                balance -= tx.amount as i64;
            }
            if tx.to == address {
                balance += tx.amount as i64;
            }
        }
    }
    balance
}
