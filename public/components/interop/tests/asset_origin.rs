use eve_interop::{
    AddressBytes, AssetKind, AssetOrigin, ChainNamespace, InteropError, TokenBehavior,
    decode_chain_address, encode_route_binding, parse_chain_address, validate_asset_origin,
};
mod support;

#[test]
fn ti01_erc20_identity_binds_contract_genesis_and_namespace() {
    let ethereum = support::network(ChainNamespace::Ethereum);
    let contract = decode_chain_address(ethereum, &[6; 20]).unwrap();
    let asset = AssetOrigin {
        chain: ethereum,
        kind: AssetKind::Erc20 {
            contract,
            behavior: TokenBehavior::Standard,
        },
    };
    assert_eq!(validate_asset_origin(&asset), Ok(()));
    let mut wrong = asset;
    wrong.chain.genesis[0] ^= 1;
    assert_eq!(
        validate_asset_origin(&wrong),
        Err(InteropError::AssetNamespaceMismatch)
    );
    let mut route = support::named_routes()[0];
    route.asset = asset;
    let first = encode_route_binding(&route).unwrap();
    route.asset.kind = AssetKind::Erc20 {
        contract: decode_chain_address(ethereum, &[7; 20]).unwrap(),
        behavior: TokenBehavior::Standard,
    };
    assert_ne!(encode_route_binding(&route).unwrap(), first);
}

#[test]
fn ti06_nonstandard_erc20_behaviors_and_native_alias_are_rejected() {
    let ethereum = support::network(ChainNamespace::Ethereum);
    let contract = decode_chain_address(ethereum, &[6; 20]).unwrap();
    for behavior in [
        TokenBehavior::FeeOnTransfer,
        TokenBehavior::Rebasing,
        TokenBehavior::Callback,
        TokenBehavior::Token2022 {
            extension_bitmap: 0,
        },
    ] {
        assert_eq!(
            validate_asset_origin(&AssetOrigin {
                chain: ethereum,
                kind: AssetKind::Erc20 { contract, behavior }
            }),
            Err(InteropError::UnsupportedTokenBehavior)
        );
    }
    let zero = decode_chain_address(ethereum, &[0; 20]).unwrap();
    assert_eq!(
        validate_asset_origin(&AssetOrigin {
            chain: ethereum,
            kind: AssetKind::Erc20 {
                contract: zero,
                behavior: TokenBehavior::Standard
            }
        }),
        Err(InteropError::InvalidAssetAddress)
    );
}

#[test]
fn ti01_standard_spl_identity_binds_mint_and_exact_token_program() {
    let solana = support::network(ChainNamespace::Solana);
    let mint = decode_chain_address(solana, &[6; 32]).unwrap();
    let token_program =
        parse_chain_address(solana, "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap();
    let mut asset = AssetOrigin {
        chain: solana,
        kind: AssetKind::SplToken {
            mint,
            token_program,
            behavior: TokenBehavior::Standard,
        },
    };
    assert_eq!(validate_asset_origin(&asset), Ok(()));
    let unknown = decode_chain_address(solana, &[7; 32]).unwrap();
    asset.kind = AssetKind::SplToken {
        mint,
        token_program: unknown,
        behavior: TokenBehavior::Standard,
    };
    assert_eq!(
        validate_asset_origin(&asset),
        Err(InteropError::UnsupportedTokenProgram)
    );
    assert!(matches!(mint.bytes, AddressBytes::Solana(_)));
}

#[test]
fn ti06_token2022_and_wrong_network_spl_accounts_fail_closed() {
    let solana = support::network(ChainNamespace::Solana);
    let mint = decode_chain_address(solana, &[6; 32]).unwrap();
    let token_program =
        parse_chain_address(solana, "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap();
    for extension_bitmap in [0, 1, u64::MAX] {
        assert_eq!(
            validate_asset_origin(&AssetOrigin {
                chain: solana,
                kind: AssetKind::SplToken {
                    mint,
                    token_program,
                    behavior: TokenBehavior::Token2022 { extension_bitmap }
                }
            }),
            Err(InteropError::UnsupportedTokenBehavior)
        );
    }
    let mut wrong_mint = mint;
    wrong_mint.chain.genesis[0] ^= 1;
    assert_eq!(
        validate_asset_origin(&AssetOrigin {
            chain: solana,
            kind: AssetKind::SplToken {
                mint: wrong_mint,
                token_program,
                behavior: TokenBehavior::Standard
            }
        }),
        Err(InteropError::AssetNamespaceMismatch)
    );
}
