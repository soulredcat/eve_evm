// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::{
    net::SocketAddr,
    sync::{
        Arc,
        atomic::AtomicBool,
        mpsc::{Receiver, SyncSender},
    },
    thread::JoinHandle,
    time::Instant,
};

pub(super) struct CheckpointProxy {
    pub(super) address: SocketAddr,
    pub(super) reached: Receiver<usize>,
    pub(super) release: SyncSender<()>,
    pub(super) cancelled: Arc<AtomicBool>,
    pub(super) thread: Option<JoinHandle<anyhow::Result<usize>>>,
}

pub(super) struct ProxyControl {
    pub(super) reached: SyncSender<usize>,
    pub(super) release: Receiver<()>,
    pub(super) cancelled: Arc<AtomicBool>,
    pub(super) deadline: Instant,
}

pub(super) const MAXIMUM_PROXY_REQUEST_BYTES: usize = 1_048_576;
pub(super) const MAXIMUM_PROXY_RESPONSE_BYTES: usize = 262_144;
pub(super) const MAXIMUM_PROXY_CONNECTIONS: usize = 3;
