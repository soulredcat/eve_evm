// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::{
    net::{SocketAddr, TcpListener, TcpStream},
    sync::{Arc, Mutex, atomic::AtomicBool},
    thread::JoinHandle,
};
pub(crate) struct Connection {
    pub id: usize,
    pub source: usize,
    pub target: usize,
    pub incoming: TcpStream,
    pub outgoing: TcpStream,
}
pub(crate) struct ProxyState {
    pub next_connection: std::sync::atomic::AtomicUsize,
    pub stop: AtomicBool,
    pub partition: AtomicBool,
    pub owners: Mutex<Vec<Option<u32>>>,
    pub connections: Mutex<Vec<Connection>>,
    pub workers: Mutex<Vec<JoinHandle<()>>>,
    pub accepted: Mutex<Vec<(usize, usize)>>,
    pub errors: Mutex<Vec<String>>,
}
pub(crate) struct ProxySet {
    pub addresses: Vec<SocketAddr>,
    pub listeners: Vec<TcpListener>,
    pub state: Arc<ProxyState>,
    pub acceptors: Vec<JoinHandle<()>>,
}
