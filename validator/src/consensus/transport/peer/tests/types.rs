// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{AuthenticatedEnginePeer, shutdown_engine_peer};
use std::{os::unix::net::UnixStream, process::Child};
use tempfile::TempDir;

pub(super) struct TestEngineConnection {
    pub stream: Option<UnixStream>,
    pub child: Option<Child>,
    pub directory: Option<TempDir>,
    pub expected_sha256: [u8; 32],
    pub image: crate::development::engine::VerifiedEngineImage,
    pub executable: std::path::PathBuf,
}

impl Drop for TestEngineConnection {
    fn drop(&mut self) {
        if let Some(child) = &mut self.child {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

/// Real process/accepted socket fixture; no simulated credentials create a peer capability.
pub(in crate::consensus) struct AuthenticatedTestPeer {
    pub(in crate::consensus) peer: AuthenticatedEnginePeer,
    pub(in crate::consensus) child: Child,
    pub(super) _directory: TempDir,
}

impl Drop for AuthenticatedTestPeer {
    fn drop(&mut self) {
        let _ = shutdown_engine_peer(&self.peer);
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}
