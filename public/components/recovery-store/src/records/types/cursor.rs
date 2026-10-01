// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

/// Sequence zero binds the immutable namespace; subsequent cursors bind complete opaque records.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OpaqueRecordCursor {
    pub sequence: u64,
    pub content_hash: [u8; 32],
}
