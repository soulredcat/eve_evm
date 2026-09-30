use super::{BridgeTransferData, VerifiedBridgeMessage};

impl VerifiedBridgeMessage {
    pub fn transfer(&self) -> &BridgeTransferData {
        &self.transfer
    }
}
