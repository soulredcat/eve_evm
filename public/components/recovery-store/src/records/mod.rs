// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Bounded synced opaque history. Local integrity is not authenticated finality or signer policy.

mod budget;
mod encoding;
mod hashing;
mod repository;
pub mod segmented;
#[cfg(test)]
mod tests;
mod types;

pub use budget::{development_opaque_record_budget, validate_opaque_record_budget};
pub use repository::{
    OpaqueCompactionRequest, compact_opaque_records, compare_and_append_opaque_records,
    opaque_record_bootstrap_cursor, opaque_record_budget, opaque_record_cursor,
    opaque_record_identity, open_opaque_record_repository, prospective_opaque_record_cursor,
    read_opaque_record, required_opaque_compaction_reservation,
};
pub use types::{
    OpaqueRecord, OpaqueRecordAck, OpaqueRecordBudget, OpaqueRecordCursor, OpaqueRecordDisposition,
    OpaqueRecordIdentity, OpaqueRecordRepository,
};
