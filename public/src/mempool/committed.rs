// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::{MempoolHandle, MempoolHead, PoolError};
use eve_state::StateCommit;
use std::sync::Arc;
impl MempoolHandle {
    pub async fn committed(&self, head: Arc<StateCommit>) -> Result<(), PoolError> {
        self.update_mempool_head(Arc::new(MempoolHead::Local(head)))
            .await
    }
}
