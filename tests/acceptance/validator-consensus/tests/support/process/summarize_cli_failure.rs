// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::{
    collections::BTreeSet,
    fs::OpenOptions,
    io::Read,
    os::unix::fs::{MetadataExt, OpenOptionsExt},
    path::Path,
};

/// Accept only bounded private CLI diagnostics stamped with the exact owned PID.
pub(crate) fn summarize_cli_failure(path: &Path, process_id: u32) -> String {
    let read = || -> Option<String> {
        let file = OpenOptions::new()
            .read(true)
            .custom_flags(
                (rustix::fs::OFlags::NOFOLLOW | rustix::fs::OFlags::NONBLOCK).bits() as i32,
            )
            .open(path)
            .ok()?;
        let metadata = file.metadata().ok()?;
        if !metadata.is_file()
            || metadata.len() > 16_384
            || metadata.mode() & 0o777 != 0o600
            || metadata.uid() != rustix::process::geteuid().as_raw()
        {
            return None;
        }
        let mut bytes = Vec::new();
        file.take(16_385).read_to_end(&mut bytes).ok()?;
        if bytes.len() > 16_384 {
            return None;
        }
        let text = std::str::from_utf8(&bytes).ok()?;
        let prefix = format!("EVE_DEVELOPMENT_VALIDATOR_FAILED {process_id}: ");
        let mut codes = BTreeSet::new();
        for line in text.lines() {
            let Some(values) = line
                .strip_prefix(&prefix)
                .filter(|values| values.len() <= 512)
            else {
                continue;
            };
            for value in values.split(',').take(8) {
                codes.insert(match value {
                    "NODE_ASSEMBLY_FAILED" => "NODE_ASSEMBLY_FAILED",
                    "NODE_APPLICATION_BIND_FAILED" => "NODE_APPLICATION_BIND_FAILED",
                    "NODE_ENGINE_LAUNCH_FAILED" => "NODE_ENGINE_LAUNCH_FAILED",
                    "ENGINE_NAMESPACE_FAILED" => "ENGINE_NAMESPACE_FAILED",
                    "ENGINE_LEASE_FAILED" => "ENGINE_LEASE_FAILED",
                    "ENGINE_BINARY_VALIDATION_FAILED" => "ENGINE_BINARY_VALIDATION_FAILED",
                    "ENGINE_CONFIGURATION_FAILED" => "ENGINE_CONFIGURATION_FAILED",
                    "ENGINE_PROCESS_SPAWN_FAILED" => "ENGINE_PROCESS_SPAWN_FAILED",
                    "ENGINE_PIDFD_OPEN_FAILED" => "ENGINE_PIDFD_OPEN_FAILED",
                    "ENGINE_IMAGE_BINDING_FAILED" => "ENGINE_IMAGE_BINDING_FAILED",
                    "ENGINE_VERIFIED_IMAGE_MISSING" => "ENGINE_VERIFIED_IMAGE_MISSING",
                    "ENGINE_HELD_IMAGE_CHANGED" => "ENGINE_HELD_IMAGE_CHANGED",
                    "ENGINE_PROCESS_IMAGE_UNREADABLE" => "ENGINE_PROCESS_IMAGE_UNREADABLE",
                    "ENGINE_PROCESS_DEVICE_MISMATCH" => "ENGINE_PROCESS_DEVICE_MISMATCH",
                    "ENGINE_PROCESS_INODE_MISMATCH" => "ENGINE_PROCESS_INODE_MISMATCH",
                    "ENGINE_PROCESS_LENGTH_MISMATCH" => "ENGINE_PROCESS_LENGTH_MISMATCH",
                    "ENGINE_PROCESS_MTIME_MISMATCH" => "ENGINE_PROCESS_MTIME_MISMATCH",
                    "ENGINE_PROCESS_CTIME_MISMATCH" => "ENGINE_PROCESS_CTIME_MISMATCH",
                    "IO_PERMISSION_DENIED" => "IO_PERMISSION_DENIED",
                    "IO_NOT_FOUND" => "IO_NOT_FOUND",
                    "IO_UNSUPPORTED" => "IO_UNSUPPORTED",
                    "IO_WOULD_BLOCK" => "IO_WOULD_BLOCK",
                    "IO_TIMED_OUT" => "IO_TIMED_OUT",
                    _ => "REDACTED_CLI_CATEGORY",
                });
            }
        }
        (!codes.is_empty()).then(|| codes.into_iter().take(8).collect::<Vec<_>>().join(","))
    };
    read().unwrap_or_else(|| "CLI_FAILURE_RECORD_UNAVAILABLE".into())
}
