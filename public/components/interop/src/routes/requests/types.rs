use crate::{ChainAddress, U256};

/// Unauthenticated quote/admission input; it cannot authorize a bridge claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransferRequest {
    pub route_id: [u8; 32],
    pub source_amount: U256,
    pub recipient: ChainAddress,
}
