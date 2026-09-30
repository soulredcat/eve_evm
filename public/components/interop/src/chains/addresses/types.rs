use crate::ChainIdentity;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressBytes {
    Evm([u8; 20]),
    Solana([u8; 32]),
}

/// Address bytes always carry the complete network identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChainAddress {
    pub chain: ChainIdentity,
    pub bytes: AddressBytes,
}
