// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use serde_json::Value;
use std::{net::SocketAddr, path::PathBuf, process::Child};

pub(in crate::cases) struct PublicFollower {
    pub binary: PathBuf,
    pub repository_root: PathBuf,
    pub data: PathBuf,
    pub genesis: PathBuf,
    pub validator: SocketAddr,
    pub http: SocketAddr,
    pub ws: SocketAddr,
    pub stdout: PathBuf,
    pub stderr: PathBuf,
    pub launch_count: u8,
    pub child: Option<Child>,
    pub process_start: Option<u64>,
    pub checkpoint_height: Option<u64>,
}

pub(in crate::cases) struct PublicObservation {
    pub roots: Value,
    pub block: Value,
    pub sender_balance: Value,
    pub sender_nonce: Value,
    pub status: Value,
}
