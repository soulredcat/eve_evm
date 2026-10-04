// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::state_roots_types::StateRootsResponse;
use crate::rpc::{
    RpcContext,
    encoding::quantity,
    errors::rpc_error,
    selectors::{SelectedState, capture_current_rpc_state, selected_verification_mode},
};
use crate::sync::applied::applied_anchor;
use jsonrpsee::types::ErrorObjectOwned;
use serde_json::Value;

/// One current view. EVE_APP_V1 authenticates roots/execution hash; contentDigest
/// remains local auxiliary representation, not a separately certified value.
pub(crate) fn read_state_roots(context: &RpcContext) -> Result<Value, ErrorObjectOwned> {
    let selected = capture_current_rpc_state(context)?;
    let version = &selected.commit().target;
    let authenticated = match &selected {
        SelectedState::Applied { publication } => applied_anchor(publication).is_some(),
        _ => false,
    };
    let response = StateRootsResponse {
        height: quantity(version.height),
        evm_root: version.evm_root.0,
        system_root: version.system_root.0,
        execution_hash: version.execution_hash.0,
        content_digest: version.content_digest,
        application_commitment: version.application.map(|commitment| commitment.0),
        verification_mode: selected_verification_mode(&selected),
        authenticated_finality: authenticated,
    };
    serde_json::to_value(response).map_err(|error| rpc_error(-32603, error.to_string()))
}
