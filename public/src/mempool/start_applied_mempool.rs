// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    start_mempool_head::start_mempool_head,
    types::{MempoolHandle, MempoolHead, MempoolLimits},
};
use crate::sync::applied::AppliedPublication;
use std::sync::Arc;

pub fn start_applied_mempool(
    head: Arc<AppliedPublication>,
    limits: MempoolLimits,
) -> MempoolHandle {
    start_mempool_head(Arc::new(MempoolHead::Applied(head)), limits)
}
