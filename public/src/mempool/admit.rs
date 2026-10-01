// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::{MempoolHandle, PoolCommand, PoolError};
use alloy_primitives::{B256, Bytes};
use eve_evm::ValidatedTransaction;
use tokio::sync::oneshot;
impl MempoolHandle {
    pub async fn admit(
        &self,
        raw: Bytes,
        validated: ValidatedTransaction,
    ) -> Result<B256, PoolError> {
        if raw.len() > 131_072 || alloy_primitives::keccak256(&raw) != validated.hash() {
            return Err(PoolError(
                "raw transaction size or validated identity mismatch".into(),
            ));
        }
        let (sender, receiver) = oneshot::channel();
        self.commands
            .try_send(PoolCommand::Admit(raw, Box::new(validated), sender))
            .map_err(|_| PoolError("mempool command capacity exceeded or stopped".into()))?;
        receiver
            .await
            .map_err(|_| PoolError("mempool actor stopped".into()))?
    }
}
