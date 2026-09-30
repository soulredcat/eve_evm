use super::VerifiedBridgeMessage;

impl VerifiedBridgeMessage {
    pub fn economic_transfer_id(&self) -> [u8; 32] {
        self.economic_transfer_id
    }
}
