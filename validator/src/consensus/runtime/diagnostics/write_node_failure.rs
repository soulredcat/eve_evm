// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::NodeFailureRecord;
use anyhow::{Result, ensure};
use std::{
    fs::OpenOptions,
    io::Write,
    os::unix::fs::{MetadataExt, OpenOptionsExt},
    path::Path,
};

pub(in crate::consensus::runtime) fn write_node_failure(
    data: &Path,
    record: &NodeFailureRecord,
) -> Result<()> {
    let metadata = std::fs::symlink_metadata(data)?;
    ensure!(
        metadata.is_dir()
            && metadata.uid() == rustix::process::geteuid().as_raw()
            && metadata.mode() & 0o777 == 0o700,
        "failure diagnostic requires owned private directory"
    );
    let bytes = serde_json::to_vec(record)?;
    ensure!(bytes.len() <= 8192, "failure diagnostic byte bound");
    let path = data.join(format!("runtime-failure-{}.log", record.process_id));
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(path)?;
    file.write_all(&bytes)?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    super::super::namespace::sync_node_directory(data)?;
    Ok(())
}
