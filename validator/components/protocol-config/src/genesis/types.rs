use alloy_primitives::{Address, B256, Bytes, U256};

use crate::network::SecurityProfile;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GenesisAccount {
    pub address: Address,
    pub funded_balance: U256,
    pub nonce: u64,
    pub code: Bytes,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GenesisValidator {
    pub owner: Address,
    pub classical_public_key: [u8; 32],
    pub self_bond: U256,
    pub voting_power: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScheduledUpgrade {
    pub activation_height: u64,
    pub protocol_version: u32,
    pub profile: SecurityProfile,
    pub code_digest: B256,
    pub migration_id: Bytes,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EconomicsParameters {
    pub burn_bps: u16,
    pub node_pool_bps: u16,
    pub validator_pool_bps: u16,
    pub epoch_blocks: u64,
    pub maximum_validators: u16,
    pub validator_self_bond: U256,
    pub node_self_bond: U256,
    pub default_commission_bps: u16,
    pub maximum_commission_bps: u16,
    pub unbonding_seconds: u64,
    pub unbonding_blocks: u64,
    pub maximum_tasks_per_block: u16,
    pub node_availability_bps: u16,
    pub double_sign_slash_bps: u16,
    pub downtime_window_blocks: u64,
    pub minimum_participation_bps: u16,
    pub node_service_slashing: bool,
    pub default_issuance: U256,
}

/// User allocations exclude reserved escrows. Genesis automatically allocates
/// the sum of recorded self-bonds to f100, without counting it twice in supply.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DevelopmentGenesis {
    pub schema_version: u32,
    pub protocol_version: u32,
    pub network_name: String,
    pub evm_chain_id: u64,
    pub initial_timestamp: u64,
    pub profile: SecurityProfile,
    pub economics: EconomicsParameters,
    pub accounts: Vec<GenesisAccount>,
    pub validators: Vec<GenesisValidator>,
    pub upgrades: Vec<ScheduledUpgrade>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GenesisError {
    ProductionNotAuthorized,
    UnsupportedDevelopmentProfile,
    InvalidEconomics,
    InvalidAccounts,
    ReservedAddress,
    InvalidValidators,
    InvalidKey,
    UnderfundedBond,
    InvalidVotingPower,
    InvalidUpgrade,
    ArithmeticOverflow,
}
