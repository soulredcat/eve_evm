// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::ImportWireError;
use crate::recovery::{
    bounds::{
        native_field_rule::native_field_rule,
        types::{NativeFieldRule, NativeMessageKind},
    },
    decoding::take_protobuf_varint::take_protobuf_varint,
};

/// Count fields only AFTER scan_native_frame validates the pinned message grammar.
pub(super) fn count_import_commit_signatures(mut input: &[u8]) -> Result<usize, ImportWireError> {
    let mut count = 0_usize;
    while !input.is_empty() {
        let key = take_protobuf_varint(&mut input).map_err(ImportWireError::Recovery)?;
        let rule = native_field_rule(NativeMessageKind::Commit, key >> 3)
            .map_err(ImportWireError::Recovery)?;
        if matches!(rule, NativeFieldRule::Varint) {
            take_protobuf_varint(&mut input).map_err(ImportWireError::Recovery)?;
        } else {
            let size = usize::try_from(
                take_protobuf_varint(&mut input).map_err(ImportWireError::Recovery)?,
            )
            .map_err(|_| ImportWireError::BudgetExceeded)?;
            input = input
                .get(size..)
                .ok_or(ImportWireError::MalformedEncoding)?;
            if matches!(rule, NativeFieldRule::Signatures) {
                count = count
                    .checked_add(1)
                    .ok_or(ImportWireError::BudgetExceeded)?;
            }
        }
    }
    Ok(count)
}
