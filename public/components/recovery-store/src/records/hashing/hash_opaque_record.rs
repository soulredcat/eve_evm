// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::records::{
    OpaqueRecordCursor, OpaqueRecordIdentity,
    encoding::{encode_opaque_cursor, encode_opaque_identity},
};
use sha2::{Digest, Sha256};

/// Standard SHA-256 local content binding; no finality, enrolled identity or rollback proof.
pub(in crate::records) fn hash_opaque_record(
    identity: OpaqueRecordIdentity,
    parent: OpaqueRecordCursor,
    sequence: u64,
    payload: &[u8],
) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(b"EVE_OPAQUE_RECORD_V1");
    digest.update(encode_opaque_identity(identity));
    digest.update(encode_opaque_cursor(parent));
    digest.update(sequence.to_be_bytes());
    digest.update((payload.len() as u64).to_be_bytes());
    digest.update(payload);
    digest.finalize().into()
}
