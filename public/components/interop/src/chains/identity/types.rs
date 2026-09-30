use alloy_primitives::U256;

/// These tags are permanent fields of the version-one identity encoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ChainNamespace {
    Eve = 1,
    Ethereum = 2,
    Solana = 3,
}

/// Genesis identifies the network; numeric chain IDs alone are not unique.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChainIdentity {
    pub namespace: ChainNamespace,
    pub genesis: [u8; 32],
    pub numeric_id: Option<U256>,
}
