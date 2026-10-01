// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use alloy_primitives::{B256, keccak256};

pub fn native_event_topic(signature: &str) -> B256 {
    keccak256(signature.as_bytes())
}
