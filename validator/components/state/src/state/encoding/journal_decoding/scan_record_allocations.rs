// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::{StateError, state::encoding::take_list};

/// Count all RLP leaf payloads; semantic record decoding stays in its canonical codec.
pub(crate) fn scan_record_allocations(bytes: &[u8]) -> Result<(usize, usize), StateError> {
    let mut remaining = bytes;
    let record = take_list(&mut remaining)?;
    if !remaining.is_empty() {
        return Err(StateError::MalformedEncoding);
    }
    // Existing schemas have root/value/optional lists only. No recursive traversal.
    let mut stack = [&[][..]; 3];
    stack[0] = record;
    let mut depth = 0_usize;
    let mut payload_bytes = 0_usize;
    let mut leaves = 0_usize;
    loop {
        if stack[depth].is_empty() {
            if depth == 0 {
                break;
            }
            depth -= 1;
            continue;
        }
        let mut current = stack[depth];
        let header =
            alloy_rlp::Header::decode(&mut current).map_err(|_| StateError::MalformedEncoding)?;
        let (payload, tail) = current.split_at(header.payload_length);
        stack[depth] = tail;
        if header.list {
            if depth + 1 >= stack.len() {
                return Err(StateError::MalformedEncoding);
            }
            depth += 1;
            stack[depth] = payload;
        } else {
            payload_bytes = payload_bytes
                .checked_add(payload.len())
                .ok_or(StateError::ArithmeticOverflow)?;
            leaves = leaves
                .checked_add(1)
                .ok_or(StateError::ArithmeticOverflow)?;
        }
    }
    Ok((payload_bytes, leaves))
}
