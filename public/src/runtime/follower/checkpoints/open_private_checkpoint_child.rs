// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::check_checkpoint_bootstrap_deadline::check_checkpoint_bootstrap_deadline;
use anyhow::{Context, Result, ensure};
use std::fs::File;
use std::time::Instant;

pub(super) fn open_private_checkpoint_child(
    parent: &File,
    name: &str,
    create: bool,
    deadline: Option<Instant>,
) -> Result<Option<File>> {
    if let Some(deadline) = deadline {
        check_checkpoint_bootstrap_deadline(deadline)?;
    }
    ensure!(
        matches!(name, ".checkpoints" | "content" | "proofs"),
        "invalid generated checkpoint directory"
    );
    #[cfg(target_os = "linux")]
    {
        use rustix::fs::{Mode, OFlags, mkdirat, openat};
        use std::os::unix::fs::MetadataExt;
        if create {
            match mkdirat(parent, name, Mode::from_raw_mode(0o700)) {
                Ok(()) => {
                    parent.sync_all().context("checkpoint parent sync failed")?;
                    if let Some(deadline) = deadline {
                        check_checkpoint_bootstrap_deadline(deadline)?;
                    }
                }
                Err(rustix::io::Errno::EXIST) => {}
                Err(error) => return Err(error.into()),
            }
        }
        let file = match openat(
            parent,
            name,
            OFlags::RDONLY | OFlags::DIRECTORY | OFlags::NOFOLLOW | OFlags::CLOEXEC,
            Mode::empty(),
        ) {
            Ok(file) => File::from(file),
            Err(rustix::io::Errno::NOENT) if !create => return Ok(None),
            Err(error) => return Err(error.into()),
        };
        let metadata = file.metadata()?;
        if let Some(deadline) = deadline {
            check_checkpoint_bootstrap_deadline(deadline)?;
        }
        ensure!(
            metadata.is_dir()
                && metadata.uid() == rustix::process::geteuid().as_raw()
                && metadata.mode() & 0o077 == 0,
            "checkpoint directory must be privately owned"
        );
        if create {
            file.sync_all()
                .context("checkpoint directory sync failed")?;
            if let Some(deadline) = deadline {
                check_checkpoint_bootstrap_deadline(deadline)?;
            }
            parent.sync_all()?;
            if let Some(deadline) = deadline {
                check_checkpoint_bootstrap_deadline(deadline)?;
            }
        }
        Ok(Some(file))
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (parent, name, create, deadline);
        anyhow::bail!("checkpoint directories require supported Linux durability")
    }
}
