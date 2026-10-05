// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{HistoryError, NativeHistoryVerifier};
use crate::{
    consensus::{
        certificates::CertificateError,
        signing::{canonical_block_id, normalize_timestamp},
    },
    wire::tendermint::types::Header,
};

pub(super) fn validate_header_successor(
    history: &NativeHistoryVerifier,
    header: &Header,
) -> Result<(), HistoryError> {
    if header.height
        != history
            .height
            .checked_add(1)
            .ok_or(HistoryError::HeightOverflow)?
    {
        return Err(HistoryError::WrongSuccessorHeight);
    }
    match &history.block_id {
        Some(previous) if header.last_block_id.as_ref() != Some(previous) => {
            return Err(HistoryError::WrongParentBlock);
        }
        None if canonical_block_id(header.last_block_id.as_ref())
            .map_err(|error| HistoryError::Certificate(CertificateError::Signing(error)))?
            .is_some() =>
        {
            return Err(HistoryError::WrongParentBlock);
        }
        _ => {}
    }
    if let Some(anchor) = history.genesis_app_hash
        && header.app_hash.as_slice() != anchor
    {
        return Err(HistoryError::WrongGenesisApplicationHash);
    }
    let current = normalize_timestamp(header.time.as_ref())
        .map_err(|error| HistoryError::Certificate(CertificateError::Signing(error)))?;
    if let Some(previous) = &history.header {
        let previous = normalize_timestamp(previous.time.as_ref())
            .map_err(|error| HistoryError::Certificate(CertificateError::Signing(error)))?;
        if (current.seconds, current.nanos) <= (previous.seconds, previous.nanos) {
            return Err(HistoryError::NonIncreasingTime);
        }
    }
    Ok(())
}
