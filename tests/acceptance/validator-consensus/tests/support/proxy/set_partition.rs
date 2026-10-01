// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::ProxySet;
use std::{net::Shutdown, sync::atomic::Ordering};
impl ProxySet {
    pub fn partition(&self, cut: bool) -> usize {
        let connections = self.state.connections.lock().unwrap();
        self.state.partition.store(cut, Ordering::Release);
        if !cut {
            return 0;
        }
        let mut closed = 0;
        for connection in connections
            .iter()
            .filter(|connection| (connection.source < 2) != (connection.target < 2))
        {
            let _ = connection.incoming.shutdown(Shutdown::Both);
            let _ = connection.outgoing.shutdown(Shutdown::Both);
            closed += 1;
        }
        closed
    }
    pub fn observed_edges(&self) -> Vec<(usize, usize)> {
        self.state.accepted.lock().unwrap().clone()
    }
    pub fn shutdown(&mut self) {
        self.state.stop.store(true, Ordering::Release);
        for connection in self.state.connections.lock().unwrap().iter() {
            let _ = connection.incoming.shutdown(Shutdown::Both);
            let _ = connection.outgoing.shutdown(Shutdown::Both);
        }
        for acceptor in self.acceptors.drain(..) {
            let _ = acceptor.join();
        }
        for worker in self.state.workers.lock().unwrap().drain(..) {
            let _ = worker.join();
        }
    }
}
