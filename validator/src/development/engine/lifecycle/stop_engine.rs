// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::development::engine::OwnedEngine;
use anyhow::Result;
use std::time::{Duration, Instant};

/// Stop/wait only this actual owned child; the original pidfd is used for graceful termination.
pub(crate) fn stop_engine(engine: &mut OwnedEngine) -> Result<()> {
    if let Some(child) = &mut engine.child {
        if child.try_wait()?.is_none() {
            if let Some(process) = &engine.process {
                let _ = rustix::process::pidfd_send_signal(process, rustix::process::Signal::TERM);
            }
            let deadline = Instant::now() + Duration::from_secs(5);
            while child.try_wait()?.is_none() && Instant::now() < deadline {
                std::thread::sleep(Duration::from_millis(10));
            }
            if child.try_wait()?.is_none() {
                child.kill()?;
            }
        }
        child.wait()?;
        engine.child = None;
    }
    if engine.lease.is_some() {
        super::cleanup_owned_engine_socket(&engine.data, &engine.signer_socket)?;
    }
    engine.process = None;
    if let Some(lease) = &mut engine.lease {
        crate::development::engine::home::release_engine_lease(lease)?;
    }
    engine.lease = None;
    Ok(())
}
