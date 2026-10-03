// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    scan_native_frame::scan_native_frame, scan_transaction_list::scan_transaction_list,
    types::MAXIMUM_NATIVE_FRAME_BYTES,
};
use crate::recovery::{RecoveryError, decoding::take_length_prefixed::take_length_prefixed};

pub(crate) fn scan_native_data_frame(mut input: &[u8]) -> Result<(), RecoveryError> {
    let frame = take_length_prefixed(&mut input, MAXIMUM_NATIVE_FRAME_BYTES)?;
    scan_native_frame(frame)?;
    scan_transaction_list(input)
}
