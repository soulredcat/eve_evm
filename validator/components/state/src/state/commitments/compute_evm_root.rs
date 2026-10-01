// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::collections::BTreeMap;

use alloy_primitives::Address;
use eve_protocol_config::records::EvmStateRoot;

use super::build_account_trie;
use crate::StateAccount;

/// Canonical world-state root; callers supply complete, validated logical accounts.
pub fn compute_evm_root(accounts: &BTreeMap<Address, StateAccount>) -> EvmStateRoot {
    EvmStateRoot(build_account_trie(accounts, Vec::new()).0)
}
