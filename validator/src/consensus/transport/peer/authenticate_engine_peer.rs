// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{AuthenticatedEnginePeer, EngineChannel, validate_linux_transport};
use crate::development::engine::{VerifiedEngineImage, validate_engine_image_binding};
use anyhow::{Result, ensure};
use std::{os::unix::net::UnixStream, process::Child, time::Duration};

/// Authenticate only an accepted real stream against the active task-owned child and verified launch digest.
/// No blocking socket I/O occurs here; the caller releases its child lock before frame I/O.
pub(in crate::consensus) fn authenticate_engine_peer(
    stream: UnixStream,
    child: &mut Child,
    verified_image: &VerifiedEngineImage,
    channel: EngineChannel,
    read_timeout: Duration,
    write_timeout: Duration,
) -> Result<AuthenticatedEnginePeer> {
    validate_linux_transport()?;
    #[cfg(target_os = "linux")]
    {
        ensure!(child.try_wait()?.is_none(), "engine child is not active");
        ensure!(
            !read_timeout.is_zero()
                && !write_timeout.is_zero()
                && read_timeout <= Duration::from_secs(60)
                && write_timeout <= Duration::from_secs(60),
            "invalid bounded engine socket deadlines"
        );
        let credentials = rustix::net::sockopt::socket_peercred(&stream)?;
        let pid = u32::try_from(credentials.pid.as_raw_pid())?;
        let uid = credentials.uid.as_raw();
        let gid = credentials.gid.as_raw();
        ensure!(
            pid == child.id(),
            "engine peer PID differs from active child"
        );
        ensure!(
            uid == rustix::process::geteuid().as_raw()
                && gid == rustix::process::getegid().as_raw(),
            "engine peer UID/GID differs from current effective identity"
        );
        let process =
            rustix::process::pidfd_open(credentials.pid, rustix::process::PidfdFlags::empty())
                .map_err(|_| {
                    anyhow::anyhow!("Linux pidfd process authentication is unavailable")
                })?;
        let image = validate_engine_image_binding(verified_image, pid)?;
        ensure!(
            child.try_wait()?.is_none(),
            "engine child exited during authentication"
        );
        stream.set_read_timeout(Some(read_timeout))?;
        stream.set_write_timeout(Some(write_timeout))?;
        let peer = AuthenticatedEnginePeer {
            stream,
            pid,
            uid,
            gid,
            process,
            image,
            channel,
            read_timeout,
            write_timeout,
        };
        super::validate_engine_peer_liveness::validate_engine_peer_liveness(&peer)?;
        Ok(peer)
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (
            stream,
            child,
            verified_image,
            channel,
            read_timeout,
            write_timeout,
        );
        unreachable!("unsupported transport was rejected before constructing a peer")
    }
}
