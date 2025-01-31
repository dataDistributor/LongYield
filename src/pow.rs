impl Blockchain {
    pub fn mine_block(&mut self, miner_address: &str) {
        let last_block = self.chain.last().unwrap();
        let mut nonce = 0;

        loop {
            let mut candidate_block = Block {
                index: last_block.index + 1,
                timestamp: chrono::Utc::now().timestamp(),
                transactions: self.pending_transactions.clone(),
                previous_hash: last_block.hash.clone(),
                nonce,
                hash: String::new(),
            };
            candidate_block.hash = candidate_block.calculate_hash();

            // ASIC-resistant difficulty: require hash starts with "00"
            if candidate_block.hash.starts_with("00") {
                self.add_block(candidate_block);
                self.pending_transactions.clear();
                break;
            }
            nonce += 1;
        }
    }
}