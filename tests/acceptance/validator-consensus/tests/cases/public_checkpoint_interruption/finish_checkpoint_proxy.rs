// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::CheckpointProxy;
use anyhow::Result;
use std::sync::atomic::Ordering;

pub(super) fn finish_checkpoint_proxy(proxy: &mut CheckpointProxy) -> Result<usize> {
    proxy.cancelled.store(true, Ordering::Release);
    let _ = proxy.release.try_send(());
    match proxy.thread.take() {
        Some(thread) => thread
            .join()
            .map_err(|_| anyhow::anyhow!("CHECKPOINT_PROXY_PANICKED"))?,
        None => Ok(0),
    }
}
