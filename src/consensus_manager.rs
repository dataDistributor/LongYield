use crate::blockchain::Block;
use crate::consensus::{Consensus, PowConsensus, PosConsensus};

pub enum ConsensusType {
    PoW,
    PoS,
}

pub struct ConsensusManager {
    pow: PowConsensus,
    pos: PosConsensus,
}

impl ConsensusManager {
    pub fn new(pow_difficulty: usize) -> Self {
        ConsensusManager {
            pow: PowConsensus::new(pow_difficulty),
            pos: PosConsensus::new(),
        }
    }

    pub fn select_consensus(&self, consensus_type: ConsensusType) -> &dyn Consensus {
        match consensus_type {
            ConsensusType::PoW => &self.pow,
            ConsensusType::PoS => &self.pos,
        }
    }

    pub fn execute(&self, consensus_type: ConsensusType, block: &mut Block) {
        let consensus = self.select_consensus(consensus_type);
        consensus.execute_consensus(block);
    }
}
