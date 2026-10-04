// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

#[path = "compiler.rs"]
mod compiler;

#[test]
fn downstream_cannot_construct_a_segmented_preflight_from_unbound_record_and_stats() {
    compiler::must_reject(
        r#"
        #![allow(unreachable_code)]
        use eve_storage::records::segmented::SegmentedRecordPreflight;
        fn forge() -> SegmentedRecordPreflight<'static> {
            SegmentedRecordPreflight { bytes: panic!(), limits: panic!(), record: panic!(), stats: panic!() }
        }
        fn main() {}
    "#,
        "E0451",
    );
}

#[test]
fn downstream_cannot_mutate_bytes_while_the_same_borrowed_preflight_is_used() {
    compiler::must_reject(
        r#"
        use eve_storage::records::segmented::{SegmentedCodecLimits, preflight_segmented_record, decode_segmented_record};
        fn mutate() {
            let limits = SegmentedCodecLimits { maximum_logical_bytes: 384, maximum_chunk_bytes: 64,
                maximum_segments: 6, maximum_payload_bytes: 465 };
            let mut bytes = vec![0; 300];
            let preflight = preflight_segmented_record(&bytes, &limits).unwrap();
            bytes[0] ^= 1;
            let _ = decode_segmented_record(&preflight);
        }
        fn main() {}
    "#,
        "E0502",
    );
}

#[test]
fn detached_stats_cannot_be_substituted_for_the_sealed_segmented_decoder_input() {
    compiler::must_reject(
        r#"
        use eve_storage::records::segmented::{SegmentedRecordStats, decode_segmented_record};
        fn substitute(stats: &SegmentedRecordStats) { let _ = decode_segmented_record(stats); }
        fn main() {}
    "#,
        "E0308",
    );
}
