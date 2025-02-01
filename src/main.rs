use clap::{Parser, Subcommand};
mod blockchain;
mod consensus;
mod consensus_manager;
mod crypto;
mod network;
mod persistence;
mod transaction_pool;
mod wallet;

use blockchain::{Blockchain, Block, Transaction};
use consensus_manager::{ConsensusManager, ConsensusType};
use crypto::{Wallet, sign_transaction, verify_transaction};
use persistence::{load_blockchain, save_blockchain};
use transaction_pool::TransactionPool;
use wallet::get_balance;
use ed25519_dalek::Signature;
use std::thread;
use std::time::Duration;

#[derive(Parser)]
#[command(name = "LongYield")]
#[command(about = "A decentralized blockchain prototype", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start mining blocks (and optionally the P2P node)
    Mine {
        /// Also run the P2P node
        #[arg(long)]
        node: bool,
    },
    /// Submit a transaction: <from> <to> <amount>
    Tx {
        from: String,
        to: String,
        amount: u64,
    },
    /// Print the blockchain
    Print,
    /// Check balance for an address
    Balance {
        address: String,
    },
    /// Start only the P2P node
    Node,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Print) => {
            let blockchain_file = "blockchain.json";
            match load_blockchain(blockchain_file) {
                Ok(blockchain) => {
                    println!("Blockchain contents:");
                    for block in blockchain.chain.iter() {
                        println!("{:#?}", block);
                    }
                }
                Err(e) => println!("Error loading blockchain: {}", e),
            }
        }
        Some(Commands::Balance { address }) => {
            let blockchain_file = "blockchain.json";
            match load_blockchain(blockchain_file) {
                Ok(blockchain) => {
                    let balance = get_balance(&blockchain, address);
                    println!("Balance for {}: {}", address, balance);
                }
                Err(e) => println!("Error loading blockchain: {}", e),
            }
        }
        Some(Commands::Tx { from, to, amount }) => {
            // In a real system, wallets would be persistent and securely managed.
            let wallet = Wallet::new();
            let tx_data = format!("{}{}{}", from, to, amount);
            let signature = sign_transaction(&wallet.keypair, tx_data.as_bytes());
            let transaction = Transaction {
                from: from.clone(),
                to: to.clone(),
                amount: *amount,
                signature: Some(signature.to_bytes().to_vec()),
            };
            // Verify the signature (for demonstration)
            let verified = verify_transaction(&wallet.keypair.public, tx_data.as_bytes(), &signature);
            if verified {
                let tx_pool = TransactionPool::new();
                tx_pool.add_transaction(transaction);
                println!("Transaction added to pool with valid signature.");
            } else {
                println!("Transaction signature verification failed.");
            }
        }
        Some(Commands::Mine { node }) => {
            if *node {
                let listen_addr = "/ip4/127.0.0.1/tcp/4001".parse().unwrap();
                tokio::spawn(async move {
                    network::start_p2p_node(listen_addr).await;
                });
            }
            let blockchain_file = "blockchain.json";
            let mut blockchain = match load_blockchain(blockchain_file) {
                Ok(bc) => bc,
                Err(_) => {
                    println!("No saved blockchain found. Creating a new one.");
                    Blockchain::new()
                }
            };

            let tx_pool = TransactionPool::new();
            if tx_pool.get_transactions().is_empty() {
                tx_pool.add_transaction(Transaction {
                    from: "Alice".to_string(),
                    to: "Bob".to_string(),
                    amount: 50,
                    signature: None,
                });
            }

            let consensus_manager = ConsensusManager::new(4);

            println!("Starting mining loop...");
            loop {
                let pending_transactions = tx_pool.get_transactions();
                let transactions = if pending_transactions.is_empty() {
                    vec![Transaction {
                        from: "Alice".to_string(),
                        to: "Bob".to_string(),
                        amount: 50,
                        signature: None,
                    }]
                } else {
                    pending_transactions
                };

                let previous_hash = blockchain.chain.last().unwrap().hash.clone();
                let mut block = Block::new(blockchain.chain.len() as u64, previous_hash, transactions);

                println!("Mining block {}...", block.index);
                consensus_manager.execute(ConsensusType::PoW, &mut block);
                blockchain.add_block(block);
                println!("Block mined! Chain length is now: {}", blockchain.chain.len());

                tx_pool.clear();

                if let Err(e) = save_blockchain(&blockchain, blockchain_file) {
                    println!("Error saving blockchain: {}", e);
                }

                thread::sleep(Duration::from_secs(1));
            }
        }
        Some(Commands::Node) => {
            let listen_addr = "/ip4/127.0.0.1/tcp/4001".parse().unwrap();
            tokio::spawn(async move {
                network::start_p2p_node(listen_addr).await;
            });
            println!("P2P node started. Press Ctrl+C to exit.");
            loop {
                thread::sleep(Duration::from_secs(1));
            }
        }
        None => {
            println!("No command provided. Use --help for usage information.");
        }
    }
}
