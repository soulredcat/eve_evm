use super::StateView;
use crate::StateAccount;
use alloy_primitives::Address;

pub fn read_account(view: &StateView, address: Address) -> Option<&StateAccount> {
    view.state.accounts.get(&address)
}
