use eve_interop::{ChainAddress, ChainIdentity, InteropError, RouteManifest};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequiredAuthenticationProfile {
    ClassicalDevelopment,
    ClassicalAndMldsa65 {
        encoding_version: u16,
        key_epoch: u64,
    },
}

/// Policy input referencing an anchor; this type does not authenticate it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TrustAnchorReference {
    pub chain: ChainIdentity,
    pub height: u64,
    pub block_hash: [u8; 32],
    pub historical_key_set_commitment: [u8; 32],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeploymentIdentity {
    pub address: ChainAddress,
    pub implementation_commitment: [u8; 32],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProofLimits {
    pub maximum_total_bytes: u32,
    pub maximum_segment_bytes: u32,
    pub maximum_segments: u16,
}

/// Future checked verifiers must authenticate all policy history; metadata is not approval.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceVerificationContext {
    pub route: RouteManifest,
    pub required_profile: RequiredAuthenticationProfile,
    pub source_anchor: TrustAnchorReference,
    pub source_verifier_identity: [u8; 32],
    pub destination_deployment: DeploymentIdentity,
    pub proof_limits: ProofLimits,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeProtocolError {
    InvalidMetadata(InteropError),
    WrongSourceNetwork,
    WrongSourceCustody,
    WrongOriginAsset,
    InvalidSourceReference,
    WrongInclusionLocator,
    InvalidProofLimits,
    ProofLimitExceeded,
    EmptyProof,
    AuthenticationUnavailable,
    AuthenticationFailed,
    InclusionFailed,
    StaleAnchor,
    UnsupportedProfile,
    WrongDeployment,
    InvalidQuote,
    ExpiredQuote,
    DestinationExecutionFailed,
}
