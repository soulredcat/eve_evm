// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::signing::{DurableSigner, signer_status};
use anyhow::{Error, Result};
use eve_consensus_comet::wire::tendermint::privval::RemoteSignerError;
use eve_storage::records::OpaqueRecordCursor;

/// Policy refusals that emit no signature and leave durable signer state unchanged.
const REFUSALS: [&str; 7] = [
    "signer height/round/step regression",
    "conflicting vote at identical height/round/step",
    "conflicting proposal at identical height/round/step",
    "non-nil vote requires canonical execution/data approval",
    "new signature not at current execution height",
    "execution approval parent is stale",
    "current signer parent unavailable",
];

/// Convert a safe policy refusal into the native remote-signer error response.
///
/// The engine treats `RemoteSignerError` as a non-fatal signing failure, including
/// during WAL replay where it re-requests earlier height/round/step messages.
/// Any other error, a moved durable cursor or a fenced signer remains fatal.
/// Only bare policy errors qualify; contextual internal/reexecution faults stay fatal
/// even when their underlying message matches a known refusal.
pub(super) fn refuse_signing_request(
    signer: &DurableSigner,
    cursor_before: &OpaqueRecordCursor,
    error: Error,
) -> Result<RemoteSignerError> {
    let status = signer_status(signer);
    let reason = error.root_cause().to_string();
    if status.fenced
        || &status.cursor != cursor_before
        || error.chain().count() != 1
        || !REFUSALS.contains(&reason.as_str())
    {
        return Err(error);
    }
    Ok(RemoteSignerError {
        code: 1,
        description: reason,
    })
}
