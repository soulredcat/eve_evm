// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    decode_native_message::decode_native_message, take_length_prefixed::take_length_prefixed,
};
use crate::recovery::{
    NativeFrame, RecoveryError,
    bounds::{
        scan_native_frame::scan_native_frame,
        types::{MAXIMUM_BLOCK_ID_BYTES, MAXIMUM_NATIVE_COMMIT_BYTES, MAXIMUM_NATIVE_HEADER_BYTES},
    },
};

pub(crate) fn decode_native_frame(mut input: &[u8]) -> Result<NativeFrame, RecoveryError> {
    scan_native_frame(input)?;
    Ok(NativeFrame {
        block_id: decode_native_message(take_length_prefixed(&mut input, MAXIMUM_BLOCK_ID_BYTES)?)?,
        header: decode_native_message(take_length_prefixed(
            &mut input,
            MAXIMUM_NATIVE_HEADER_BYTES,
        )?)?,
        commit: decode_native_message(take_length_prefixed(
            &mut input,
            MAXIMUM_NATIVE_COMMIT_BYTES,
        )?)?,
    })
}
