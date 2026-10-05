// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::compiler;

#[test]
fn downstream_cannot_construct_wire_preflight_from_unbound_counts_and_slices() {
    compiler::must_reject(
        r#"
        #![allow(unreachable_code)]
        use eve_finality_verifier::ImportWirePreflight;
        fn forge() -> ImportWirePreflight<'static> {
            ImportWirePreflight { bytes: panic!(), budget: panic!(), slices: panic!(), stats: panic!() }
        }
        fn main() {}
    "#,
        "E0451",
    );
}

#[test]
fn downstream_cannot_mutate_the_raw_input_while_its_bound_preflight_is_used() {
    compiler::must_reject(
        r#"
        use eve_finality_verifier::{preflight_authenticated_import_wire, decode_authenticated_import_wire};
        fn mutate() {
            // Infer the budget from the actual API's dependency identity.
            let _mutate = |budget| {
                let mut bytes = vec![0; 32];
                let preflight = preflight_authenticated_import_wire(&bytes, budget).unwrap();
                bytes[0] ^= 1;
                let _ = decode_authenticated_import_wire(&preflight);
            };
        }
        fn main() {}
    "#,
        "E0502",
    );
}

#[test]
fn downstream_cannot_decode_a_detached_stats_copy_as_a_preflight_capability() {
    compiler::must_reject(
        r#"
        use eve_finality_verifier::{ImportWireStats, decode_authenticated_import_wire};
        fn substitute(stats: &ImportWireStats) {
            let _ = decode_authenticated_import_wire(stats);
        }
        fn main() {}
    "#,
        "E0308",
    );
}
