// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{
    SEGMENTED_SCHEMA_VERSION, SegmentedLogicalIdentity, SegmentedRecordKind,
    types::{SEGMENTED_BASE_BYTES, SEGMENTED_DOMAIN},
};

pub(in crate::records::segmented) fn encode_segmented_base(
    identity: &SegmentedLogicalIdentity,
    kind: SegmentedRecordKind,
) -> [u8; SEGMENTED_BASE_BYTES] {
    let mut output = [0; SEGMENTED_BASE_BYTES];
    output[..25].copy_from_slice(SEGMENTED_DOMAIN);
    output[25] = kind as u8;
    output[26..28].copy_from_slice(&SEGMENTED_SCHEMA_VERSION.to_be_bytes());
    output[28] = identity.mode as u8;
    output[29..61].copy_from_slice(&identity.logical_id);
    output[61..69].copy_from_slice(&identity.parent.height.to_be_bytes());
    output[69..77].copy_from_slice(&identity.parent.cursor.sequence.to_be_bytes());
    output[77..109].copy_from_slice(&identity.parent.cursor.content_hash);
    output[109..141].copy_from_slice(&identity.parent.state_binding);
    output[141..149].copy_from_slice(&identity.target_height.to_be_bytes());
    output
}
