// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::records::OpaqueRecordIdentity;

pub(in crate::records) fn encode_opaque_identity(identity: OpaqueRecordIdentity) -> [u8; 96] {
    let mut bytes = [0; 96];
    bytes[..32].copy_from_slice(&identity.genesis_hash);
    bytes[32..64].copy_from_slice(&identity.owner);
    bytes[64..].copy_from_slice(&identity.domain);
    bytes
}
