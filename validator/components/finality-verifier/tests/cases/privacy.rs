// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

#[path = "../support/compiler.rs"]
mod compiler;

#[test]
fn downstream_consumer_cannot_construct_a_verified_development_header() {
    compiler::must_reject(
        r#"
        #![allow(unreachable_code)]
        use eve_finality_verifier::VerifiedDevelopmentHeader;
        fn forge() -> VerifiedDevelopmentHeader {
            VerifiedDevelopmentHeader { identity: panic!(), native: panic!() }
        }
        fn main() {}
    "#,
        "E0451",
    );
}

#[test]
fn downstream_consumer_cannot_construct_an_authenticated_application_anchor() {
    compiler::must_reject(
        r#"
        #![allow(unreachable_code)]
        use eve_finality_verifier::AuthenticatedApplicationAnchor;
        fn forge() -> AuthenticatedApplicationAnchor {
            AuthenticatedApplicationAnchor {
                identity: panic!(), execution_height: panic!(), evm_root: panic!(),
                system_root: panic!(), execution_hash: panic!(), application: panic!(),
                consensus_height: panic!(), consensus_block_id: panic!(),
            }
        }
        fn main() {}
    "#,
        "E0451",
    );
}

#[test]
fn an_external_native_capability_cannot_be_injected_into_the_anchor_api() {
    compiler::must_reject(
        r#"
        use eve_finality_verifier::{DevelopmentFinalityVerifier, VerifiedDevelopmentHeader,
            authenticate_current_application_version};
        fn inject(verifier: &DevelopmentFinalityVerifier, version: &eve_state::StateVersion,
                  foreign: &VerifiedDevelopmentHeader) {
            let _ = authenticate_current_application_version(verifier, version, foreign.native());
        }
        fn main() {}
    "#,
        "E0061",
    );
}
