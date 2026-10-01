// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::StateView;
use crate::StateAccount;
use alloy_primitives::Address;

pub fn read_account(view: &StateView, address: Address) -> Option<&StateAccount> {
    view.state.accounts.get(&address)
}
