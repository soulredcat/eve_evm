// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    open_private_checkpoint_child::open_private_checkpoint_child, types::CheckpointDirectories,
};
use anyhow::{Result, ensure};
use std::time::Instant;
use std::{fs::File, path::Path};

/// Only generated children below the already contained development namespace are opened or created.
pub(in crate::runtime::follower) fn open_development_checkpoint_directories(
    data: &Path,
    create: bool,
    deadline: Option<Instant>,
) -> Result<Option<CheckpointDirectories>> {
    if !create {
        match std::fs::symlink_metadata(data.join(".checkpoints")) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(error) => return Err(error.into()),
            Ok(metadata) => ensure!(
                metadata.is_dir() && !metadata.file_type().is_symlink(),
                "checkpoint container must be an actual directory"
            ),
        }
    }
    #[cfg(target_os = "linux")]
    {
        use rustix::fs::{Mode, OFlags, open};
        use std::os::unix::fs::MetadataExt;
        let parent = File::from(open(
            data,
            OFlags::RDONLY | OFlags::DIRECTORY | OFlags::NOFOLLOW | OFlags::CLOEXEC,
            Mode::empty(),
        )?);
        ensure!(
            parent.metadata()?.uid() == rustix::process::geteuid().as_raw(),
            "checkpoint data must be locally owned"
        );
        let Some(container) =
            open_private_checkpoint_child(&parent, ".checkpoints", create, deadline)?
        else {
            return Ok(None);
        };
        let content = open_private_checkpoint_child(&container, "content", create, deadline)?;
        let proofs = open_private_checkpoint_child(&container, "proofs", create, deadline)?;
        match (content, proofs) {
            (Some(content), Some(proofs)) => Ok(Some(CheckpointDirectories { content, proofs })),
            _ if !create => Ok(None),
            _ => anyhow::bail!("checkpoint directory creation incomplete"),
        }
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (data, create, deadline);
        anyhow::bail!("checkpoint directories require supported Linux durability")
    }
}
