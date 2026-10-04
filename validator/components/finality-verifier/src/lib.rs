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
    CheckpointLimits, CheckpointSession, CheckpointWitness, CompactRecoveryEnvelopeV1,
    DevelopmentRecoveryState, ImportError, ImportExecutionWireStats, ImportLookaheadWireStats,
    ImportNativeWireStats, ImportWireError, ImportWirePreflight, ImportWireSlices, ImportWireStats,
    ImportedState, ImportedTransition, LogicalImportWirePreflight, MAXIMUM_IMPORT_WIRE_BYTES,
    MAXIMUM_LOGICAL_IMPORT_WIRE_BYTES, NativeDataFrame, NativeFrame, RecoveryError,
    VerifiedRecoveryTransition, begin_authenticated_checkpoint, checkpoint_anchor,
    checkpoint_commit, decode_authenticated_import_wire, decode_compact_recovery_envelope,
    decode_logical_import_wire, encode_authenticated_import_wire, encode_compact_recovery_envelope,
    encode_logical_import_wire, finish_authenticated_checkpoint, import_wire_budget,
    import_wire_bytes, import_wire_slices, import_wire_stats, imported_state_anchor,
    imported_state_commit, imported_transition_input, imported_transition_state,
    initialize_authenticated_import, initialize_development_recovery,
    into_imported_checkpoint_state, into_imported_state, into_recovery_state,
    logical_import_wire_budget, logical_import_wire_bytes, logical_import_wire_slices,
    logical_import_wire_stats, measure_authenticated_import_wire, measure_logical_import_wire,
    preflight_authenticated_import_wire, preflight_logical_import_wire,
    prepare_authenticated_import, prepare_development_recovery, recovery_state_anchor,
    recovery_state_commit, recovery_transition_envelope, recovery_transition_state,
    required_checkpoint_reservation, validate_empty_recovery_envelope_bytes,
    validate_recovery_envelope_bounds, verify_checkpoint_witness,
};
