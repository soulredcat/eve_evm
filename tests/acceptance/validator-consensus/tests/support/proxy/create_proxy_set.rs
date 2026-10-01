// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::{ProxySet, ProxyState};
use anyhow::Result;
use std::{
    net::TcpListener,
    sync::{Arc, Mutex, atomic::AtomicBool},
};
impl ProxySet {
    pub fn create(count: usize) -> Result<Self> {
        let listeners: Vec<_> = (0..count)
            .map(|_| TcpListener::bind("127.0.0.1:0"))
            .collect::<std::io::Result<_>>()?;
        let addresses = listeners
            .iter()
            .map(TcpListener::local_addr)
            .collect::<std::io::Result<_>>()?;
        Ok(Self {
            addresses,
            listeners,
            state: Arc::new(ProxyState {
                next_connection: std::sync::atomic::AtomicUsize::new(0),
                stop: AtomicBool::new(false),
                partition: AtomicBool::new(false),
                owners: Mutex::new(vec![None; count]),
                connections: Mutex::new(Vec::new()),
                workers: Mutex::new(Vec::new()),
                accepted: Mutex::new(Vec::new()),
                errors: Mutex::new(Vec::new()),
            }),
            acceptors: Vec::new(),
        })
    }
    pub fn register(&self, index: usize, pid: u32) {
        self.state.owners.lock().unwrap()[index] = Some(pid);
    }
    pub fn unregister(&self, index: usize) {
        self.state.owners.lock().unwrap()[index] = None;
    }
}
