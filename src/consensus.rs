use crate::blockchain::Block;

pub trait Consensus {
    fn execute_consensus(&self, block: &mut Block);
    fn validate_block(&self, block: &Block) -> bool;
}

pub struct PowConsensus {
    pub difficulty: usize,
}

impl PowConsensus {
    pub fn new(difficulty: usize) -> Self {
        PowConsensus { difficulty }
    }
}

impl Consensus for PowConsensus {
    fn execute_consensus(&self, block: &mut Block) {
        let target = "0".repeat(self.difficulty);
        while !block.hash.starts_with(&target) {
            block.nonce += 1;
            block.hash = Block::calculate_hash(
                block.index,
                block.timestamp,
                &block.previous_hash,
                block.nonce,
                &block.transactions,
            );
        }
    }

    fn validate_block(&self, block: &Block) -> bool {
        let target = "0".repeat(self.difficulty);
        block.hash.starts_with(&target)
            && block.hash == Block::calculate_hash(
                block.index,
                block.timestamp,
                &block.previous_hash,
                block.nonce,
                &block.transactions,
            )
    }
}

pub struct PosConsensus {}

impl PosConsensus {
    pub fn new() -> Self {
        PosConsensus {}
    }
}

impl Consensus for PosConsensus {
    fn execute_consensus(&self, _block: &mut Block) {
        unimplemented!("Proof-of-Stake consensus not implemented yet");
    }

    fn validate_block(&self, _block: &Block) -> bool {
        unimplemented!("Proof-of-Stake validation not implemented yet");
    }
}
