// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

pub(in crate::recovery) mod bounds;
pub(in crate::recovery) mod decoding;
pub(in crate::recovery) mod encoding;
pub(in crate::recovery) mod import;
pub(in crate::recovery) mod initialization;
pub(in crate::recovery) mod replay;
pub(in crate::recovery) mod types;
pub(in crate::recovery) mod verification;
pub(in crate::recovery) mod views;

pub use bounds::validate_empty_recovery_envelope_bytes::validate_empty_recovery_envelope_bytes;
pub use bounds::validate_recovery_envelope_bounds::validate_recovery_envelope_bounds;
pub use decoding::decode_compact_recovery_envelope::decode_compact_recovery_envelope;
pub use encoding::encode_compact_recovery_envelope::encode_compact_recovery_envelope;
pub use import::{
    AuthenticatedCheckpoint, AuthenticatedImportInput, CheckpointError, CheckpointExecutionWitness,
    CheckpointLimits, CheckpointSession, CheckpointWitness, CheckpointWitnessWireError,
    CheckpointWitnessWireKind, CheckpointWitnessWirePreflight, CheckpointWitnessWireStats,
    ImportError, ImportExecutionWireStats, ImportLookaheadWireStats, ImportNativeWireStats,
    ImportWireError, ImportWirePreflight, ImportWireSlices, ImportWireStats, ImportedState,
    ImportedTransition, LogicalImportWirePreflight, MAXIMUM_CHECKPOINT_WITNESS_WIRE_BYTES,
    MAXIMUM_IMPORT_WIRE_BYTES, MAXIMUM_LOGICAL_IMPORT_WIRE_BYTES, begin_authenticated_checkpoint,
    checkpoint_anchor, checkpoint_commit, checkpoint_witness_wire_budget,
    checkpoint_witness_wire_bytes, checkpoint_witness_wire_kind, checkpoint_witness_wire_limits,
    checkpoint_witness_wire_stats, checkpoint_witness_wire_version_bytes,
    decode_authenticated_import_wire, decode_checkpoint_witness_wire, decode_logical_import_wire,
    encode_authenticated_import_wire, encode_checkpoint_witness_wire, encode_logical_import_wire,
    finish_authenticated_checkpoint, import_wire_budget, import_wire_bytes, import_wire_slices,
    import_wire_stats, imported_state_anchor, imported_state_commit, imported_transition_input,
    imported_transition_state, initialize_authenticated_import, into_imported_checkpoint_state,
    into_imported_state, logical_import_wire_budget, logical_import_wire_bytes,
    logical_import_wire_slices, logical_import_wire_stats, measure_authenticated_import_wire,
    measure_checkpoint_witness_wire, measure_logical_import_wire,
    preflight_authenticated_import_wire, preflight_checkpoint_witness_wire,
    preflight_logical_import_wire, prepare_authenticated_import, required_checkpoint_reservation,
    required_checkpoint_witness_decode_reservation, verify_checkpoint_witness,
};
pub use initialization::initialize_development_recovery::initialize_development_recovery;
pub use replay::prepare_development_recovery::prepare_development_recovery;
pub use types::{
    CompactRecoveryEnvelopeV1, DevelopmentRecoveryState, NativeDataFrame, NativeFrame,
    RecoveryError, VerifiedRecoveryTransition,
};
pub use views::into_recovery_state::into_recovery_state;
pub use views::recovery_state_anchor::recovery_state_anchor;
pub use views::recovery_state_commit::recovery_state_commit;
pub use views::recovery_transition_envelope::recovery_transition_envelope;
pub use views::recovery_transition_state::recovery_transition_state;
