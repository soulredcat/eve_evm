// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{forward_connection::forward_connection, types::ProxySet};
use anyhow::Result;
use std::{net::SocketAddr, sync::atomic::Ordering, time::Duration};
impl ProxySet {
    pub fn start(&mut self, backends: Vec<SocketAddr>) -> Result<()> {
        for (target, listener) in self.listeners.iter().enumerate() {
            let listener = listener.try_clone()?;
            listener.set_nonblocking(true)?;
            let state = self.state.clone();
            let backend = backends[target];
            self.acceptors.push(std::thread::spawn(move || {
                while !state.stop.load(Ordering::Acquire) {
                    match listener.accept() {
                        Ok((incoming, remote)) => {
                            let mut workers = state.workers.lock().unwrap();
                            workers.retain(|worker| !worker.is_finished());
                            if workers.len() >= 128 {
                                continue;
                            }
                            let shared = state.clone();
                            let worker = std::thread::spawn(move || {
                                if let Err(error) = forward_connection(
                                    shared.clone(),
                                    target,
                                    backend,
                                    incoming,
                                    remote,
                                ) {
                                    let mut errors = shared.errors.lock().unwrap();
                                    if errors.len() < 16 {
                                        errors.push(error.to_string());
                                    }
                                }
                            });
                            workers.push(worker);
                        }
                        Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                            std::thread::sleep(Duration::from_millis(10))
                        }
                        Err(_) => break,
                    }
                }
            }));
        }
        Ok(())
    }
}
