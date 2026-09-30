use alloy_primitives::{Address, B256};
use eve_protocol_config::{
    native::{KeyPossessionInput, encode_key_possession},
    network::SecurityProfile,
    records::GenesisHash,
};

#[test]
fn possession_bytes_bind_network_profile_owner_role_nonce_epoch_and_full_key() {
    let input = KeyPossessionInput {
        genesis: GenesisHash(B256::repeat_byte(1)),
        chain_id: 31_337,
        protocol_version: 1,
        profile: SecurityProfile::ClassicalDev,
        owner: Address::repeat_byte(2),
        role: 2,
        nonce: 7,
        key_epoch: 1,
        public_key: [3; 32],
    };
    let original = encode_key_possession(input).unwrap();
    let mut changed = input;
    changed.genesis = GenesisHash(B256::repeat_byte(2));
    assert_ne!(encode_key_possession(changed).unwrap(), original);
    changed = input;
    changed.chain_id += 1;
    assert_ne!(encode_key_possession(changed).unwrap(), original);
    changed = input;
    changed.protocol_version += 1;
    assert_ne!(encode_key_possession(changed).unwrap(), original);
    changed = input;
    changed.owner = Address::repeat_byte(4);
    assert_ne!(encode_key_possession(changed).unwrap(), original);
    changed = input;
    changed.role = 1;
    assert_ne!(encode_key_possession(changed).unwrap(), original);
    changed = input;
    changed.nonce += 1;
    assert_ne!(encode_key_possession(changed).unwrap(), original);
    changed = input;
    changed.key_epoch += 1;
    assert_ne!(encode_key_possession(changed).unwrap(), original);
    changed = input;
    changed.public_key = [4; 32];
    assert_ne!(encode_key_possession(changed).unwrap(), original);
    changed = input;
    changed.role = 0;
    assert!(encode_key_possession(changed).is_err());
    changed = input;
    changed.profile = SecurityProfile::HybridExperimental;
    assert!(encode_key_possession(changed).is_err());
}
