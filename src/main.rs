// src/main.rs

mod blockchain;
mod consensus;
mod consensus_manager;
mod network;

use blockchain::{Blockchain, Block, Transaction};
use consensus_manager::{ConsensusManager, ConsensusType};
use std::env;

#[tokio::main]
async fn main() {
    // Parse command-line arguments.
    let args: Vec<String> = env::args().collect();
    let mut run_node = false;
    let mut mine = false;
    let mut stake = false;
    
    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "--node" => run_node = true,
            "--mine" => mine = true,
            "--stake" => stake = true,
            _ => {}
        }
    }
    
    // Start the network node if requested.
    if run_node {
        // Adjust the listen address as needed.
        let listen_addr = "/ip4/127.0.0.1/tcp/4001".parse().unwrap();
        tokio::spawn(async move {
            network::start_p2p_node(listen_addr).await;
        });
    }
    
    // Initialize the blockchain.
    let mut blockchain = Blockchain::new();
    
    // Create a sample transaction.
    let transactions = vec![
        Transaction { 
            from: "Alice".to_string(), 
            to: "Bob".to_string(), 
            amount: 50 
        }
    ];
    
    // Create a new block using the previous block's hash.
    let previous_hash = blockchain.chain.last().unwrap().hash.clone();
    let mut block = Block::new(blockchain.chain.len() as u64, previous_hash, transactions);
    
    // Initialize the Consensus Manager.
    let consensus_manager = ConsensusManager::new(4); // PoW difficulty set to 4.
    
    // Execute consensus based on the flags.
    if mine {
        println!("Executing Proof of Work (PoW) consensus...");
        consensus_manager.execute(ConsensusType::PoW, &mut block);
    } else if stake {
        println!("Executing Proof of Stake (PoS) consensus...");
        consensus_manager.execute(ConsensusType::PoS, &mut block);
    } else {
        println!("Adding block without running consensus...");
    }
    
    // Add the block to the blockchain.
    blockchain.add_block(block);
    println!("Blockchain valid: {}", blockchain.is_valid());
}
