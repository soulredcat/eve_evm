// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Certified state import is distinct from independent canonical EVM replay.
mod initialize_authenticated_import;
mod prepare_authenticated_import;
mod types;
mod validate_import_execution_context;
mod validate_import_execution_history;
mod validate_import_input_bounds;
mod validate_import_parent;
mod views;
mod wire;

pub use initialize_authenticated_import::initialize_authenticated_import;
pub use prepare_authenticated_import::prepare_authenticated_import;
pub use types::{AuthenticatedImportInput, ImportError, ImportedState, ImportedTransition};
pub use views::{
    imported_state_anchor, imported_state_commit, imported_transition_input,
    imported_transition_state, into_imported_state,
};
pub use wire::{
    ImportExecutionWireStats, ImportLookaheadWireStats, ImportNativeWireStats, ImportWireError,
    ImportWirePreflight, ImportWireSlices, ImportWireStats, MAXIMUM_IMPORT_WIRE_BYTES,
    decode_authenticated_import_wire, encode_authenticated_import_wire, import_wire_budget,
    import_wire_bytes, import_wire_slices, import_wire_stats, measure_authenticated_import_wire,
    preflight_authenticated_import_wire,
};
