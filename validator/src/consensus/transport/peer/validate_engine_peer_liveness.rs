// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{AuthenticatedEnginePeer, validate_linux_transport};
use std::io;
#[cfg(target_os = "linux")]
use std::os::fd::AsFd;

/// Query the original process FD without reaping; a saved proof cannot authenticate a reused PID.
pub(in crate::consensus) fn validate_engine_peer_liveness(
    peer: &AuthenticatedEnginePeer,
) -> io::Result<()> {
    validate_linux_transport()?;
    #[cfg(target_os = "linux")]
    {
        let status = rustix::process::waitid(
            rustix::process::WaitId::PidFd(peer.process.as_fd()),
            rustix::process::WaitIdOptions::EXITED
                | rustix::process::WaitIdOptions::NOHANG
                | rustix::process::WaitIdOptions::NOWAIT,
        )
        .map_err(|_| {
            io::Error::new(
                io::ErrorKind::BrokenPipe,
                "engine process lifecycle is unavailable",
            )
        })?;
        if status.is_some() {
            return Err(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "authenticated engine process exited",
            ));
        }
        if peer.uid != rustix::process::geteuid().as_raw()
            || peer.gid != rustix::process::getegid().as_raw()
        {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "engine effective identity changed",
            ));
        }
        if crate::development::engine::engine_process_image_identity(peer.pid)? != peer.image {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "authenticated engine executable changed",
            ));
        }
        let status = rustix::process::waitid(
            rustix::process::WaitId::PidFd(peer.process.as_fd()),
            rustix::process::WaitIdOptions::EXITED
                | rustix::process::WaitIdOptions::NOHANG
                | rustix::process::WaitIdOptions::NOWAIT,
        )
        .map_err(|_| {
            io::Error::new(
                io::ErrorKind::BrokenPipe,
                "engine process lifecycle is unavailable",
            )
        })?;
        if status.is_some() {
            return Err(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "authenticated engine process exited",
            ));
        }
    }
    Ok(())
}
