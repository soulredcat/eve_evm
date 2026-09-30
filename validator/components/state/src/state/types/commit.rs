use alloy_consensus::Header;
use alloy_primitives::Bytes;

use super::{CompleteState, StateVersion};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BlockPayload {
    pub header: Header,
    /// Canonical signed transaction and typed receipt envelope bytes, in block order.
    pub transactions: Vec<Bytes>,
    pub receipts: Vec<Bytes>,
}

/// Structural whole-commit/recovery input, not evidence of consensus authorization.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateCommit {
    pub parent: Option<StateVersion>,
    pub target: StateVersion,
    pub state: CompleteState,
    pub block: BlockPayload,
}
