// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::collections::BTreeSet;

/// Render fixed operator diagnostics without exposing the underlying error text.
pub fn summarize_cli_failure(error: &anyhow::Error) -> String {
    let mut codes = BTreeSet::new();
    for cause in error.chain().take(8) {
        let code = match cause.to_string().as_str() {
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
            _ => continue,
        };
        codes.insert(code);
    }
    let io_kind = error
        .downcast_ref::<std::io::Error>()
        .map(|error| error.kind())
        .or_else(|| {
            error
                .downcast_ref::<rustix::io::Errno>()
                .map(|error| std::io::Error::from(*error).kind())
        });
    if let Some(kind) = io_kind {
        codes.insert(match kind {
            std::io::ErrorKind::PermissionDenied => "IO_PERMISSION_DENIED",
            std::io::ErrorKind::NotFound => "IO_NOT_FOUND",
            std::io::ErrorKind::Unsupported => "IO_UNSUPPORTED",
            std::io::ErrorKind::WouldBlock => "IO_WOULD_BLOCK",
            std::io::ErrorKind::TimedOut => "IO_TIMED_OUT",
            _ => "REDACTED_IO_KIND",
        });
    }
    if codes.is_empty() {
        "REDACTED_UNCLASSIFIED".into()
    } else {
        codes.into_iter().take(8).collect::<Vec<_>>().join(",")
    }
}
