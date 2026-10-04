// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{SelectedState, capture_current_rpc_state, resolve_applied_selector};
use crate::rpc::{RpcContext, errors::rpc_error};
use jsonrpsee::types::ErrorObjectOwned;
use serde_json::Value;

pub(super) fn capture_applied_selected_state(
    context: &RpcContext,
    selector: &Value,
) -> Result<SelectedState, ErrorObjectOwned> {
    let selected = capture_current_rpc_state(context)?;
    let SelectedState::Applied { publication } = &selected else {
        return Err(rpc_error(-32603, "applied source selection mismatch"));
    };
    resolve_applied_selector(publication, selector)?;
    Ok(selected)
}
