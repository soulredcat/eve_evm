// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::{io, os::unix::net::UnixStream, path::Path, time::Duration};

pub(in crate::consensus::runtime) fn connect_signer_socket(
    runtime: &tokio::runtime::Runtime,
    socket: &Path,
) -> io::Result<UnixStream> {
    let stream = runtime.block_on(async {
        tokio::time::timeout(
            Duration::from_millis(250),
            tokio::net::UnixStream::connect(socket),
        )
        .await
    })??;
    let stream = stream.into_std()?;
    stream.set_nonblocking(false)?;
    Ok(stream)
}
