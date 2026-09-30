#[path = "support/mod.rs"]
mod support;

use eve_protocol_config::{
    genesis::{encode_development_genesis, hash_development_genesis},
    network::LaunchMode,
};

#[test]
fn genesis_digest_ignores_input_insertion_order_but_binds_timestamp_and_funded_state() {
    let original = support::genesis();
    let mut shuffled = original.clone();
    shuffled.accounts.reverse();
    shuffled.validators.reverse();
    assert_eq!(
        encode_development_genesis(LaunchMode::Development, &original).unwrap(),
        encode_development_genesis(LaunchMode::Development, &shuffled).unwrap()
    );
    let hash = hash_development_genesis(LaunchMode::Development, &original).unwrap();
    shuffled.initial_timestamp += 1;
    assert_ne!(
        hash,
        hash_development_genesis(LaunchMode::Development, &shuffled).unwrap()
    );
    shuffled = original.clone();
    shuffled.accounts[0].nonce = 1;
    assert_ne!(
        hash,
        hash_development_genesis(LaunchMode::Development, &shuffled).unwrap()
    );
}
