// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

/// Immutable local namespace binding; no field proves enrollment or network authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OpaqueRecordIdentity {
    pub genesis_hash: [u8; 32],
    pub owner: [u8; 32],
    pub domain: [u8; 32],
}
