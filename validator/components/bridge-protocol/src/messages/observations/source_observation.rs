use eve_interop::{AssetOrigin, ChainAddress, ChainIdentity, U256};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InclusionLocator {
    EvmReceipt {
        transaction_index: u32,
        log_index: u32,
    },
    SolanaInstruction {
        transaction_index: u32,
        instruction_index: u16,
        inner_instruction_index: Option<u16>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofSegment {
    pub bytes: Vec<u8>,
}

/// Untrusted transport evidence. Ingress must bound decoding before allocating this input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceObservation {
    pub source: ChainIdentity,
    pub source_height: u64,
    pub source_block_hash: [u8; 32],
    pub source_custody: ChainAddress,
    pub source_sequence: u64,
    pub route_id: [u8; 32],
    pub asset: AssetOrigin,
    pub source_amount: U256,
    pub recipient: ChainAddress,
    pub inclusion: InclusionLocator,
    pub proof: Vec<ProofSegment>,
}
