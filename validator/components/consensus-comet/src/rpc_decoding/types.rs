// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::wire::tendermint::types::{BlockId, Commit, Header};

/// Bounded native RPC parser; canonical certificate admission remains authoritative.
pub(super) const MAXIMUM_DECODED_VALIDATORS: usize = 64;
/// Untrusted native RPC data, not an authenticated finality capability.
pub struct NativeBlock {
    pub header: Header,
    pub last_commit: Option<Commit>,
    pub block_id: BlockId,
    pub transactions: Vec<Vec<u8>>,
}
