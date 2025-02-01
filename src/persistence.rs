use crate::blockchain::Blockchain;
use std::fs;
use std::error::Error;

pub fn save_blockchain(blockchain: &Blockchain, filename: &str) -> Result<(), Box<dyn Error>> {
    let json = serde_json::to_string_pretty(blockchain)?;
    fs::write(filename, json)?;
    println!("Blockchain saved to {}", filename);
    Ok(())
}

pub fn load_blockchain(filename: &str) -> Result<Blockchain, Box<dyn Error>> {
    let data = fs::read_to_string(filename)?;
    let blockchain: Blockchain = serde_json::from_str(&data)?;
    println!("Blockchain loaded from {}", filename);
    Ok(blockchain)
}
