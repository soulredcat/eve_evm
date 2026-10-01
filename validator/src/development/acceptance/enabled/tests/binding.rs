// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::support::fixture;
use crate::development::acceptance::enabled::load_acceptance_fixture;
use eve_state::SecurityProfile;

#[test]
fn acceptance_capability_requires_classical_valid_genesis_and_exact_supplied_manifest() {
    let mut material = fixture();
    assert!(material.load().future.len() == 1);
    assert!(load_acceptance_fixture(None, &material.genesis).is_err());
    for profile in [
        SecurityProfile::HybridExperimental,
        SecurityProfile::PqProfileVerified,
    ] {
        let mut wrong = material.genesis.clone();
        wrong.profile = profile;
        assert!(load_acceptance_fixture(Some(&material.path), &wrong).is_err());
    }
    let contract = material
        .genesis
        .accounts
        .iter_mut()
        .find(|account| account.address == material.contract)
        .unwrap();
    contract.code = hex::decode("60006000fd").unwrap().into();
    assert!(load_acceptance_fixture(Some(&material.path), &material.genesis).is_err());
    assert!(
        load_acceptance_fixture(None, &material.genesis)
            .unwrap()
            .is_none()
    );
}

#[test]
fn semantically_identical_manifest_bytes_cannot_change_genesis_bound_digest() {
    let material = fixture();
    material.load();
    std::fs::write(
        &material.path,
        serde_json::to_vec_pretty(&material.document).unwrap(),
    )
    .unwrap();
    assert!(load_acceptance_fixture(Some(&material.path), &material.genesis).is_err());
}

#[test]
fn loader_rejects_oversized_symlink_and_unknown_manifest_fields() {
    let mut material = fixture();
    material.document["operator_schedule"] = serde_json::json!(true);
    super::support::rebind(&mut material);
    assert!(load_acceptance_fixture(Some(&material.path), &material.genesis).is_err());
    std::fs::write(&material.path, vec![b' '; 16385]).unwrap();
    assert!(load_acceptance_fixture(Some(&material.path), &material.genesis).is_err());
    #[cfg(unix)]
    {
        let alias = material.root.path().join("alias.json");
        std::os::unix::fs::symlink(&material.path, &alias).unwrap();
        assert!(load_acceptance_fixture(Some(&alias), &material.genesis).is_err());
    }
}
