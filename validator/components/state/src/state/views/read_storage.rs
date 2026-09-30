use super::StateView;
use alloy_primitives::{Address, U256};

pub fn read_storage(view: &StateView, address: Address, slot: U256) -> U256 {
    view.state
        .accounts
        .get(&address)
        .and_then(|account| account.storage.get(&slot))
        .copied()
        .unwrap_or(U256::ZERO)
}
