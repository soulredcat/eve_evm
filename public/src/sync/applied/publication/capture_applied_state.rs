// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::sync::applied::{AppliedError, AppliedPublication, AppliedReader};
use std::sync::Arc;

/// Clone one coherent charged publication, dropping the guard before all caller work.
pub fn capture_applied_state(
    reader: &AppliedReader,
) -> Result<Arc<AppliedPublication>, AppliedError> {
    let publication = reader
        .publication
        .read()
        .map_err(|_| AppliedError::PublicationUnavailable)?;
    Ok(Arc::clone(&publication))
}
