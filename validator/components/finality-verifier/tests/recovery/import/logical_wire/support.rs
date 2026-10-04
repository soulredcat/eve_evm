// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

pub const DOMAIN: &[u8] = b"EVE_IMPORT_V2";

pub fn replace(bytes: &[u8], field: usize, change: impl FnOnce(&[u8]) -> Vec<u8>) -> Vec<u8> {
    crate::codec_mutation::replace_field(bytes, DOMAIN.len(), field, change)
}
