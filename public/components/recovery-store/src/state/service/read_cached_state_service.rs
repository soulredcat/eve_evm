// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::StateService;
use crate::state::ImmutableStateView;
use anyhow::{Context, Result, anyhow, ensure};
use std::sync::{Arc, atomic::Ordering};
/// Only inspect the current RAM view; caller may reserve I/O buffers before a miss.
pub fn read_cached_state_service(
    service: &StateService,
) -> Result<Option<Arc<ImmutableStateView>>> {
    ensure!(
        !service.reader.fence.load(Ordering::Acquire),
        "state service fenced"
    );
    let publication = service
        .reader
        .publication
        .read()
        .map_err(|_| anyhow!("durable publication poisoned"))?
        .clone()
        .context("durable publication missing")?;
    let cache = service
        .cache
        .read()
        .map_err(|_| anyhow!("state cache poisoned"))?;
    Ok(cache
        .as_ref()
        .filter(|view| {
            view.database_sequence == publication.database_sequence
                && view.commit.target == publication.store_head
        })
        .map(Arc::clone))
}
