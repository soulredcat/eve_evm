// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::compiler;

#[test]
fn downstream_consumer_cannot_construct_authenticated_import_state() {
    compiler::must_reject(
        r#"
        #![allow(unreachable_code)]
        use eve_finality_verifier::ImportedState;
        fn forge() -> ImportedState {
            ImportedState {
                commit: panic!(), finality: panic!(), policy: panic!(),
                lookahead: panic!(), lookahead_header: panic!(), anchor: panic!(),
            }
        }
        fn main() {}
    "#,
        "E0451",
    );
}

#[test]
fn downstream_consumer_cannot_construct_authenticated_import_transition() {
    compiler::must_reject(
        r#"
        #![allow(unreachable_code)]
        use eve_finality_verifier::ImportedTransition;
        fn forge() -> ImportedTransition {
            ImportedTransition { state: panic!(), input: panic!() }
        }
        fn main() {}
    "#,
        "E0451",
    );
}

#[test]
fn authenticated_import_state_cannot_be_passed_as_an_independent_replay_state() {
    compiler::must_reject(
        r#"
        use eve_finality_verifier::{ImportedState, recovery_state_commit};
        fn reinterpret(state: &ImportedState) {
            let _ = recovery_state_commit(state);
        }
        fn main() {}
    "#,
        "E0308",
    );
}

#[test]
fn authenticated_import_transition_cannot_be_consumed_as_an_independent_replay_transition() {
    compiler::must_reject(
        r#"
        use eve_finality_verifier::{ImportedTransition, into_recovery_state};
        fn reinterpret(transition: ImportedTransition) {
            let _ = into_recovery_state(transition);
        }
        fn main() {}
    "#,
        "E0308",
    );
}
