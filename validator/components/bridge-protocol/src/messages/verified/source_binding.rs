use super::{SourceMessageBinding, VerifiedBridgeMessage};

impl VerifiedBridgeMessage {
    pub fn source_binding(&self) -> &SourceMessageBinding {
        &self.source_binding
    }
}
