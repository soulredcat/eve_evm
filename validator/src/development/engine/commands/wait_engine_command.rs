// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Result, ensure};
use std::{
    path::Path,
    process::{Child, ExitStatus},
    time::{Duration, Instant},
};

pub(in crate::development::engine) fn wait_engine_command(
    child: &mut Child,
    output: &Path,
    error: &Path,
) -> Result<ExitStatus> {
    let deadline = Instant::now() + Duration::from_secs(15);
    loop {
        ensure!(
            output.metadata()?.len() <= 65_536
                && error.metadata()?.len() <= 65_536
                && Instant::now() < deadline,
            "owned engine command exceeded time/output limit"
        );
        if let Some(status) = child.try_wait()? {
            return Ok(status);
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}
