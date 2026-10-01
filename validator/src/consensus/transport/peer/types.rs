// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::time::Duration;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::consensus) enum EngineChannel {
    Application,
    Signer,
}

/// Sealed provenance for one accepted FD. Only real child/credentials/image checks construct it.
/// Local OS/operator compromise and copied keys are outside this capability's assurance.
pub(in crate::consensus) struct AuthenticatedEnginePeer {
    #[cfg(unix)]
    pub(super) stream: std::os::unix::net::UnixStream,
    pub(super) pid: u32,
    #[cfg(target_os = "linux")]
    pub(super) uid: u32,
    #[cfg(target_os = "linux")]
    pub(super) gid: u32,
    #[cfg(target_os = "linux")]
    pub(super) process: rustix::fd::OwnedFd,
    #[cfg(target_os = "linux")]
    pub(super) image: crate::development::engine::EngineImageIdentity,
    pub(super) channel: EngineChannel,
    pub(super) read_timeout: Duration,
    pub(super) write_timeout: Duration,
}

/// Close-only connection capability; it cannot read/write packets or become an authenticated proof.
pub(in crate::consensus) struct EnginePeerShutdown {
    #[cfg(unix)]
    pub(super) stream: std::os::unix::net::UnixStream,
}
