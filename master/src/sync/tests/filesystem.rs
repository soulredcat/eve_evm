// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixtures::{fixture, proof_directory};
use crate::sync::{import_master_wire, open_master_follower};

#[test]
fn missing_or_corrupted_completed_proof_cannot_use_local_roots_as_finality() {
    for remove in [false, true] {
        let fixture = fixture();
        let mut follower =
            open_master_follower(fixture.config.clone(), &fixture.chain.genesis).unwrap();
        import_master_wire(
            &mut follower,
            &fixture.wires[0],
            &fixture.chain.commits[1].target,
        )
        .unwrap();
        drop(follower);
        let proof = proof_directory(&fixture).join("00000000000000000001.proof");
        if remove {
            std::fs::remove_file(proof).unwrap();
        } else {
            use std::os::unix::fs::PermissionsExt;
            assert_eq!(
                std::fs::metadata(&proof).unwrap().permissions().mode() & 0o777,
                0o400
            );
            // Deliberate operator-level corruption of this isolated test namespace.
            std::fs::set_permissions(&proof, std::fs::Permissions::from_mode(0o600)).unwrap();
            std::fs::write(proof, b"corrupt completed proof").unwrap();
        }
        assert!(open_master_follower(fixture.config.clone(), &fixture.chain.genesis).is_err());
    }
}

#[test]
fn unexplained_gaps_unicode_names_and_nonregular_files_reject_without_listing_allocations() {
    for name in ["00000000000000000003.proof", "unexplained", "ééééééééééééé"] {
        let fixture = fixture();
        let follower =
            open_master_follower(fixture.config.clone(), &fixture.chain.genesis).unwrap();
        drop(follower);
        std::fs::write(proof_directory(&fixture).join(name), &fixture.wires[0]).unwrap();
        assert!(open_master_follower(fixture.config.clone(), &fixture.chain.genesis).is_err());
    }
    let fixture = fixture();
    let follower = open_master_follower(fixture.config.clone(), &fixture.chain.genesis).unwrap();
    drop(follower);
    std::fs::create_dir(proof_directory(&fixture).join("pending.proof")).unwrap();
    assert!(open_master_follower(fixture.config.clone(), &fixture.chain.genesis).is_err());
}

#[test]
fn symlink_and_hardlinked_native_proof_files_are_refused_before_reading_their_payloads() {
    use std::os::unix::fs::symlink;
    for hardlink in [false, true] {
        let fixture = fixture();
        let follower =
            open_master_follower(fixture.config.clone(), &fixture.chain.genesis).unwrap();
        drop(follower);
        let outside = fixture.root.path().join("outside.proof");
        std::fs::write(&outside, &fixture.wires[0]).unwrap();
        let staged = proof_directory(&fixture).join("pending.proof");
        if hardlink {
            std::fs::hard_link(&outside, &staged).unwrap();
        } else {
            symlink(&outside, &staged).unwrap();
        }
        assert!(open_master_follower(fixture.config.clone(), &fixture.chain.genesis).is_err());
    }
}

#[test]
fn oversized_staging_file_is_refused_by_metadata_before_any_body_allocation() {
    let fixture = fixture();
    let follower = open_master_follower(fixture.config.clone(), &fixture.chain.genesis).unwrap();
    drop(follower);
    let staged = std::fs::File::create(proof_directory(&fixture).join("pending.proof")).unwrap();
    staged
        .set_len(fixture.config.maximum_proof_bytes as u64 + 1)
        .unwrap();
    assert!(open_master_follower(fixture.config.clone(), &fixture.chain.genesis).is_err());
}
