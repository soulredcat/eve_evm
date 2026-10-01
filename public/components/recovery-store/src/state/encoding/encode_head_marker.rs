// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use alloy_primitives::B256;

pub(crate) fn encode_head_marker(height: u64, identity: B256) -> Vec<u8> {
    [&height.to_be_bytes(), identity.as_slice()].concat()
}
