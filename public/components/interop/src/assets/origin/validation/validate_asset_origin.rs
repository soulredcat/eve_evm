use crate::{
    AddressBytes, AssetKind, AssetOrigin, ChainNamespace, InteropError, TokenBehavior,
    validate_chain_address, validate_chain_identity,
};

/// Enforce initial metadata scope; this does not verify token implementation behavior.
pub fn validate_asset_origin(asset: &AssetOrigin) -> Result<(), InteropError> {
    validate_chain_identity(&asset.chain)?;
    match asset.kind {
        AssetKind::Native => Ok(()),
        AssetKind::Erc20 { contract, behavior } => {
            validate_chain_address(&contract)?;
            if asset.chain.namespace == ChainNamespace::Solana || contract.chain != asset.chain {
                return Err(InteropError::AssetNamespaceMismatch);
            }
            if contract.bytes == AddressBytes::Evm([0; 20]) {
                return Err(InteropError::InvalidAssetAddress);
            }
            if behavior != TokenBehavior::Standard {
                return Err(InteropError::UnsupportedTokenBehavior);
            }
            Ok(())
        }
        AssetKind::SplToken {
            mint,
            token_program,
            behavior,
        } => {
            validate_chain_address(&mint)?;
            validate_chain_address(&token_program)?;
            if asset.chain.namespace != ChainNamespace::Solana
                || mint.chain != asset.chain
                || token_program.chain != asset.chain
            {
                return Err(InteropError::AssetNamespaceMismatch);
            }
            if mint.bytes == AddressBytes::Solana([0; 32]) {
                return Err(InteropError::InvalidAssetAddress);
            }
            if behavior != TokenBehavior::Standard {
                return Err(InteropError::UnsupportedTokenBehavior);
            }
            let standard = crate::parse_chain_address(
                asset.chain,
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
            )?;
            if token_program != standard {
                return Err(InteropError::UnsupportedTokenProgram);
            }
            Ok(())
        }
    }
}
