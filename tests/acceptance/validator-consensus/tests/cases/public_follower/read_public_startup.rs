// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::PublicFollower;
use anyhow::{Result, ensure};
use serde_json::Value;
use std::{
    fs::File,
    io::Read,
    time::{Duration, Instant},
};

pub(super) fn read_public_startup(
    follower: &mut PublicFollower,
    deadline: Instant,
) -> Result<Value> {
    loop {
        ensure!(
            Instant::now() < deadline,
            "PUBLIC_FOLLOWER_STARTUP_DEADLINE"
        );
        ensure!(
            follower
                .child
                .as_mut()
                .ok_or_else(|| anyhow::anyhow!("PUBLIC_FOLLOWER_CHILD_MISSING"))?
                .try_wait()?
                .is_none(),
            "PUBLIC_FOLLOWER_PROCESS_EXITED"
        );
        let mut bytes = Vec::new();
        File::open(&follower.stdout)?
            .take(4_097)
            .read_to_end(&mut bytes)?;
        ensure!(bytes.len() <= 4_096, "PUBLIC_FOLLOWER_STARTUP_BOUND");
        if let Some(line) = bytes
            .split(|byte| *byte == b'\n')
            .find(|line| !line.is_empty())
            && let Ok(started) = serde_json::from_slice::<Value>(line)
        {
            ensure!(
                started["http_address"] == follower.http.to_string()
                    && started["ws_address"] == follower.ws.to_string()
                    && started["verification_mode"] == "AUTHENTICATED_IMPORT_CLASSICAL_DEV",
                "PUBLIC_FOLLOWER_STARTUP_IDENTITY"
            );
            return Ok(started);
        }
        std::thread::sleep(Duration::from_millis(25));
    }
}
