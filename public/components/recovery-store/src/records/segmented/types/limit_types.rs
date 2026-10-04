// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

pub(in crate::records::segmented) const SEGMENTED_DOMAIN: &[u8] = b"EVE_SEGMENTED_RECOVERY_V1";
pub const SEGMENTED_SCHEMA_VERSION: u16 = 1;
pub(in crate::records::segmented) const SEGMENTED_BASE_BYTES: usize = 149;
pub const SEGMENTED_SEGMENT_HEADER_BYTES: usize = 177;
pub const SEGMENTED_SEGMENT_OVERHEAD_BYTES: usize = 209;
pub const SEGMENTED_MARKER_HEADER_BYTES: usize = 193;
pub const SEGMENTED_MARKER_OVERHEAD_BYTES: usize = 225;
pub const SEGMENTED_MAX_REFERENCES: usize = 6;
pub const SEGMENTED_MAX_MARKER_BYTES: usize = 465;
/// Target below the opaque 4,198,312-byte payload cap; two targets fit an 8 MiB part.
pub const SEGMENTED_MAX_PHYSICAL_PAYLOAD_BYTES: usize = 4_194_304;
pub const SEGMENTED_MAX_CHUNK_BYTES: usize = 4_194_095;

/// Explicit local codec admission supplied by the versioned part policy.
/// These limits do not reinterpret PublicBudget v1 or establish RSS enforcement.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SegmentedCodecLimits {
    pub maximum_logical_bytes: usize,
    pub maximum_chunk_bytes: usize,
    pub maximum_segments: usize,
    pub maximum_payload_bytes: usize,
}
