// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Result, ensure};
use std::{
    process::Child,
    time::{Duration, Instant},
};
pub(crate) fn wait_child(child: &mut Child) -> Result<()> {
    let deadline = Instant::now() + Duration::from_secs(10);
    while child.try_wait()?.is_none() {
        if Instant::now() >= deadline {
            child.kill()?;
            let _ = child.wait();
            anyhow::bail!("owned validator graceful shutdown deadline");
        }
        std::thread::sleep(Duration::from_millis(25));
    }
    ensure!(child.try_wait()?.is_some(), "owned child was not reaped");
    Ok(())
}
