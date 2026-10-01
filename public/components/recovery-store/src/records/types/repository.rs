// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{OpaqueRecordBudget, OpaqueRecordCursor, OpaqueRecordIdentity};
use rocksdb::DB;
use std::sync::atomic::AtomicBool;

/// Exclusive local owner; raw database and cached acknowledgment are confined to this domain.
pub struct OpaqueRecordRepository {
    pub(in crate::records) database: DB,
    pub(in crate::records) identity: OpaqueRecordIdentity,
    pub(in crate::records) budget: OpaqueRecordBudget,
    pub(in crate::records) head: OpaqueRecordCursor,
    pub(in crate::records) fenced: AtomicBool,
    #[cfg(test)]
    pub(in crate::records) simulated_failure: Option<SimulatedOpaqueFailure>,
}

#[cfg(test)]
#[derive(Clone, Copy)]
pub(in crate::records) enum SimulatedOpaqueFailure {
    BeforeWrite,
    AfterSuccessfulSync,
}
