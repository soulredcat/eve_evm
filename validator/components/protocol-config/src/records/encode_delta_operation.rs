// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::encoding::encode_list;

use super::{
    DeltaOperation, RecordError, encode_system_record, hash_system_key,
    namespace_bytes::namespace_bytes,
};

pub fn encode_delta_operation(operation: &DeltaOperation) -> Result<Vec<u8>, RecordError> {
    hash_system_key(operation.namespace, &operation.logical_key)?;
    let mut fields = vec![
        alloy_rlp::encode(1_u8),
        alloy_rlp::encode(namespace_bytes(operation.namespace)),
        alloy_rlp::encode(operation.logical_key.as_ref()),
        alloy_rlp::encode(u8::from(operation.value.is_some())),
    ];
    if let Some(value) = &operation.value {
        if value.namespace != operation.namespace || value.logical_key != operation.logical_key {
            return Err(RecordError::InvalidRecord);
        }
        fields.push(encode_system_record(value)?);
    }
    Ok(encode_list(&fields))
}
