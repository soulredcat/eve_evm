// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::cases::public_follower::PublicFollower;
use anyhow::{Context, Result, ensure};
use rustix::fs::{Mode, OFlags, RenameFlags, open, renameat_with};
use std::{
    fs::File,
    io::ErrorKind,
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
};

/// Preserve only this reaped fixture's configured data under an unused name.
/// Held parent directory and NOREPLACE prevent following or overwriting entries.
pub(super) fn relocate_owned_replica(
    replica: &PublicFollower,
    namespace: &Path,
) -> Result<PathBuf> {
    ensure!(
        replica.child.is_none() && replica.process_start.is_none() && replica.launch_count == 1,
        "PUBLIC_REPLICA_RELOCATION_REQUIRES_REAPED_OWNED_CHILD"
    );
    let root = namespace.canonicalize()?;
    ensure!(
        root.is_absolute() && root == replica.repository_root.canonicalize()?,
        "PUBLIC_REPLICA_RELOCATION_ROOT_MISMATCH"
    );
    let parent = root.join("local-tests");
    let source = parent.join("public-checkpoint-state");
    let lost_name = format!("preserved-lost-public-replica-{}", replica.launch_count);
    let lost = parent.join(&lost_name);
    ensure!(
        replica.data == source
            && source.is_absolute()
            && lost.is_absolute()
            && source.starts_with(&root)
            && lost.starts_with(&root)
            && source.parent() == Some(parent.as_path())
            && lost.parent() == Some(parent.as_path()),
        "PUBLIC_REPLICA_RELOCATION_ABSOLUTE_CONTAINMENT"
    );
    for path in [&root, &parent, &source] {
        let metadata = std::fs::symlink_metadata(path)?;
        ensure!(
            metadata.is_dir()
                && !metadata.file_type().is_symlink()
                && metadata.uid() == rustix::process::geteuid().as_raw(),
            "PUBLIC_REPLICA_RELOCATION_UNSAFE_DIRECTORY"
        );
        ensure!(
            path.canonicalize()? == *path,
            "PUBLIC_REPLICA_RELOCATION_PATH_ALIAS"
        );
    }
    ensure!(
        root.metadata()?.mode() & 0o077 == 0 && parent.metadata()?.mode() & 0o077 == 0,
        "PUBLIC_REPLICA_RELOCATION_PRIVATE_PARENT"
    );
    match std::fs::symlink_metadata(&lost) {
        Err(error) if error.kind() == ErrorKind::NotFound => {}
        Err(error) => return Err(error.into()),
        Ok(_) => anyhow::bail!("PUBLIC_REPLICA_RELOCATION_TARGET_OCCUPIED"),
    }
    let original = std::fs::symlink_metadata(&source)?;
    let directory = File::from(open(
        &parent,
        OFlags::RDONLY | OFlags::DIRECTORY | OFlags::NOFOLLOW | OFlags::CLOEXEC,
        Mode::empty(),
    )?);
    let held = directory.metadata()?;
    let expected = parent.metadata()?;
    ensure!(
        held.dev() == expected.dev() && held.ino() == expected.ino(),
        "PUBLIC_REPLICA_RELOCATION_PARENT_CHANGED"
    );
    renameat_with(
        &directory,
        source.file_name().context("PUBLIC_REPLICA_SOURCE_NAME")?,
        &directory,
        lost_name.as_str(),
        RenameFlags::NOREPLACE,
    )?;
    directory.sync_all()?;
    let preserved = std::fs::symlink_metadata(&lost)?;
    ensure!(
        preserved.is_dir()
            && !preserved.file_type().is_symlink()
            && preserved.dev() == original.dev()
            && preserved.ino() == original.ino(),
        "PUBLIC_REPLICA_RELOCATION_DID_NOT_PRESERVE_DIRECTORY"
    );
    ensure!(
        std::fs::symlink_metadata(&source).is_err_and(|error| error.kind() == ErrorKind::NotFound),
        "PUBLIC_REPLICA_CONFIGURED_DATA_STILL_PRESENT"
    );
    Ok(lost)
}
