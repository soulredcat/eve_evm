// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

pub(crate) fn height_key(prefix: &[u8], height: u64) -> Vec<u8> {
    [prefix, &height.to_be_bytes()].concat()
}
