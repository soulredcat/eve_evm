#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransactionReference {
    Evm { transaction_hash: [u8; 32] },
    Solana { signature: [u8; 64] },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DestinationExecutionOutcome {
    Applied,
    AlreadyConsumed,
}

/// A real chain executor's checked result; a submitted RPC transaction is insufficient.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DestinationExecutionReceipt {
    pub economic_transfer_id: [u8; 32],
    pub transaction: TransactionReference,
    pub outcome: DestinationExecutionOutcome,
}
