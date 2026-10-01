// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::io;

pub(in crate::consensus) fn validate_linux_transport() -> io::Result<()> {
    if cfg!(target_os = "linux") {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "authenticated engine transport requires Linux",
        ))
    }
}
