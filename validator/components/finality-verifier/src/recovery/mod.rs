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
    AuthenticatedImportInput, ImportError, ImportExecutionWireStats, ImportLookaheadWireStats,
    ImportNativeWireStats, ImportWireError, ImportWirePreflight, ImportWireSlices, ImportWireStats,
    ImportedState, ImportedTransition, LogicalImportWirePreflight, MAXIMUM_IMPORT_WIRE_BYTES,
    MAXIMUM_LOGICAL_IMPORT_WIRE_BYTES, decode_authenticated_import_wire,
    decode_logical_import_wire, encode_authenticated_import_wire, encode_logical_import_wire,
    import_wire_budget, import_wire_bytes, import_wire_slices, import_wire_stats,
    imported_state_anchor, imported_state_commit, imported_transition_input,
    imported_transition_state, initialize_authenticated_import, into_imported_state,
    logical_import_wire_budget, logical_import_wire_bytes, logical_import_wire_slices,
    logical_import_wire_stats, measure_authenticated_import_wire, measure_logical_import_wire,
    preflight_authenticated_import_wire, preflight_logical_import_wire,
    prepare_authenticated_import,
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
