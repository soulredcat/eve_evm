use alloy_primitives::U256;

/// Block-level allocation; `burn` is also the cumulative-burn counter delta.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FeeAllocation {
    pub collected: U256,
    pub burn: U256,
    pub node_pool: U256,
    pub validator_pool: U256,
}
