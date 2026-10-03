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

pub use initialize_authenticated_import::initialize_authenticated_import;
pub use prepare_authenticated_import::prepare_authenticated_import;
pub use types::{AuthenticatedImportInput, ImportError, ImportedState, ImportedTransition};
pub use views::{
    imported_state_anchor, imported_state_commit, imported_transition_input,
    imported_transition_state, into_imported_state,
};
