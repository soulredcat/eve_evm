// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    StorageAdmissionError,
    types::{StorageAdmissionAccounting, StorageAdmissionPool},
};
use eve_node_policy::{PublicBudget, validate_public_budget};
use std::sync::{Arc, Mutex};
pub(in crate::sync::applied) fn create_storage_admission_pool(
    budget: PublicBudget,
    working: &Arc<crate::sync::applied::resources::EstimatedWorkingPool>,
) -> Result<Arc<StorageAdmissionPool>, StorageAdmissionError> {
    validate_public_budget(budget).map_err(|_| StorageAdmissionError::InvalidConfiguration)?;
    let control_bytes = super::required_storage_admission_control_reservation()?;
    let control = super::super::reserve_estimated_working(working, control_bytes)
        .map_err(super::map_working_control_error::map_working_control_error)?;
    Ok(Arc::new(StorageAdmissionPool {
        _control: control,
        reads_limit: budget.concurrent_storage_reads,
        staging_limit: budget.snapshot_staging_bytes,
        accounting: Mutex::new(StorageAdmissionAccounting {
            reads: 0,
            peak_reads: 0,
            staging: 0,
            peak_staging: 0,
        }),
    }))
}
