use clap::Parser;
use std::{sync::{Arc, Mutex}, net::SocketAddr, time::Duration};
use tokio::time;

mod blockchain;
mod mempool;
mod network;
mod wallet;

use crate::{
    blockchain::Blockchain,
    mempool::Mempool,
    wallet::Wallet,
    network::start_p2p_node
};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(long)] node: bool,
    #[arg(long)] mine: bool,
    #[arg(long)] stake: Option<u64>,
    #[arg(long, default_value = "127.0.0.1:8080")] listen_addr: SocketAddr,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    let blockchain = Arc::new(Mutex::new(Blockchain::new()));
    let mempool = Arc::new(Mutex::new(Mempool::new()));
    let wallet = Wallet::new();

    if args.node {
        println!("Node address: {}", wallet.address());
        
        // Start networking
        let listen_addr = format!("/ip4/{}/tcp/{}", 
            args.listen_addr.ip(), 
            args.listen_addr.port()
        ).parse()?;
        
        tokio::spawn(async move {
            start_p2p_node(listen_addr).await;
        });

        // Mining loop
        if args.mine {
            let blockchain = blockchain.clone();
            let mempool = mempool.clone();
            
            tokio::spawn(async move {
                let mut interval = time::interval(Duration::from_secs(10));
                loop {
                    interval.tick().await;
                    let mut chain = blockchain.lock().unwrap();
                    let mut mempool = mempool.lock().unwrap();
                    
                    let transactions = mempool.transactions.drain(..).collect();
                    chain.mine_block(transactions);
                    println!("Mined block #{}", chain.chain.len());
                }
            });
        }

        // Keep node running
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }

    Ok(())
}