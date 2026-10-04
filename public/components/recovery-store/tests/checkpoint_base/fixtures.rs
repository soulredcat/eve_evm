// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_state::{
    StateVersion, build_state_version, development_state_budget, initialize_development_state,
};
use eve_storage::records::{
    OpaqueRecordBudget, OpaqueRecordCursor, OpaqueRecordIdentity, OpaqueRecordRepository,
    compare_and_append_opaque_records, development_opaque_record_budget,
    opaque_record_bootstrap_cursor, opaque_record_cursor, open_opaque_record_repository,
    segmented::{SegmentedRecoveryAnchor, checkpoints::*},
};
use sha2::{Digest, Sha256};

pub fn limits() -> CheckpointBaseLimits {
    CheckpointBaseLimits {
        maximum_payload_bytes: CHECKPOINT_BASE_MAX_PAYLOAD_BYTES,
    }
}

pub fn reservation() -> usize {
    required_checkpoint_base_encoding_reservation(&limits()).unwrap()
}

pub fn budget() -> OpaqueRecordBudget {
    OpaqueRecordBudget {
        maximum_record_bytes: 8_192,
        maximum_read_bytes: 8_192,
        maximum_batch_bytes: 16_384,
        maximum_batch_records: 1,
        maximum_retained_records: 128,
        block_cache_bytes: 65_536,
        write_buffer_bytes: 65_536,
        ..development_opaque_record_budget()
    }
}

pub fn identity() -> OpaqueRecordIdentity {
    OpaqueRecordIdentity {
        genesis_hash: [1; 32],
        owner: [2; 32],
        domain: [3; 32],
    }
}

pub fn open(path: &std::path::Path) -> OpaqueRecordRepository {
    open_opaque_record_repository(&path.join("store"), identity(), budget()).unwrap()
}

pub fn version(height: u64) -> StateVersion {
    let genesis = eve_development_fixtures::genesis::genesis();
    let budget = development_state_budget();
    let initial = initialize_development_state(&genesis, &budget).unwrap();
    let mut header = initial.block.header.clone();
    header.number = height;
    build_state_version(&initial.state, &header, &budget).unwrap()
}

pub fn prepared(height: u64) -> PreparedCheckpointBaseTarget {
    prepare_checkpoint_base_target(&version(height), &limits(), reservation()).unwrap()
}

pub fn bootstrap(repository: &OpaqueRecordRepository) -> SegmentedRecoveryAnchor {
    SegmentedRecoveryAnchor {
        height: 0,
        cursor: opaque_record_bootstrap_cursor(repository),
        state_binding: [4; 32],
    }
}

pub fn metadata(
    parent: SegmentedRecoveryAnchor,
    physical: OpaqueRecordCursor,
    height: u64,
) -> CheckpointBaseMetadata {
    CheckpointBaseMetadata {
        mode: CheckpointBaseMode::AuthenticatedImport,
        previous_opaque_cursor: physical,
        previous_logical_anchor: parent,
        target_height: height,
        target_state_binding: [5; 32],
        snapshot_manifest_id: [6; 32],
        snapshot_body_hash: [7; 32],
        proof_manifest_id: [8; 32],
        proof_root: [9; 32],
        proof_genesis_height: 0,
        execution_start: 1,
        execution_end: height,
        lookahead_height: height.saturating_add(1),
        retained_start: 0,
        retained_end: height,
    }
}

pub fn inert_parent() -> SegmentedRecoveryAnchor {
    SegmentedRecoveryAnchor {
        height: 0,
        cursor: OpaqueRecordCursor {
            sequence: 0,
            content_hash: [1; 32],
        },
        state_binding: [4; 32],
    }
}

pub fn encode(metadata: CheckpointBaseMetadata, target: &PreparedCheckpointBaseTarget) -> Vec<u8> {
    encode_checkpoint_base(metadata, target, &limits(), reservation()).unwrap()
}

pub fn append(repository: &mut OpaqueRecordRepository, payload: Vec<u8>) -> OpaqueRecordCursor {
    let head = opaque_record_cursor(repository).unwrap();
    compare_and_append_opaque_records(repository, head, &[payload])
        .unwrap()
        .appended
}

pub fn rehash(bytes: &mut [u8]) {
    let footer = bytes.len() - 32;
    let hash: [u8; 32] = Sha256::digest(&bytes[..footer]).into();
    bytes[footer..].copy_from_slice(&hash);
}

pub fn membership_charge(repository: &OpaqueRecordRepository) -> usize {
    required_checkpoint_base_membership_reservation(
        &eve_storage::records::opaque_record_budget(repository),
        &limits(),
    )
    .unwrap()
}
