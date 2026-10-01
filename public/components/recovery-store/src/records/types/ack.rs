// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::OpaqueRecordCursor;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OpaqueRecordDisposition {
    NewlySynced,
    ExactReplay,
}

/// An exact replay acknowledges its original cursor without lowering the current durable head.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OpaqueRecordAck {
    pub appended: OpaqueRecordCursor,
    pub store_head: OpaqueRecordCursor,
    pub database_sequence: u64,
    pub disposition: OpaqueRecordDisposition,
}
