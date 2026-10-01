// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::{
    fs::OpenOptions,
    io::Read,
    os::unix::fs::{MetadataExt, OpenOptionsExt},
    path::Path,
};

/// Read only a bounded private regular record for the exact owned child identity.
pub(super) fn read_owned_failure_metadata(
    data: &Path,
    process_id: u32,
) -> Option<serde_json::Value> {
    let file = OpenOptions::new()
        .read(true)
        .custom_flags((rustix::fs::OFlags::NOFOLLOW | rustix::fs::OFlags::NONBLOCK).bits() as i32)
        .open(data.join(format!("runtime-failure-{process_id}.log")))
        .ok()?;
    let metadata = file.metadata().ok()?;
    if !metadata.is_file()
        || metadata.len() > 8193
        || metadata.uid() != data.metadata().ok()?.uid()
        || metadata.mode() & 0o777 != 0o600
    {
        return None;
    }
    let mut bytes = Vec::new();
    file.take(8194).read_to_end(&mut bytes).ok()?;
    if bytes.len() > 8193 {
        return None;
    }
    let record: serde_json::Value = serde_json::from_slice(&bytes).ok()?;
    if record["version"].as_u64() != Some(1)
        || record["process_id"].as_u64() != Some(u64::from(process_id))
    {
        return None;
    }
    Some(record)
}
