// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{create_fixture, temporary_fixture, test_key, vote_request};
use crate::consensus::signing::{
    open_durable_signer, sign_vote, signer_status, types::SimulatedSignerFailure,
};
use eve_consensus_comet::consensus::{
    certificates::verify_native_ed25519_signature, signing::encode_vote_sign_bytes,
};
use eve_storage::records::development_opaque_record_budget;

#[test]
fn actual_child_exit_after_synced_history_replays_exactly() {
    if std::env::var("EVE_B3_SIGNER_CRASH_CHILD").as_deref() == Ok("1") {
        let path = std::path::PathBuf::from(std::env::var_os("EVE_B3_SIGNER_CHILD_ROOT").unwrap());
        let allowed = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("local-tests/b3-preparation/signing-unit")
            .canonicalize()
            .unwrap();
        assert!(path.canonicalize().unwrap().starts_with(allowed));
        let fixture = create_fixture(&path);
        let mut signer = open_durable_signer(
            &fixture.root.join("signing"),
            fixture.config.clone(),
            test_key(1),
            fixture.service,
            development_opaque_record_budget(),
        )
        .unwrap();
        signer.simulated_failure = Some(SimulatedSignerFailure::ExitAfterSync);
        sign_vote(&mut signer, vote_request(&fixture.config), None).unwrap();
        panic!("child returned past the synced signing boundary");
    }
    let (directory, fixture) = temporary_fixture();
    drop(fixture);
    let child = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "consensus::signing::tests::process_retry::actual_child_exit_after_synced_history_replays_exactly"])
        .env("EVE_B3_SIGNER_CRASH_CHILD", "1").env("EVE_B3_SIGNER_CHILD_ROOT", directory.path())
        .stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null()).status().unwrap();
    assert_eq!(child.code(), Some(71));
    let fixture = create_fixture(directory.path());
    let mut signer = open_durable_signer(
        &fixture.root.join("signing"),
        fixture.config.clone(),
        test_key(1),
        fixture.service,
        development_opaque_record_budget(),
    )
    .unwrap();
    assert_eq!(signer_status(&signer).cursor.sequence, 1);
    let vote = sign_vote(&mut signer, vote_request(&fixture.config), None).unwrap();
    assert!(
        verify_native_ed25519_signature(
            &fixture.config.expected_public_key,
            &encode_vote_sign_bytes(&fixture.config.chain_id, &vote).unwrap(),
            &vote.signature
        )
        .is_ok()
    );
    assert_eq!(signer_status(&signer).cursor.sequence, 1);
}
