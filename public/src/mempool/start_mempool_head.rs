// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    run_mempool::run_mempool,
    types::{MempoolHandle, MempoolHead, MempoolLimits, PoolState},
};
use std::{collections::BTreeMap, sync::Arc};
use tokio::sync::mpsc;

pub(crate) fn start_mempool_head(head: Arc<MempoolHead>, limits: MempoolLimits) -> MempoolHandle {
    let (commands, receiver) = mpsc::channel(256);
    let state = PoolState {
        head,
        limits,
        bytes: 0,
        entries: BTreeMap::new(),
        hashes: BTreeMap::new(),
    };
    tokio::spawn(run_mempool(state, receiver));
    MempoolHandle { commands }
}
