// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    native_field_rule::native_field_rule,
    types::{NativeFieldRule, NativeMessageKind},
};
use crate::recovery::{RecoveryError, decoding::take_protobuf_varint::take_protobuf_varint};
use eve_consensus_comet::consensus::certificates::MAX_DEVELOPMENT_VALIDATORS;

/// Checks all nested lengths/counts without allocating decoded protobuf objects.
pub(crate) fn scan_native_message(
    mut input: &[u8],
    kind: NativeMessageKind,
) -> Result<(), RecoveryError> {
    let mut previous = 0_u64;
    let mut signatures = 0_usize;
    while !input.is_empty() {
        let key = take_protobuf_varint(&mut input)?;
        let field = key >> 3;
        let rule = native_field_rule(kind, field)?;
        let repeated = matches!(rule, NativeFieldRule::Signatures);
        if field < previous || (field == previous && !repeated) {
            return Err(RecoveryError::NonCanonicalEncoding);
        }
        previous = field;
        match rule {
            NativeFieldRule::Varint => {
                if key & 7 != 0 {
                    return Err(RecoveryError::MalformedEncoding);
                }
                if take_protobuf_varint(&mut input)? == 0 {
                    return Err(RecoveryError::NonCanonicalEncoding);
                }
            }
            _ => {
                if key & 7 != 2 {
                    return Err(RecoveryError::MalformedEncoding);
                }
                let size = usize::try_from(take_protobuf_varint(&mut input)?)
                    .map_err(|_| RecoveryError::BudgetExceeded)?;
                let maximum = match rule {
                    NativeFieldRule::Bytes(maximum) | NativeFieldRule::Message(_, maximum) => {
                        maximum
                    }
                    NativeFieldRule::Signatures => 128,
                    NativeFieldRule::Varint => return Err(RecoveryError::MalformedEncoding),
                };
                if size > maximum {
                    return Err(RecoveryError::BudgetExceeded);
                }
                let value = input.get(..size).ok_or(RecoveryError::MalformedEncoding)?;
                input = &input[size..];
                match rule {
                    NativeFieldRule::Bytes(_) if value.is_empty() => {
                        return Err(RecoveryError::NonCanonicalEncoding);
                    }
                    NativeFieldRule::Message(nested, _) => scan_native_message(value, nested)?,
                    NativeFieldRule::Signatures => {
                        signatures += 1;
                        if signatures > MAX_DEVELOPMENT_VALIDATORS {
                            return Err(RecoveryError::BudgetExceeded);
                        }
                        scan_native_message(value, NativeMessageKind::CommitSig)?;
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}
