// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    relay_checkpoint_bytes::relay_checkpoint_bytes,
    types::{MAXIMUM_PROXY_REQUEST_BYTES, MAXIMUM_PROXY_RESPONSE_BYTES},
};
use anyhow::Result;
use std::{
    net::{Shutdown, SocketAddr, TcpStream},
    sync::{Arc, atomic::AtomicBool},
    time::{Duration, Instant},
};

pub(super) fn forward_checkpoint_connection(
    downstream: TcpStream,
    address: SocketAddr,
    cancelled: &Arc<AtomicBool>,
    total_deadline: Instant,
) -> Result<()> {
    let deadline = total_deadline.min(Instant::now() + Duration::from_secs(15));
    let upstream = TcpStream::connect_timeout(&address, Duration::from_secs(2))?;
    for stream in [&downstream, &upstream] {
        stream.set_read_timeout(Some(Duration::from_millis(100)))?;
        stream.set_write_timeout(Some(Duration::from_secs(2)))?;
    }
    let request_input = downstream.try_clone()?;
    let request_output = upstream.try_clone()?;
    let response_input = upstream.try_clone()?;
    let response_output = downstream.try_clone()?;
    let request_cancelled = Arc::clone(cancelled);
    let request = std::thread::spawn(move || {
        relay_checkpoint_bytes(
            request_input,
            request_output,
            &request_cancelled,
            deadline,
            MAXIMUM_PROXY_REQUEST_BYTES,
        )
    });
    let response = relay_checkpoint_bytes(
        response_input,
        response_output,
        cancelled,
        deadline,
        MAXIMUM_PROXY_RESPONSE_BYTES,
    );
    let _ = upstream.shutdown(Shutdown::Both);
    let _ = downstream.shutdown(Shutdown::Both);
    let request_result = request
        .join()
        .map_err(|_| anyhow::anyhow!("CHECKPOINT_PROXY_REQUEST_PANICKED"))?;
    response?;
    // Response completion deliberately closes the still-reading request direction.
    if !cancelled.load(std::sync::atomic::Ordering::Acquire) {
        match request_result {
            Ok(()) => {}
            Err(error)
                if error.downcast_ref::<std::io::Error>().is_some_and(|error| {
                    matches!(
                        error.kind(),
                        std::io::ErrorKind::NotConnected
                            | std::io::ErrorKind::ConnectionReset
                            | std::io::ErrorKind::BrokenPipe
                    )
                }) => {}
            Err(error) => return Err(error),
        }
    }
    Ok(())
}
