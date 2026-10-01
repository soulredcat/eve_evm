// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Bounded synced opaque history. Local integrity is not authenticated finality or signer policy.

mod budget;
mod encoding;
mod hashing;
mod repository;
#[cfg(test)]
mod tests;
mod types;

pub use budget::{development_opaque_record_budget, validate_opaque_record_budget};
pub use repository::{
    compare_and_append_opaque_records, opaque_record_cursor, open_opaque_record_repository,
    read_opaque_record,
};
pub use types::{
    OpaqueRecord, OpaqueRecordAck, OpaqueRecordBudget, OpaqueRecordCursor, OpaqueRecordDisposition,
    OpaqueRecordIdentity, OpaqueRecordRepository,
};
