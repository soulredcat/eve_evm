// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

/// Encoded-record/read limits include all 88 metadata bytes. Batch limits include RocksDB framing/head.
/// These bound decoded records and configured DB resources, not total OS page cache or physical disk bytes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OpaqueRecordBudget {
    pub maximum_record_bytes: usize,
    pub maximum_batch_bytes: usize,
    pub maximum_batch_records: usize,
    pub maximum_read_bytes: usize,
    pub maximum_retained_records: u64,
    pub block_cache_bytes: usize,
    pub write_buffer_bytes: usize,
    pub write_buffer_count: i32,
    pub maximum_background_jobs: i32,
    pub maximum_open_files: i32,
}
