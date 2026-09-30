//! Typed bridge verification/execution contracts; no verifier or custody implementation.
#![forbid(unsafe_code)]
mod messages;
mod quotes;
mod verification;

pub use messages::{
    BoundedSourceObservation, BridgeTransferData, DestinationExecutionOutcome,
    DestinationExecutionReceipt, DestinationExecutor, InclusionLocator, ProofSegment,
    SourceMessageBinding, SourceObservation, TransactionReference, VerifiedBridgeMessage,
    validate_source_observation,
};
pub use quotes::{BridgeQuote, QuotedFee, validate_bridge_quote};
pub use verification::{
    BridgeProtocolError, DeploymentIdentity, ProofLimits, RequiredAuthenticationProfile,
    SourceFinalityVerifier, SourceVerificationContext, TrustAnchorReference,
};
