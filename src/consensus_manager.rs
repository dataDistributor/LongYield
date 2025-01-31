// src/consensus_manager.rs

use crate::blockchain::Block;
use crate::consensus::{Consensus, PowConsensus, PosConsensus};

/// Enum to represent the consensus mechanism choices.
pub enum ConsensusType {
    PoW,
    PoS,
    // In the future you can add: ProofOfLocation, ProofOfIntegrity, etc.
}

/// The ConsensusManager holds our consensus implementations and selects one as needed.
pub struct ConsensusManager {
    pow: PowConsensus,
    pos: PosConsensus,
    // Additional consensus mechanisms can be added here.
}

impl ConsensusManager {
    /// Create a new ConsensusManager.
    pub fn new(pow_difficulty: usize) -> Self {
        ConsensusManager {
            pow: PowConsensus::new(pow_difficulty),
            pos: PosConsensus::new(),
        }
    }
    
    /// Select the desired consensus implementation.
    pub fn select_consensus(&self, consensus_type: ConsensusType) -> &dyn Consensus {
        match consensus_type {
            ConsensusType::PoW => &self.pow,
            ConsensusType::PoS => &self.pos,
        }
    }
    
    /// Execute the consensus process on a block using the chosen mechanism.
    pub fn execute(&self, consensus_type: ConsensusType, block: &mut Block) {
        let consensus = self.select_consensus(consensus_type);
        consensus.execute_consensus(block);
    }
}
