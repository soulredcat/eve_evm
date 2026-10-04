// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{super::*, fixtures::limits, pending_metadata_fixture::*};
use crate::sync::applied::{capture_applied_state, finish_applied_state_service};
use std::{fs::File, sync::Arc};

#[test]
fn unknown_future_or_oversized_metadata_refuses_before_any_other_pending_repair() {
    for proof in [false, true] {
        let fixture = fixture();
        let old = capture_applied_state(&fixture.reader).unwrap();
        for future in [false, true] {
            let staging = copied_staging(&fixture, proof, true);
            let id = if proof {
                &fixture.artifacts.proof_id
            } else {
                &fixture.artifacts.content.content_id
            };
            let path = namespace(&staging, id);
            let known = if proof {
                b"EVE_CHECKPOINT_PROOF_V1\0".as_slice()
            } else {
                b"EVE_CHECKPOINT_STORE_V1\0".as_slice()
            };
            std::fs::write(path.join("manifest.pending"), known).unwrap();
            let unknown = if future {
                vec![0x91_u8; 81]
            } else {
                b"partial".to_vec()
            };
            std::fs::write(path.join("complete.pending"), &unknown).unwrap();
            let root = File::open(staging.path()).unwrap();
            let result = if proof {
                repair_applied_checkpoint_proof_pending(
                    &fixture.artifacts.content,
                    &root,
                    proof_manifest(&fixture),
                )
            } else {
                repair_applied_checkpoint_content_pending(
                    &fixture.owner,
                    &root,
                    content_manifest(&fixture),
                    &fixture.target_encoding,
                    limits(),
                )
            };
            assert!(matches!(
                result,
                Err(CheckpointAppliedError::PendingMetadataRefused)
            ));
            assert_eq!(std::fs::read(path.join("manifest.pending")).unwrap(), known);
            assert_eq!(
                std::fs::read(path.join("complete.pending")).unwrap(),
                unknown
            );
        }
        assert!(Arc::ptr_eq(
            &old,
            &capture_applied_state(&fixture.reader).unwrap()
        ));
        drop(fixture.artifacts);
        drop(finish_applied_state_service(fixture.owner));
    }
}

#[test]
fn actual_foreign_manifest_future_codec_and_oversized_manifest_are_preserved() {
    for proof in [false, true] {
        let fixture = fixture();
        for selector in 0..3 {
            let staging = copied_staging(&fixture, proof, true);
            let id = if proof {
                &fixture.artifacts.proof_id
            } else {
                &fixture.artifacts.content.content_id
            };
            let source = if proof {
                proof_manifest(&fixture)
            } else {
                content_manifest(&fixture)
            };
            let mut bytes = source.to_vec();
            match selector {
                0 => bytes[if proof { 112 } else { 44 }] ^= 1,
                1 => bytes[24] = 1,
                _ => bytes.resize(
                    if proof {
                        limits().proofs.maximum_manifest_bytes + 1
                    } else {
                        limits().content.maximum_manifest_bytes + 1
                    },
                    0x92,
                ),
            }
            let path = namespace(&staging, id).join("manifest.pending");
            std::fs::write(&path, &bytes).unwrap();
            let root = File::open(staging.path()).unwrap();
            let result = if proof {
                repair_applied_checkpoint_proof_pending(
                    &fixture.artifacts.content,
                    &root,
                    proof_manifest(&fixture),
                )
            } else {
                repair_applied_checkpoint_content_pending(
                    &fixture.owner,
                    &root,
                    content_manifest(&fixture),
                    &fixture.target_encoding,
                    limits(),
                )
            };
            assert!(matches!(
                result,
                Err(CheckpointAppliedError::PendingMetadataRefused)
            ));
            assert_eq!(std::fs::read(path).unwrap(), bytes);
        }
        drop(fixture.artifacts);
        drop(finish_applied_state_service(fixture.owner));
    }
}
