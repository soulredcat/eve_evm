// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Canonical complete logical state, journals and commitments.
//! These data contracts and consistency checks do not authenticate finality.

mod state;

pub use alloy_consensus::Header;
pub use alloy_primitives::{Address, B256, Bytes, U256};
pub use eve_protocol_config::genesis::{
    DevelopmentGenesis, GenesisAccount, GenesisValidator, development_economics,
};
pub use eve_protocol_config::network::SecurityProfile;
pub use eve_protocol_config::records::{
    ApplicationCommitment, EvmStateRoot, ExecutionBlockHash, GenesisHash, SystemNamespace,
    SystemRecord, SystemStateRoot, SystemValue, encode_system_record, hash_system_key,
};
pub use state::commitments::{
    compute_commit_identity, compute_evm_root, compute_state_content_digest, compute_system_root,
};
pub use state::encoding::{
    JournalDecodePreflight, decode_block_payload, decode_state_commit, decode_state_journal,
    decode_state_version, decode_system_record, encode_block_payload, encode_state_commit,
    encode_state_version, preflight_state_journal,
};
pub use state::genesis::{
    estimate_genesis_initialization_reservation, initialize_development_state,
};
pub use state::journals::{
    apply_state_journal, apply_state_journal_reserved, encode_state_journal,
    estimate_journal_candidate_reservation, estimate_journal_wire_candidate_reservation,
    measure_state_journal_bytes, project_state_journal,
};
pub use state::limits::{
    BOUNDED_STATE_CODEC_SCRATCH_BYTES, development_state_budget, measure_complete_state_bytes,
};
pub use state::validation::{
    build_state_commit, build_state_version, validate_block_hash_history, validate_block_payload,
    validate_complete_state, validate_retained_block, validate_state_commit,
    validate_state_version,
};
pub use state::views::{
    StateView, capture_state_view, read_account, read_code, read_execution_hash, read_storage,
    read_system, view_state, view_version,
};

pub use state::proofs::{
    AccountProof, ProofLimits, StateProofError, StorageProof, build_account_proof,
    estimate_proof_reservation,
};
pub use state::types::{
    BlockPayload, CompleteState, JournalOperation, StateAccount, StateBudget, StateCommit,
    StateError, StateIdentity, StateJournal, StateVersion,
};
