use crate::{network::SecurityProfile, records::GenesisHash};
use alloy_primitives::Address;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KeyPossessionInput {
    pub genesis: GenesisHash,
    pub chain_id: u64,
    pub protocol_version: u32,
    pub profile: SecurityProfile,
    pub owner: Address,
    pub role: u8,
    pub nonce: u64,
    pub key_epoch: u64,
    pub public_key: [u8; 32],
}
