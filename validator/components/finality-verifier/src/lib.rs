// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! EVE development history, authenticated outcomes and canonical recovery replay.
mod finality;
mod recovery;

pub use finality::{
    AuthenticatedApplicationAnchor, DevelopmentFinalityVerifier, FinalityError,
    VerifiedDevelopmentHeader, authenticate_current_application_version,
    initialize_development_finality, verify_next_development_header,
};

pub use recovery::{
    AuthenticatedCheckpoint, AuthenticatedImportInput, CheckpointError, CheckpointExecutionWitness,
    CheckpointLimits, CheckpointSession, CheckpointWitness, CheckpointWitnessWireError,
    CheckpointWitnessWireKind, CheckpointWitnessWirePreflight, CheckpointWitnessWireStats,
    CompactRecoveryEnvelopeV1, DevelopmentRecoveryState, ImportError, ImportExecutionWireStats,
    ImportLookaheadWireStats, ImportNativeWireStats, ImportWireError, ImportWirePreflight,
    ImportWireSlices, ImportWireStats, ImportedState, ImportedTransition,
    LogicalImportWirePreflight, MAXIMUM_CHECKPOINT_WITNESS_WIRE_BYTES, MAXIMUM_IMPORT_WIRE_BYTES,
    MAXIMUM_LOGICAL_IMPORT_WIRE_BYTES, NativeDataFrame, NativeFrame, RecoveryError,
    VerifiedRecoveryTransition, begin_authenticated_checkpoint, checkpoint_anchor,
    checkpoint_commit, checkpoint_witness_wire_budget, checkpoint_witness_wire_bytes,
    checkpoint_witness_wire_kind, checkpoint_witness_wire_limits, checkpoint_witness_wire_stats,
    checkpoint_witness_wire_version_bytes, decode_authenticated_import_wire,
    decode_checkpoint_witness_wire, decode_compact_recovery_envelope, decode_logical_import_wire,
    encode_authenticated_import_wire, encode_checkpoint_witness_wire,
    encode_compact_recovery_envelope, encode_logical_import_wire, finish_authenticated_checkpoint,
    import_wire_budget, import_wire_bytes, import_wire_slices, import_wire_stats,
    imported_state_anchor, imported_state_commit, imported_transition_input,
    imported_transition_state, initialize_authenticated_import, initialize_development_recovery,
    into_imported_checkpoint_state, into_imported_state, into_recovery_state,
    logical_import_wire_budget, logical_import_wire_bytes, logical_import_wire_slices,
    logical_import_wire_stats, measure_authenticated_import_wire, measure_checkpoint_witness_wire,
    measure_logical_import_wire, preflight_authenticated_import_wire,
    preflight_checkpoint_witness_wire, preflight_logical_import_wire, prepare_authenticated_import,
    prepare_development_recovery, recovery_state_anchor, recovery_state_commit,
    recovery_transition_envelope, recovery_transition_state, required_checkpoint_reservation,
    required_checkpoint_witness_decode_reservation, validate_empty_recovery_envelope_bytes,
    validate_recovery_envelope_bounds, verify_checkpoint_witness,
};
