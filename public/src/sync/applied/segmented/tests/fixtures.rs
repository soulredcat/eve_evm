// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::persistence::segmented::required_segmented_metadata_reservation;
use crate::sync::applied::{
    SegmentedAppliedConfig,
    tests::import_fixtures::{ImportChain, import_chain, import_config},
};
use eve_finality_verifier::{
    encode_logical_import_wire, imported_state_commit, initialize_authenticated_import,
    into_imported_state, prepare_authenticated_import,
};
use eve_node_policy::{
    SegmentedRecoveryBounds, available_segmented_allowance, development_segmented_recovery_policy,
};
use eve_state::{
    Bytes, JournalOperation, development_state_budget, estimate_journal_candidate_reservation,
};
use eve_storage::records::segmented::SegmentedCodecLimits;
use std::{path::Path, sync::Arc};

/// Actual signed execution inputs with hash-checked unused code auxiliary data.
pub(crate) fn logical_chain(large: bool) -> ImportChain {
    let mut chain = import_chain();
    let budget = development_state_budget();
    let mut parent = Arc::new(initialize_authenticated_import(&chain.genesis, &budget).unwrap());
    for index in 0..chain.inputs.len() {
        chain.inputs[index].journal.parent = imported_state_commit(&parent).target.clone();
        if large && index == 0 {
            for seed in 1..=255_u8 {
                let code = Bytes::from(vec![seed; 24_576]);
                chain.inputs[index]
                    .journal
                    .operations
                    .push(JournalOperation::PutCode {
                        code_hash: alloy_primitives::keccak256(&code),
                        code,
                    });
            }
        }
        let required = estimate_journal_candidate_reservation(
            &imported_state_commit(&parent).state,
            &chain.inputs[index].journal,
            &budget,
        )
        .unwrap();
        let prepared = prepare_authenticated_import(
            &parent,
            Arc::new(chain.inputs[index].clone()),
            &budget,
            required,
        )
        .unwrap();
        parent = into_imported_state(prepared);
        chain.commits[index + 1] = imported_state_commit(&parent).clone();
    }
    chain.records = chain
        .inputs
        .iter()
        .map(|input| encode_logical_import_wire(input, &budget).unwrap())
        .collect();
    chain
}

pub(crate) fn configuration(path: &Path, chain: &ImportChain) -> SegmentedAppliedConfig {
    let mut application = import_config(path, chain);
    application.repository_budget.maximum_record_bytes = 4_198_400;
    application.repository_budget.maximum_read_bytes = 4_198_400;
    application.repository_budget.maximum_batch_records = 1;
    let codec = SegmentedCodecLimits {
        maximum_logical_bytes: eve_finality_verifier::MAXIMUM_LOGICAL_IMPORT_WIRE_BYTES,
        maximum_chunk_bytes: 4_194_095,
        maximum_segments: 6,
        maximum_payload_bytes: 4_194_304,
    };
    let repository = application.repository_budget;
    let scratch = 2 * codec.maximum_payload_bytes
        + 88
        + repository.maximum_batch_bytes
        + 2 * repository.maximum_read_bytes
        + 4_096;
    let bounds = SegmentedRecoveryBounds {
        maximum_logical_bytes: codec.maximum_logical_bytes as u64,
        maximum_segment_bytes: codec.maximum_payload_bytes as u64,
        segment_header_bytes: 177,
        segment_footer_bytes: 32,
        maximum_segments: 6,
        maximum_marker_bytes: 465,
        opaque_record_header_bytes: 88,
        maximum_opaque_record_bytes: repository.maximum_record_bytes as u64,
        maximum_opaque_read_bytes: repository.maximum_read_bytes as u64,
        maximum_opaque_batch_bytes: repository.maximum_batch_bytes as u64,
        maximum_opaque_batch_records: 1,
        required_metadata_bytes: required_segmented_metadata_reservation().unwrap() as u64,
        required_scratch_bytes: scratch as u64,
        metadata_limit_bytes: 1_048_576,
        scratch_limit_bytes: application.worker_scratch_limit,
        available_auxiliary_bytes: available_segmented_allowance(application.public_budget)
            .unwrap(),
    };
    let policy = development_segmented_recovery_policy(application.public_budget, bounds).unwrap();
    SegmentedAppliedConfig {
        application,
        policy,
        codec,
    }
}
