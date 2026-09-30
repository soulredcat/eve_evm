use crate::{AssetOrigin, ChainAddress, ChainIdentity, U256};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RouteState {
    NotImplemented,
    LocalFixtureOnly,
    VerifierIncomplete,
    ReviewRequired,
    Paused,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum VerificationRequirement {
    EthereumBeaconFinalityAndReceipt = 1,
    EveConsensusAndApplicationCommitment = 2,
    SolanaAuthenticatedReplay = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CryptoCoverage {
    Classical = 1,
    HybridExperimental = 2,
    MixedTrust = 3,
}

/// Version-one binding metadata, not an authenticated/approved route capability.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RouteManifest {
    pub version: u16,
    pub route_id: [u8; 32],
    pub source: ChainIdentity,
    pub destination: ChainIdentity,
    pub source_custody: ChainAddress,
    pub destination_custody: ChainAddress,
    pub asset: AssetOrigin,
    pub source_decimals: u8,
    pub destination_decimals: u8,
    pub maximum_source_amount: U256,
    pub maximum_destination_amount: U256,
    pub verification: VerificationRequirement,
    pub crypto_coverage: CryptoCoverage,
    pub state: RouteState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InteropError {
    InvalidGenesis,
    InvalidNumericChainId,
    InvalidAddressWidth,
    InvalidAddressEncoding,
    NonCanonicalAddress,
    AddressNamespaceMismatch,
    AssetNamespaceMismatch,
    InvalidAssetAddress,
    UnsupportedTokenBehavior,
    UnsupportedTokenProgram,
    InvalidDecimals,
    ZeroAmount,
    NonExactAmount,
    AmountOverflow,
    UnknownRoute,
    DuplicateRoute,
    RegistryCapacityExceeded,
    UnsupportedManifestVersion,
    InvalidRouteIdentity,
    CustodyNamespaceMismatch,
    InvalidCustodyAddress,
    UnsupportedAssetForwarding,
    VerificationNamespaceMismatch,
    RecipientNamespaceMismatch,
    RouteDisabled(RouteState),
}
