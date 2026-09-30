use crate::{
    BridgeProtocolError, DeploymentIdentity, DestinationExecutionReceipt, VerifiedBridgeMessage,
};

/// Implementations must atomically/durably consume replay identity and custody effects.
/// No master/relayer assertion or timeout-only refund satisfies this contract.
pub trait DestinationExecutor {
    fn execute_verified_message(
        &mut self,
        destination: &DeploymentIdentity,
        message: &VerifiedBridgeMessage,
    ) -> Result<DestinationExecutionReceipt, BridgeProtocolError>;
}
