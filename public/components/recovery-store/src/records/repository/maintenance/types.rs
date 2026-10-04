// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::records::{OpaqueRecordCursor, OpaqueRecordIdentity};

/// Local KEEP_ALL maintenance admission. This grants no authority to prune,
/// authenticate history, release votes or change an acknowledged logical head.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OpaqueCompactionRequest {
    pub expected_identity: OpaqueRecordIdentity,
    pub expected_head: OpaqueRecordCursor,
    pub maximum_records: u64,
}
