// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    start_mempool_head::start_mempool_head,
    types::{MempoolHandle, MempoolHead, MempoolLimits},
};
use eve_state::StateCommit;
use std::sync::Arc;
pub fn start_mempool(head: Arc<StateCommit>, limits: MempoolLimits) -> MempoolHandle {
    start_mempool_head(Arc::new(MempoolHead::Local(head)), limits)
}
