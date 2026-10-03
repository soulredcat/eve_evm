// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::ensure_submission_deadline::ensure_submission_deadline;
use crate::support::rpc::rpc_json;
use anyhow::Result;
use serde_json::Value;
use std::{net::SocketAddr, time::Instant};

pub(super) fn submission_rpc_until(
    address: SocketAddr,
    path: &str,
    deadline: Instant,
) -> Result<Value> {
    ensure_submission_deadline(deadline)?;
    let result = rpc_json(address, path);
    ensure_submission_deadline(deadline)?;
    result
}
