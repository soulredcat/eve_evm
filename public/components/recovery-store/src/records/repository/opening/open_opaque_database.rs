// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::records::OpaqueRecordBudget;
use anyhow::{Context, Result};
use rocksdb::{BlockBasedOptions, Cache, DB, DBCompressionType, Options};
use std::path::Path;

pub(in crate::records) fn open_opaque_database(
    path: &Path,
    budget: &OpaqueRecordBudget,
    fresh: bool,
) -> Result<DB> {
    let mut options = Options::default();
    options.create_if_missing(fresh);
    options.set_error_if_exists(fresh);
    options.set_compression_type(DBCompressionType::None);
    options.set_write_buffer_size(budget.write_buffer_bytes);
    options.set_max_write_buffer_number(budget.write_buffer_count);
    options.set_max_background_jobs(budget.maximum_background_jobs);
    options.set_max_open_files(budget.maximum_open_files);
    let mut table = BlockBasedOptions::default();
    table.set_block_cache(&Cache::new_lru_cache(budget.block_cache_bytes));
    options.set_block_based_table_factory(&table);
    DB::open(&options, path).context("open exclusive opaque-record namespace")
}
