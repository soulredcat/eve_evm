// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointMessageError, CheckpointRequest,
    validate_checkpoint_request_fields::validate_checkpoint_request_fields,
};
pub(super) fn validate_checkpoint_request(
    request: &CheckpointRequest,
) -> Result<(), CheckpointMessageError> {
    if request.genesis.height != 0 {
        return Err(CheckpointMessageError::MalformedEncoding);
    }
    validate_checkpoint_request_fields(request.height, &request.kind)
}
