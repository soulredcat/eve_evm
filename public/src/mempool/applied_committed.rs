// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::{MempoolHandle, MempoolHead, PoolError};
use crate::sync::applied::AppliedPublication;
use std::sync::Arc;

impl MempoolHandle {
    pub async fn applied_committed(&self, head: Arc<AppliedPublication>) -> Result<(), PoolError> {
        self.update_mempool_head(Arc::new(MempoolHead::Applied(head)))
            .await
    }
}
