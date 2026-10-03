// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::compiler;

#[test]
fn downstream_cannot_construct_v2_preflight_from_detached_counts() {
    compiler::must_reject(
        r#"
        #![allow(unreachable_code)]
        use eve_finality_verifier::LogicalImportWirePreflight;
        fn forge() -> LogicalImportWirePreflight<'static> {
            LogicalImportWirePreflight { bytes: panic!(), budget: panic!(), slices: panic!(), stats: panic!() }
        }
        fn main() {}
    "#,
        "E0451",
    );
}

#[test]
fn downstream_cannot_mutate_v2_source_bytes_while_preflight_is_used() {
    compiler::must_reject(
        r#"
        use eve_finality_verifier::{preflight_logical_import_wire, decode_logical_import_wire};
        use eve_state::development_state_budget;
        fn mutate() {
            let mut bytes = vec![0; 32];
            let preflight = preflight_logical_import_wire(&bytes, &development_state_budget()).unwrap();
            bytes[0] ^= 1;
            let _ = decode_logical_import_wire(&preflight);
        }
        fn main() {}
    "#,
        "E0502",
    );
}

#[test]
fn downstream_cannot_decode_v2_from_statistics_or_a_v1_preflight() {
    compiler::must_reject(
        r#"
        use eve_finality_verifier::{ImportWireStats, ImportWirePreflight, decode_logical_import_wire};
        fn substitute(stats: &ImportWireStats, compact: &ImportWirePreflight<'_>) {
            let _ = decode_logical_import_wire(stats);
            let _ = decode_logical_import_wire(compact);
        }
        fn main() {}
    "#,
        "E0308",
    );
}
