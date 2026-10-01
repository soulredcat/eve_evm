// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::OpaqueRecordCursor;

/// Caller-selected payload bytes; storage does not decode signatures, heights, rounds or steps.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpaqueRecord {
    pub sequence: u64,
    pub parent: OpaqueRecordCursor,
    pub payload: Vec<u8>,
    pub content_hash: [u8; 32],
}
