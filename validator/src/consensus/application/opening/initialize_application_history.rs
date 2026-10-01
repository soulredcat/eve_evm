// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Result, ensure};
use eve_storage::state::{HistoryReadBudget, StateRepository, ensure_history_index};

pub(super) fn initialize_application_history(
    repository: &mut StateRepository,
    maximum_bytes: usize,
) -> Result<()> {
    for _ in 0..4096 {
        let status = ensure_history_index(
            repository,
            HistoryReadBudget {
                maximum_block_bytes: maximum_bytes,
                maximum_rebuild_blocks: 256,
                maximum_index_batch_bytes: maximum_bytes,
            },
        )?;
        if status.complete {
            return Ok(());
        }
    }
    ensure!(
        false,
        "application retained history bootstrap exceeded bounded passes"
    );
    Ok(())
}
