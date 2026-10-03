// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    decode_native_frame::decode_native_frame, take_length_prefixed::take_length_prefixed,
    take_u32::take_u32,
};
use crate::recovery::{
    NativeDataFrame, RecoveryError,
    bounds::{
        scan_native_data_frame::scan_native_data_frame,
        types::{MAXIMUM_NATIVE_FRAME_BYTES, MAXIMUM_TRANSACTION_BYTES},
    },
};
use eve_state::Bytes;

pub(crate) fn decode_native_data_frame(mut input: &[u8]) -> Result<NativeDataFrame, RecoveryError> {
    scan_native_data_frame(input)?;
    let frame = decode_native_frame(take_length_prefixed(
        &mut input,
        MAXIMUM_NATIVE_FRAME_BYTES,
    )?)?;
    let count = take_u32(&mut input)?;
    let mut transactions = Vec::with_capacity(count);
    for _ in 0..count {
        transactions.push(Bytes::copy_from_slice(take_length_prefixed(
            &mut input,
            MAXIMUM_TRANSACTION_BYTES,
        )?));
    }
    Ok(NativeDataFrame {
        frame,
        transactions,
    })
}
