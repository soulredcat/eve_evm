// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::{MempoolHandle, PoolCommand, PoolEntry, PoolError};
use alloy_primitives::B256;
use tokio::sync::oneshot;
impl MempoolHandle {
    pub async fn find(&self, hash: B256) -> Result<Option<PoolEntry>, PoolError> {
        let (sender, receiver) = oneshot::channel();
        self.commands
            .try_send(PoolCommand::Find(hash, sender))
            .map_err(|_| PoolError("mempool command capacity exceeded or stopped".into()))?;
        let result = receiver
            .await
            .map_err(|_| PoolError("mempool actor stopped".into()))?;
        Ok(result)
    }
}
