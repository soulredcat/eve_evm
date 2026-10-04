// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_development_fixtures::recovery::{RecoveryChain, recovery_chain};
use eve_state::{encode_state_commit, preflight_state_commit};
use eve_storage::checkpoints::{
    checkpoint_manifest_id, create_checkpoint_manifest,
    messages::{
        CheckpointMessageLimits, CheckpointRequest, CheckpointRequestKind, CheckpointResponse,
        checkpoint_message_storage_limits, encode_checkpoint_response,
    },
    preflight_checkpoint_manifest, required_checkpoint_metadata_reservation,
};

pub(super) struct Fixture {
    pub chain: RecoveryChain,
    pub limits: CheckpointMessageLimits,
    pub manifest: Vec<u8>,
    pub id: [u8; 32],
    pub body: Vec<u8>,
}
pub(super) fn fixture() -> Fixture {
    let chain = recovery_chain();
    let limits = CheckpointMessageLimits {
        logical: eve_state::development_state_budget(),
        maximum_body_bytes: 32 * 1_048_576,
        maximum_manifest_bytes: 262_144,
        maximum_chunk_bytes: 4_194_304,
    };
    let body = encode_state_commit(&chain.commits[1], &limits.logical).unwrap();
    let storage = checkpoint_message_storage_limits(&limits, 64).unwrap();
    let checked = preflight_state_commit(&body, &limits.logical).unwrap();
    let manifest = create_checkpoint_manifest(
        &checked,
        &chain.commits[1].target,
        &storage,
        required_checkpoint_metadata_reservation(&storage).unwrap(),
    )
    .unwrap();
    let version = eve_state::encode_state_version(&chain.commits[1].target).unwrap();
    let id = checkpoint_manifest_id(
        &preflight_checkpoint_manifest(&manifest, &version, &storage).unwrap(),
    );
    Fixture {
        chain,
        limits,
        manifest,
        id,
        body,
    }
}
impl Fixture {
    pub(super) fn request(&self, kind: CheckpointRequestKind) -> CheckpointRequest {
        CheckpointRequest {
            genesis: self.chain.commits[0].target.clone(),
            height: 1,
            kind,
        }
    }
    pub(super) fn manifest_response(&self) -> CheckpointResponse {
        CheckpointResponse::Manifest {
            target: self.chain.commits[1].target.clone(),
            durable_tip: Box::new(self.chain.commits[3].target.clone()),
            manifest_id: self.id,
            manifest: self.manifest.clone(),
        }
    }
    pub(super) fn chunk_response(&self, index: u32) -> CheckpointResponse {
        let offset = index as usize * 64;
        let data = &self.body[offset..(offset + 64).min(self.body.len())];
        CheckpointResponse::Chunk {
            target: self.chain.commits[1].target.clone(),
            manifest_id: self.id,
            body_sha256: eve_state::hash_state_delta_bytes(&self.body),
            total_length: self.body.len() as u64,
            index,
            chunk_bytes: 64,
            data: eve_state::Bytes::copy_from_slice(data),
        }
    }
    pub(super) fn execution_response(&self) -> CheckpointResponse {
        CheckpointResponse::Execution {
            target: self.chain.commits[1].target.clone(),
            block: Box::new(self.chain.commits[1].block.clone()),
        }
    }
    pub(super) fn encoded(&self, response: &CheckpointResponse) -> Vec<u8> {
        encode_checkpoint_response(response, &self.limits).unwrap()
    }
}
