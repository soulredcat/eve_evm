// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::recovery::types::RECORD_PREFIX;

pub fn record_key(height: u64) -> Vec<u8> {
    [RECORD_PREFIX, &height.to_be_bytes()].concat()
}
