// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{engine_process_image_identity, types::EngineImageIdentity};
use std::{
    io,
    os::unix::fs::MetadataExt,
    time::{Duration, Instant},
};

/// Read the spawned engine image once its exec transition is complete.
///
/// A vfork-style spawn resumes the caller during the child's exec, before the
/// kernel installs the child's new address space. Until then `/proc/<pid>/exe`
/// still names the caller's own executable. Only that exact transient state is
/// polled within `limit`; every other image is returned for strict comparison.
pub(in crate::development::engine) fn await_engine_exec(
    pid: u32,
    limit: Duration,
) -> io::Result<EngineImageIdentity> {
    let caller = std::fs::metadata("/proc/self/exe")?;
    let caller = (caller.dev(), caller.ino());
    let deadline = Instant::now() + limit;
    loop {
        let current = std::fs::metadata(format!("/proc/{pid}/exe"))?;
        if (current.dev(), current.ino()) != caller {
            return engine_process_image_identity(pid);
        }
        if Instant::now() >= deadline {
            return Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "ENGINE_PROCESS_EXEC_INCOMPLETE",
            ));
        }
        std::thread::sleep(Duration::from_millis(1));
    }
}
