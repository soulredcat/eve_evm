// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::development::engine::OwnedEngine;
use anyhow::{Result, ensure};
use std::process::Child;

pub(crate) fn engine_child(engine: &mut OwnedEngine) -> Result<&mut Child> {
    ensure!(
        engine.output_log.metadata()?.len() <= 64 * 1_048_576
            && engine.error_log.metadata()?.len() <= 64 * 1_048_576,
        "owned engine diagnostic backlog limit; supervisor must stop the child"
    );
    let child = engine
        .child
        .as_mut()
        .ok_or_else(|| anyhow::anyhow!("owned engine has stopped"))?;
    ensure!(
        child.try_wait()?.is_none(),
        "owned engine exited; private diagnostics preserved"
    );
    Ok(child)
}
