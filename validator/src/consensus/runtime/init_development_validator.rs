// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{DevelopmentValidatorInitialization, bootstrap::assemble_development_node};
use crate::development::config::DevelopmentValidatorConfig;
use anyhow::Result;

/// Initialize immutable local namespaces and native routing identity without voting startup.
pub fn init_development_validator(
    config: &DevelopmentValidatorConfig,
) -> Result<DevelopmentValidatorInitialization> {
    Ok(assemble_development_node(config.clone())?.initialization)
}
