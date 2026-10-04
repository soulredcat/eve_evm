// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::rpc::errors::rpc_error;
use crate::sync::applied::{AppliedPublication, applied_commit};
use jsonrpsee::types::ErrorObjectOwned;

/// Charge only this retained block's decoder/encoding work, from the same captured owner.
pub(crate) fn estimate_applied_history_reservation(
    publication: &AppliedPublication,
) -> Result<u32, ErrorObjectOwned> {
    let block = &applied_commit(publication).block;
    let raw = block
        .transactions
        .iter()
        .chain(&block.receipts)
        .try_fold(0_usize, |sum, bytes| sum.checked_add(bytes.len()))
        .ok_or_else(|| rpc_error(-32005, "applied history raw-byte accounting overflow"))?;
    let count = block
        .transactions
        .len()
        .checked_add(block.receipts.len())
        .ok_or_else(|| rpc_error(-32005, "applied history count accounting overflow"))?;
    let bytes = raw
        .checked_mul(4)
        .and_then(|bytes| bytes.checked_add(count.checked_mul(512)?))
        .and_then(|bytes| bytes.checked_add(65_536))
        .ok_or_else(|| rpc_error(-32005, "applied history reservation overflow"))?;
    u32::try_from(bytes.div_ceil(1024))
        .map_err(|_| rpc_error(-32005, "applied history reservation exceeds RPC pool units"))
}
