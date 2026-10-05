// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    scan_native_message::scan_native_message,
    types::{
        MAXIMUM_BLOCK_ID_BYTES, MAXIMUM_NATIVE_COMMIT_BYTES, MAXIMUM_NATIVE_FRAME_BYTES,
        MAXIMUM_NATIVE_HEADER_BYTES, NativeMessageKind,
    },
};
use crate::recovery::{RecoveryError, decoding::take_length_prefixed::take_length_prefixed};

pub(crate) fn scan_native_frame(mut input: &[u8]) -> Result<(), RecoveryError> {
    if input.len() > MAXIMUM_NATIVE_FRAME_BYTES {
        return Err(RecoveryError::BudgetExceeded);
    }
    let id = take_length_prefixed(&mut input, MAXIMUM_BLOCK_ID_BYTES)?;
    let header = take_length_prefixed(&mut input, MAXIMUM_NATIVE_HEADER_BYTES)?;
    let commit = take_length_prefixed(&mut input, MAXIMUM_NATIVE_COMMIT_BYTES)?;
    if !input.is_empty() {
        return Err(RecoveryError::NonCanonicalEncoding);
    }
    scan_native_message(id, NativeMessageKind::BlockId)?;
    scan_native_message(header, NativeMessageKind::Header)?;
    scan_native_message(commit, NativeMessageKind::Commit)
}
