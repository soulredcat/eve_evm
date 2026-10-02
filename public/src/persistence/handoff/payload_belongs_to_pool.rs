// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{HandoffPool, RecoveryPayload};
use std::sync::Arc;

pub(in crate::persistence) fn payload_belongs_to_pool(
    payload: &RecoveryPayload,
    pool: &Arc<HandoffPool>,
) -> bool {
    Arc::ptr_eq(&payload.0._lease.pool, pool)
}
