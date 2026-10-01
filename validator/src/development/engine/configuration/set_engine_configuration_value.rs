// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Result, ensure};

pub(in crate::development::engine) fn set_engine_configuration_value(
    configuration: &mut toml::Value,
    path: &[&str],
    value: toml::Value,
) -> Result<()> {
    ensure!(!path.is_empty(), "empty native configuration path");
    let mut target = configuration;
    for key in &path[..path.len() - 1] {
        target = target
            .get_mut(*key)
            .ok_or_else(|| anyhow::anyhow!("pinned native configuration section missing"))?;
    }
    let target = target
        .as_table_mut()
        .ok_or_else(|| anyhow::anyhow!("native configuration target is not a table"))?
        .get_mut(path[path.len() - 1])
        .ok_or_else(|| anyhow::anyhow!("pinned native configuration field missing"))?;
    ensure!(
        std::mem::discriminant(target) == std::mem::discriminant(&value),
        "native configuration field type mismatch"
    );
    *target = value;
    Ok(())
}
