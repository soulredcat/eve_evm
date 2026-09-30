use crate::RequiredAuthenticationProfile;
use eve_interop::{AssetOrigin, ChainAddress, ChainIdentity, CryptoCoverage, U256};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuotedFee {
    pub chain: ChainIdentity,
    pub asset: AssetOrigin,
    pub amount: U256,
}

/// Estimates/disclosures only; no quote field authenticates finality or approves custody.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BridgeQuote {
    pub route_id: [u8; 32],
    pub route_version: u16,
    pub asset: AssetOrigin,
    pub source_amount: U256,
    pub estimated_destination_amount: U256,
    pub minimum_destination_amount: U256,
    pub recipient: ChainAddress,
    pub source_fee: QuotedFee,
    pub destination_fee: QuotedFee,
    pub relayer_fee: Option<QuotedFee>,
    pub account_creation_fee: Option<QuotedFee>,
    pub expiry_source_height: u64,
    pub required_profile: RequiredAuthenticationProfile,
    pub crypto_coverage: CryptoCoverage,
}
