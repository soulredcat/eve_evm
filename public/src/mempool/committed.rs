// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::{MempoolHandle, PoolCommand, PoolError};
use eve_state::StateCommit;
use std::sync::Arc;
use tokio::sync::oneshot;
impl MempoolHandle {
    pub async fn committed(&self, head: Arc<StateCommit>) -> Result<(), PoolError> {
        let (sender, receiver) = oneshot::channel();
        self.commands
            .try_send(PoolCommand::Committed(head, sender))
            .map_err(|_| PoolError("mempool command capacity exceeded or stopped".into()))?;
        receiver
            .await
            .map_err(|_| PoolError("mempool actor stopped".into()))?
    }
}
