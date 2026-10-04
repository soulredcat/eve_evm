// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_development_fixtures::recovery::{RecoveryChain, recovery_chain};
use eve_state::{
    development_state_budget, encode_state_commit, encode_state_version, preflight_state_commit,
};
use eve_storage::checkpoints::{
    checkpoint_manifest_id, create_checkpoint_manifest,
    messages::{CheckpointMessageLimits, checkpoint_message_storage_limits},
    preflight_checkpoint_manifest, required_checkpoint_metadata_reservation,
};
pub fn fixture() -> (
    RecoveryChain,
    CheckpointMessageLimits,
    Vec<u8>,
    Vec<u8>,
    [u8; 32],
) {
    let chain = recovery_chain();
    let limits = CheckpointMessageLimits {
        logical: development_state_budget(),
        maximum_body_bytes: 32 * 1_048_576,
        maximum_manifest_bytes: 262_144,
        maximum_chunk_bytes: 4_194_304,
    };
    let body = encode_state_commit(&chain.commits[1], &limits.logical).unwrap();
    let storage = checkpoint_message_storage_limits(&limits, 64).unwrap();
    let sealed = preflight_state_commit(&body, &limits.logical).unwrap();
    let manifest = create_checkpoint_manifest(
        &sealed,
        &chain.commits[1].target,
        &storage,
        required_checkpoint_metadata_reservation(&storage).unwrap(),
    )
    .unwrap();
    let version = encode_state_version(&chain.commits[1].target).unwrap();
    let id = checkpoint_manifest_id(
        &preflight_checkpoint_manifest(&manifest, &version, &storage).unwrap(),
    );
    (chain, limits, body, manifest, id)
}
