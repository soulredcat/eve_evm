// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::{MempoolHandle, PoolCommand, PoolError};
use alloy_primitives::Address;
use tokio::sync::oneshot;
impl MempoolHandle {
    pub async fn pending_nonce(&self, address: Address) -> Result<u64, PoolError> {
        let (sender, receiver) = oneshot::channel();
        self.commands
            .try_send(PoolCommand::PendingNonce(address, sender))
            .map_err(|_| PoolError("mempool command capacity exceeded or stopped".into()))?;
        let result = receiver
            .await
            .map_err(|_| PoolError("mempool actor stopped".into()))?;
        Ok(result)
    }
}
