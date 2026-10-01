// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use alloy_primitives::{Address, B256};
use serde_json::Value;
use std::collections::BTreeSet;
pub(crate) struct LogFilter {
    pub addresses: Option<BTreeSet<Address>>,
    pub topics: Vec<Option<BTreeSet<B256>>>,
    pub from: Option<Value>,
    pub to: Option<Value>,
    pub block_hash: Option<B256>,
}
