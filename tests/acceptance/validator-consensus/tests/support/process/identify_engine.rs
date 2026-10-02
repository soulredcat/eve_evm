// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Context, Result, bail, ensure};
use std::{
    path::Path,
    process::Child,
    time::{Duration, Instant},
};

pub(crate) fn process_start(pid: u32) -> Result<u64> {
    let stat = std::fs::read_to_string(format!("/proc/{pid}/stat"))?;
    stat.rsplit_once(')')
        .context("invalid owned process stat")?
        .1
        .split_whitespace()
        .nth(19)
        .context("owned start time missing")?
        .parse()
        .context("owned start time invalid")
}

pub(crate) fn identify_engine(
    parent: &mut Child,
    binary: &Path,
    home: &Path,
    data: &Path,
    stderr: &Path,
) -> Result<(u32, u64)> {
    let expected = binary.canonicalize()?;
    let parent_pid = parent.id();
    let deadline = Instant::now() + Duration::from_secs(60);
    let mut child_seen = false;
    let mut image_matched = false;
    loop {
        if parent.try_wait()?.is_some() {
            bail!(
                "owned validator exited during engine discovery; failure categories {},{}; artifacts {}",
                super::summarize_node_failure(data, parent_pid),
                super::summarize_cli_failure(stderr, parent_pid),
                data.display()
            );
        }
        let children =
            std::fs::read_to_string(format!("/proc/{parent_pid}/task/{parent_pid}/children"))?;
        for value in children.split_whitespace() {
            child_seen = true;
            let pid: u32 = value.parse()?;
            let image = std::fs::read_link(format!("/proc/{pid}/exe"));
            let command = std::fs::read(format!("/proc/{pid}/cmdline")).unwrap_or_default();
            let arguments: Vec<_> = command.split(|byte| *byte == 0).collect();
            image_matched |= image.as_deref().ok() == Some(expected.as_path());
            if image.as_deref().ok() == Some(expected.as_path())
                && arguments.get(1) == Some(&b"start".as_slice())
                && arguments.get(2) == Some(&b"--home".as_slice())
                && arguments.get(3) == Some(&home.as_os_str().as_encoded_bytes())
            {
                return Ok((pid, process_start(pid)?));
            }
        }
        ensure!(
            Instant::now() < deadline,
            "owned native engine child not found; failure categories {},{}; artifacts {}",
            if !child_seen {
                "ENGINE_DISCOVERY_NO_CHILD"
            } else if !image_matched {
                "ENGINE_DISCOVERY_IMAGE_MISMATCH"
            } else {
                "ENGINE_DISCOVERY_ARGUMENT_MISMATCH"
            },
            super::summarize_node_failure(data, parent_pid),
            data.display()
        );
        std::thread::sleep(Duration::from_millis(25));
    }
}
