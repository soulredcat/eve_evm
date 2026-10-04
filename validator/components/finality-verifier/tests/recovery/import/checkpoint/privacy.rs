// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::compiler;

#[test]
fn downstream_consumer_cannot_construct_an_authenticated_checkpoint_capability() {
    compiler::must_reject(
        r#"
        #![allow(unreachable_code)]
        use eve_finality_verifier::AuthenticatedCheckpoint;
        fn forge() -> AuthenticatedCheckpoint {
            AuthenticatedCheckpoint { commit: panic!(), finality: panic!(), policy: panic!(),
                lookahead: panic!(), lookahead_header: panic!(), anchor: panic!() }
        }
        fn main() {}
    "#,
        "E0451",
    );
}

#[test]
fn checkpoint_capability_cannot_be_passed_as_independent_replay_authority() {
    compiler::must_reject(
        r#"
        use eve_finality_verifier::{AuthenticatedCheckpoint, recovery_state_commit};
        fn reinterpret(value: &AuthenticatedCheckpoint) { let _ = recovery_state_commit(value); }
        fn main() {}
    "#,
        "E0308",
    );
}
