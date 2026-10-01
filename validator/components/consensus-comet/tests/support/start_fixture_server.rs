// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::{
    io,
    net::{Shutdown, SocketAddr, TcpListener, TcpStream},
    path::PathBuf,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self, JoinHandle},
    time::Duration,
};

use anyhow::Result;
use eve_consensus_comet::wire::framing::{read_abci_request, write_abci_response};

use super::{FixtureState, handle_fixture_request::handle_fixture_request};

pub struct FixtureServer {
    pub address: SocketAddr,
    stopped: Arc<AtomicBool>,
    connections: Arc<Mutex<Vec<TcpStream>>>,
    join: Option<JoinHandle<()>>,
}

impl Drop for FixtureServer {
    fn drop(&mut self) {
        self.stopped.store(true, Ordering::Release);
        for connection in self.connections.lock().unwrap().iter() {
            let _ = connection.shutdown(Shutdown::Both);
        }
        if let Some(join) = self.join.take() {
            join.join().expect("fixture server thread failed");
        }
    }
}

pub fn start_fixture_server(
    address: SocketAddr,
    state: Arc<Mutex<FixtureState>>,
    checkpoint: PathBuf,
) -> Result<FixtureServer> {
    let listener = TcpListener::bind(address)?;
    let address = listener.local_addr()?;
    listener.set_nonblocking(true)?;
    let stopped = Arc::new(AtomicBool::new(false));
    let stop_accept = stopped.clone();
    let open_connections = Arc::new(Mutex::new(Vec::new()));
    let shutdown_connections = open_connections.clone();
    let join = thread::spawn(move || {
        let mut connections = Vec::new();
        while !stop_accept.load(Ordering::Acquire) {
            let (mut stream, _) = match listener.accept() {
                Ok(connection) => connection,
                Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(5));
                    continue;
                }
                Err(error) => panic!("fixture accept failed: {error}"),
            };
            let state = state.clone();
            let checkpoint = checkpoint.clone();
            let stopped = stop_accept.clone();
            shutdown_connections
                .lock()
                .unwrap()
                .push(stream.try_clone().unwrap());
            connections.push(thread::spawn(move || {
                while !stopped.load(Ordering::Acquire) {
                    let request = match read_abci_request(&mut stream, 1_048_576) {
                        Ok(request) => request,
                        Err(error)
                            if matches!(
                                error.kind(),
                                io::ErrorKind::UnexpectedEof
                                    | io::ErrorKind::ConnectionReset
                                    | io::ErrorKind::ConnectionAborted
                            ) =>
                        {
                            break;
                        }
                        Err(error) => panic!("fixture framing failed: {error}"),
                    };
                    let response =
                        handle_fixture_request(request, &mut state.lock().unwrap(), &checkpoint)
                            .expect("fixture request failed");
                    if write_abci_response(&mut stream, &response, 1_048_576).is_err() {
                        break;
                    }
                }
            }));
        }
        for connection in connections {
            connection.join().expect("fixture connection failed");
        }
    });
    Ok(FixtureServer {
        address,
        stopped,
        connections: open_connections,
        join: Some(join),
    })
}
