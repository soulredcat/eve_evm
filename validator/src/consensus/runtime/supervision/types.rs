// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::{os::unix::net::UnixListener, path::PathBuf};

pub(in crate::consensus::runtime) struct ApplicationListener {
    pub listener: UnixListener,
    pub path: PathBuf,
    pub device: u64,
    pub inode: u64,
}
