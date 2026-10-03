// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    append_length_prefixed::append_length_prefixed, encode_native_frame::encode_native_frame,
};
use crate::recovery::{
    NativeDataFrame, RecoveryError,
    bounds::{
        measure_native_frame_bytes::measure_native_frame_bytes,
        measure_transaction_list_bytes::measure_transaction_list_bytes,
    },
};

pub(crate) fn encode_native_data_frame(frame: &NativeDataFrame) -> Result<Vec<u8>, RecoveryError> {
    let maximum = 4
        + measure_native_frame_bytes(&frame.frame)?
        + measure_transaction_list_bytes(&frame.transactions)?;
    let mut output = Vec::with_capacity(maximum);
    append_length_prefixed(&mut output, &encode_native_frame(&frame.frame)?)?;
    let count =
        u32::try_from(frame.transactions.len()).map_err(|_| RecoveryError::BudgetExceeded)?;
    output.extend_from_slice(&count.to_be_bytes());
    for transaction in &frame.transactions {
        append_length_prefixed(&mut output, transaction.as_ref())?;
    }
    Ok(output)
}
