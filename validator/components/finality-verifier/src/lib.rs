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
    AuthenticatedImportInput, CompactRecoveryEnvelopeV1, DevelopmentRecoveryState, ImportError,
    ImportExecutionWireStats, ImportLookaheadWireStats, ImportNativeWireStats, ImportWireError,
    ImportWirePreflight, ImportWireSlices, ImportWireStats, ImportedState, ImportedTransition,
    MAXIMUM_IMPORT_WIRE_BYTES, NativeDataFrame, NativeFrame, RecoveryError,
    VerifiedRecoveryTransition, decode_authenticated_import_wire, decode_compact_recovery_envelope,
    encode_authenticated_import_wire, encode_compact_recovery_envelope, import_wire_budget,
    import_wire_bytes, import_wire_slices, import_wire_stats, imported_state_anchor,
    imported_state_commit, imported_transition_input, imported_transition_state,
    initialize_authenticated_import, initialize_development_recovery, into_imported_state,
    into_recovery_state, measure_authenticated_import_wire, preflight_authenticated_import_wire,
    prepare_authenticated_import, prepare_development_recovery, recovery_state_anchor,
    recovery_state_commit, recovery_transition_envelope, recovery_transition_state,
    validate_empty_recovery_envelope_bytes, validate_recovery_envelope_bounds,
};
