// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::mempool::{
    MempoolHandle,
    types::{PoolCapture, PoolCommand},
};
use tokio::sync::oneshot;

pub(super) async fn capture(pool: &MempoolHandle) -> PoolCapture {
    let (sender, receiver) = oneshot::channel();
    pool.commands
        .try_send(PoolCommand::Capture(sender))
        .map_err(|_| "mempool capture command refused")
        .unwrap();
    receiver.await.unwrap()
}
