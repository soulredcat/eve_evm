// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_node_policy::SegmentedRecoveryBounds;

pub const MIB: u64 = 1_048_576;

pub fn bounds() -> SegmentedRecoveryBounds {
    SegmentedRecoveryBounds {
        maximum_logical_bytes: 21_025_569,
        maximum_segment_bytes: 4 * MIB,
        segment_header_bytes: 177,
        segment_footer_bytes: 32,
        maximum_segments: 6,
        maximum_marker_bytes: 465,
        opaque_record_header_bytes: 88,
        maximum_opaque_record_bytes: 4 * MIB + 4_096,
        maximum_opaque_read_bytes: 4 * MIB + 4_096,
        maximum_opaque_batch_bytes: 16 * MIB,
        maximum_opaque_batch_records: 1,
        // Test runtime sizing facts. Production derives descriptor charges from
        // its actual structures; this fixture makes no descriptor/RSS assertion.
        required_metadata_bytes: 4_096,
        required_scratch_bytes: 33_566_808,
        metadata_limit_bytes: 8_192,
        scratch_limit_bytes: 40 * MIB,
        available_auxiliary_bytes: 60 * MIB,
    }
}
