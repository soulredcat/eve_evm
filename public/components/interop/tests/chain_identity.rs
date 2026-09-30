use eve_interop::{
    AddressBytes, ChainAddress, ChainNamespace, InteropError, U256, decode_chain_address,
    encode_chain_identity, format_chain_address, parse_chain_address, validate_chain_address,
    validate_chain_identity,
};
mod support;

#[test]
fn ti01_namespace_and_genesis_prevent_numeric_chain_id_collision() {
    let mut ethereum = support::network(ChainNamespace::Ethereum);
    let eve = support::network(ChainNamespace::Eve);
    ethereum.numeric_id = eve.numeric_id;
    ethereum.genesis = eve.genesis;
    assert_ne!(
        encode_chain_identity(&ethereum),
        encode_chain_identity(&eve)
    );
    let mut other_genesis = eve;
    other_genesis.genesis[0] ^= 1;
    assert_ne!(
        encode_chain_identity(&eve),
        encode_chain_identity(&other_genesis)
    );
}

#[test]
fn ti01_chain_identity_rejects_zero_genesis_wrong_or_zero_numeric_id() {
    for namespace in [
        ChainNamespace::Eve,
        ChainNamespace::Ethereum,
        ChainNamespace::Solana,
    ] {
        let mut chain = support::network(namespace);
        chain.genesis = [0; 32];
        assert_eq!(
            validate_chain_identity(&chain),
            Err(InteropError::InvalidGenesis)
        );
        chain = support::network(namespace);
        chain.numeric_id = if namespace == ChainNamespace::Solana {
            Some(U256::from(1))
        } else {
            None
        };
        assert_eq!(
            validate_chain_identity(&chain),
            Err(InteropError::InvalidNumericChainId)
        );
    }
    let mut chain = support::network(ChainNamespace::Ethereum);
    chain.numeric_id = Some(U256::ZERO);
    assert_eq!(
        validate_chain_identity(&chain),
        Err(InteropError::InvalidNumericChainId)
    );
}

#[test]
fn ti01_address_widths_are_checked_without_truncation() {
    let ethereum = support::network(ChainNamespace::Ethereum);
    let solana = support::network(ChainNamespace::Solana);
    assert_eq!(
        decode_chain_address(ethereum, &[7; 32]),
        Err(InteropError::InvalidAddressWidth)
    );
    assert_eq!(
        decode_chain_address(solana, &[7; 20]),
        Err(InteropError::InvalidAddressWidth)
    );
    assert_eq!(
        validate_chain_address(&ChainAddress {
            chain: solana,
            bytes: AddressBytes::Evm([7; 20])
        }),
        Err(InteropError::AddressNamespaceMismatch)
    );
    assert_eq!(
        decode_chain_address(solana, &[7; 33]),
        Err(InteropError::InvalidAddressWidth)
    );
}

#[test]
fn ti01_canonical_display_roundtrips_and_rejects_bad_encodings() {
    let ethereum = support::network(ChainNamespace::Ethereum);
    let evm = parse_chain_address(ethereum, "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
    assert_eq!(
        format_chain_address(&evm).unwrap(),
        "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
    );
    assert_eq!(
        parse_chain_address(ethereum, "0xAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"),
        Err(InteropError::NonCanonicalAddress)
    );
    assert!(parse_chain_address(ethereum, "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").is_err());
    let solana = support::network(ChainNamespace::Solana);
    let program = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
    assert_eq!(
        format_chain_address(&parse_chain_address(solana, program).unwrap()).unwrap(),
        program
    );
    assert!(parse_chain_address(solana, "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").is_err());
    assert!(parse_chain_address(solana, "111111111111111111111111111111111").is_err());
    assert!(parse_chain_address(solana, &"1".repeat(1000)).is_err());
}

#[test]
fn ti01_full_width_evm_chain_id_is_preserved() {
    let mut ethereum = support::network(ChainNamespace::Ethereum);
    ethereum.numeric_id = Some(U256::MAX);
    let encoded = encode_chain_identity(&ethereum).unwrap();
    assert_eq!(&encoded[encoded.len() - 32..], &[255; 32]);
}

#[test]
fn ti01_identity_matches_reviewed_byte_vector() {
    let fixture: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/b0-binding-v1.json")).unwrap();
    for namespace in [
        ChainNamespace::Eve,
        ChainNamespace::Ethereum,
        ChainNamespace::Solana,
    ] {
        let name = match namespace {
            ChainNamespace::Eve => "eve",
            ChainNamespace::Ethereum => "ethereum",
            ChainNamespace::Solana => "solana",
        };
        assert_eq!(
            hex::encode(encode_chain_identity(&support::network(namespace)).unwrap()),
            fixture["identities"][name].as_str().unwrap()
        );
    }
}
