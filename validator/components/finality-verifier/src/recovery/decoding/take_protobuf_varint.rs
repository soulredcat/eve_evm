// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::recovery::RecoveryError;

pub(crate) fn take_protobuf_varint(input: &mut &[u8]) -> Result<u64, RecoveryError> {
    let mut value = 0_u64;
    for index in 0..10 {
        let byte = *input.first().ok_or(RecoveryError::MalformedEncoding)?;
        *input = &input[1..];
        if index == 9 && byte > 1 {
            return Err(RecoveryError::MalformedEncoding);
        }
        value |= u64::from(byte & 0x7f) << (index * 7);
        if byte & 0x80 == 0 {
            if index > 0 && byte == 0 {
                return Err(RecoveryError::NonCanonicalEncoding);
            }
            return Ok(value);
        }
    }
    Err(RecoveryError::MalformedEncoding)
}
