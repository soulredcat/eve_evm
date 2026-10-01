// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::{MempoolHandle, PoolCommand, PoolError};
use std::time::Instant;
use tokio::sync::oneshot;
impl MempoolHandle {
    pub async fn evict_at(&self, now: Instant) -> Result<usize, PoolError> {
        let (sender, receiver) = oneshot::channel();
        self.commands
            .try_send(PoolCommand::Evict(now, sender))
            .map_err(|_| PoolError("mempool command capacity exceeded or stopped".into()))?;
        let result = receiver
            .await
            .map_err(|_| PoolError("mempool actor stopped".into()))?;
        Ok(result)
    }
}
