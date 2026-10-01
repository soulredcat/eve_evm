// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_consensus_comet::wire::tendermint::types::{BlockId, CanonicalBlockId, PartSetHeader};

/// Mechanical projection for revalidation through the canonical native codec.
pub(super) fn restore_block_id(value: Option<CanonicalBlockId>) -> Option<BlockId> {
    value.map(|block| BlockId {
        hash: block.hash,
        part_set_header: block.part_set_header.map(|parts| PartSetHeader {
            total: parts.total,
            hash: parts.hash,
        }),
    })
}
