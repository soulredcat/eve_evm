use crate::{
    BoundedSourceObservation, BridgeProtocolError, SourceVerificationContext, VerifiedBridgeMessage,
};

/// Actual source finality/history/profile and custody inclusion must all succeed.
/// RPC labels/relayer assertions are not an implementation of this contract.
pub trait SourceFinalityVerifier {
    fn verify_source_finality(
        &self,
        context: &SourceVerificationContext,
        observation: &BoundedSourceObservation,
    ) -> Result<VerifiedBridgeMessage, BridgeProtocolError>;
}
