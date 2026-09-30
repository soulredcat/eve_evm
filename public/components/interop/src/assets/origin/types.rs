use crate::{ChainAddress, ChainIdentity};

/// Metadata assertions are untrusted until chain-specific evidence is verified.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenBehavior {
    Standard,
    FeeOnTransfer,
    Rebasing,
    Callback,
    Token2022 { extension_bitmap: u64 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetKind {
    Native,
    Erc20 {
        contract: ChainAddress,
        behavior: TokenBehavior,
    },
    SplToken {
        mint: ChainAddress,
        token_program: ChainAddress,
        behavior: TokenBehavior,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AssetOrigin {
    pub chain: ChainIdentity,
    pub kind: AssetKind,
}
