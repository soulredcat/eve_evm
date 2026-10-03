// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

#[path = "../support/compiler.rs"]
mod compiler;

#[test]
fn downstream_consumer_cannot_construct_verified_recovery_state() {
    compiler::must_reject(
        r#"
        #![allow(unreachable_code)]
        use eve_finality_verifier::DevelopmentRecoveryState;
        fn forge() -> DevelopmentRecoveryState {
            DevelopmentRecoveryState {
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
fn downstream_consumer_cannot_construct_verified_recovery_transition() {
    compiler::must_reject(
        r#"
        #![allow(unreachable_code)]
        use eve_finality_verifier::VerifiedRecoveryTransition;
        fn forge() -> VerifiedRecoveryTransition {
            VerifiedRecoveryTransition { state: panic!(), envelope: panic!() }
        }
        fn main() {}
    "#,
        "E0451",
    );
}
