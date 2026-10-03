// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::append_length_prefixed::append_length_prefixed;
use crate::recovery::{
    NativeFrame, RecoveryError, bounds::measure_native_frame_bytes::measure_native_frame_bytes,
};
use prost::Message;

pub(crate) fn encode_native_frame(frame: &NativeFrame) -> Result<Vec<u8>, RecoveryError> {
    let mut output = Vec::with_capacity(measure_native_frame_bytes(frame)?);
    append_length_prefixed(&mut output, &frame.block_id.encode_to_vec())?;
    append_length_prefixed(&mut output, &frame.header.encode_to_vec())?;
    append_length_prefixed(&mut output, &frame.commit.encode_to_vec())?;
    Ok(output)
}
