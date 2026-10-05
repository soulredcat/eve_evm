// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{EstimatedWorkingPool, types::EstimatedAccounting};
use crate::sync::applied::AppliedError;
use std::sync::{Arc, Mutex};

pub(in crate::sync::applied) fn create_estimated_working_pool(
    limit: u64,
) -> Result<Arc<EstimatedWorkingPool>, AppliedError> {
    if limit == 0 {
        return Err(AppliedError::InvalidConfiguration);
    }
    Ok(Arc::new(EstimatedWorkingPool {
        limit,
        accounting: Mutex::new(EstimatedAccounting { bytes: 0, peak: 0 }),
    }))
}
