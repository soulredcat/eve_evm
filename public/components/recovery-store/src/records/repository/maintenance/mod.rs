// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Exclusive KEEP_ALL compaction; no pruning or finality authority.
mod compact_opaque_records;
mod required_opaque_compaction_reservation;
mod types;
mod verify_opaque_compaction_head;

pub use compact_opaque_records::compact_opaque_records;
pub use required_opaque_compaction_reservation::required_opaque_compaction_reservation;
pub use types::OpaqueCompactionRequest;
