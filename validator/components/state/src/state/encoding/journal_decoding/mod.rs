// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Borrowed journal scanning precedes every owned operation/blob allocation.

mod decode_borrowed_operation;
mod materialize_operation;
mod preflight_state_journal;
mod scan_record_allocations;
mod scan_version_allocations;
mod take_borrowed_bytes;
mod take_encoded_list;
mod types;

pub(crate) use decode_borrowed_operation::decode_borrowed_operation;
pub(crate) use materialize_operation::materialize_operation;
pub use preflight_state_journal::preflight_state_journal;
pub(crate) use take_borrowed_bytes::take_borrowed_bytes;
pub(crate) use take_encoded_list::take_encoded_list;
pub(crate) use types::BorrowedJournalOperation;
pub use types::JournalDecodePreflight;
