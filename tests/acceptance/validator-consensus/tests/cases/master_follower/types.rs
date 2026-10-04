// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::{net::SocketAddr, path::PathBuf, process::Child};

pub(super) struct MasterFollower {
    pub binary: PathBuf,
    pub repository_root: PathBuf,
    pub artifact: PathBuf,
    pub data: PathBuf,
    pub genesis: PathBuf,
    pub validator: SocketAddr,
    pub stdout: PathBuf,
    pub stderr: PathBuf,
    pub launch_count: u8,
    pub child: Option<Child>,
    pub process_start: Option<u64>,
    pub _namespace: tempfile::TempDir,
}
