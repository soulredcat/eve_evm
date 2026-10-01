// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Result, ensure};
use std::path::Path;

pub(in crate::development::engine) fn validate_engine_socket_paths(
    data: &Path,
    application: &Path,
    signer: &Path,
) -> Result<()> {
    ensure!(
        application != signer,
        "engine application and signer sockets must differ"
    );
    for socket in [application, signer] {
        let text = socket
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("engine Unix path must be UTF-8"))?;
        ensure!(
            socket.is_absolute() && text.len() <= 100 && !text.contains(['\0', '\n', '\r']),
            "invalid bounded engine Unix socket path"
        );
        ensure!(
            socket
                .parent()
                .ok_or_else(|| anyhow::anyhow!("engine socket parent missing"))?
                .canonicalize()?
                == data.canonicalize()?,
            "engine socket must stay under task data"
        );
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::{FileTypeExt, MetadataExt};
        let metadata = std::fs::symlink_metadata(application)?;
        ensure!(
            metadata.file_type().is_socket()
                && metadata.uid() == rustix::process::geteuid().as_raw(),
            "application endpoint must be an existing owned Unix listener"
        );
    }
    ensure!(
        !signer.try_exists()?,
        "native signer listener path already exists; never overwrite foreign endpoints"
    );
    Ok(())
}
