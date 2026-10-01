// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::{MempoolHandle, PoolCommand, PoolEntry, PoolError};

use tokio::sync::oneshot;
impl MempoolHandle {
    pub async fn select(&self) -> Result<Vec<PoolEntry>, PoolError> {
        let (sender, receiver) = oneshot::channel();
        self.commands
            .try_send(PoolCommand::Select(sender))
            .map_err(|_| PoolError("mempool command capacity exceeded or stopped".into()))?;
        receiver
            .await
            .map_err(|_| PoolError("mempool actor stopped".into()))?
    }
}
