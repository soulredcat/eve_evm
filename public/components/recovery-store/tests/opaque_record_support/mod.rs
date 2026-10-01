// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_storage::records::{
    OpaqueRecordBudget, OpaqueRecordIdentity, development_opaque_record_budget,
};

pub fn identity() -> OpaqueRecordIdentity {
    // Inert test namespace bytes, not live consensus keys or enrolled authority.
    OpaqueRecordIdentity {
        genesis_hash: [1; 32],
        owner: [2; 32],
        domain: [3; 32],
    }
}

pub fn budget() -> OpaqueRecordBudget {
    OpaqueRecordBudget {
        maximum_record_bytes: 512,
        maximum_read_bytes: 512,
        maximum_batch_bytes: 4096,
        maximum_batch_records: 8,
        maximum_retained_records: 8,
        block_cache_bytes: 65_536,
        write_buffer_bytes: 65_536,
        ..development_opaque_record_budget()
    }
}
