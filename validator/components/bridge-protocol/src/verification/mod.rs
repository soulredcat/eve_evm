mod context;
mod source_finality_verifier;
mod validate_source_context;
pub use context::{
    BridgeProtocolError, DeploymentIdentity, ProofLimits, RequiredAuthenticationProfile,
    SourceVerificationContext, TrustAnchorReference,
};
pub use source_finality_verifier::SourceFinalityVerifier;
pub(crate) use validate_source_context::validate_source_context;
