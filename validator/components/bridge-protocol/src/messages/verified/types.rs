use crate::RequiredAuthenticationProfile;
use eve_interop::{AssetOrigin, ChainAddress, ChainIdentity, U256};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BridgeTransferData {
    pub route_id: [u8; 32],
    pub asset: AssetOrigin,
    pub source_amount: U256,
    pub destination_amount: U256,
    pub recipient: ChainAddress,
    pub source_sequence: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceMessageBinding {
    pub source: ChainIdentity,
    pub source_height: u64,
    pub source_block_hash: [u8; 32],
    pub route_binding: [u8; 32],
    pub authenticated_profile: RequiredAuthenticationProfile,
}

/// Only a future reviewed checked verifier inside this canonical module may construct it.
/// No public/default/deserialization or observation-to-verified constructor exists.
///
/// ```compile_fail
/// use eve_bridge_protocol::VerifiedBridgeMessage;
/// let forged = VerifiedBridgeMessage { economic_transfer_id: [0; 32], source_binding: todo!(), transfer: todo!() };
/// ```
/// ```compile_fail
/// use eve_bridge_protocol::VerifiedBridgeMessage;
/// let forged: VerifiedBridgeMessage = Default::default();
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifiedBridgeMessage {
    pub(super) economic_transfer_id: [u8; 32],
    pub(super) source_binding: SourceMessageBinding,
    pub(super) transfer: BridgeTransferData,
}
