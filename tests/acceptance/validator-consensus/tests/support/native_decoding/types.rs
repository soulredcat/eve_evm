// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_consensus_comet::wire::tendermint::types::{BlockId, Commit, Header};

/// Independent test parser resource bound; canonical certificate admission remains authoritative.
pub(super) const MAXIMUM_DECODED_VALIDATORS: usize = 64;
/// Test data container, not a protobuf Block or authenticated finality capability.
pub(crate) struct NativeBlock {
    pub header: Header,
    pub last_commit: Option<Commit>,
    pub block_id: BlockId,
    pub transactions: Vec<Vec<u8>>,
}
