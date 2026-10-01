// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::state::StateStorageBudget;
use anyhow::{Context, Result};
use rocksdb::{BlockBasedOptions, Cache, DB, DBCompressionType, Options};
use std::path::Path;

pub(crate) fn open_state_database(path: &Path, budget: &StateStorageBudget) -> Result<DB> {
    let mut options = Options::default();
    options.create_if_missing(true);
    options.set_compression_type(DBCompressionType::None);
    options.set_write_buffer_size(budget.database.write_buffer_bytes);
    options.set_max_write_buffer_number(budget.database.write_buffer_count);
    options.set_max_background_jobs(budget.database.max_background_jobs);
    options.set_max_open_files(budget.database.max_open_files);
    let mut table = BlockBasedOptions::default();
    table.set_block_cache(&Cache::new_lru_cache(budget.database.block_cache_bytes));
    options.set_block_based_table_factory(&table);
    DB::open(&options, path).context("open complete-state recovery namespace")
}
