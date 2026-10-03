// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use serde_json::Value;
use std::time::Duration;

pub(in super::super) struct RpcReply {
    pub target: String,
    pub result: Value,
    pub delay: Duration,
}
