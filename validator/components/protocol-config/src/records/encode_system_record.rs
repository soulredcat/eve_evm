// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::encoding::{encode_list, encode_optional_height};

use super::{
    RecordError, SystemNamespace, SystemRecord, SystemValue, hash_system_key,
    namespace_bytes::namespace_bytes,
};

pub fn encode_system_record(record: &SystemRecord) -> Result<Vec<u8>, RecordError> {
    if record.schema_version != 1 {
        return Err(RecordError::InvalidVersion);
    }
    hash_system_key(record.namespace, &record.logical_key)?;
    let (namespace, fields) = match &record.value {
        SystemValue::Validator {
            owner,
            key,
            power,
            activation,
            removal,
            key_epoch,
        } => {
            if owner.is_zero()
                || *power == 0
                || *activation == 0
                || removal.is_some_and(|height| height < *activation)
            {
                return Err(RecordError::InvalidRecord);
            }
            (
                SystemNamespace::Validator,
                vec![
                    alloy_rlp::encode(*owner),
                    alloy_rlp::encode(key.as_slice()),
                    alloy_rlp::encode(*power),
                    alloy_rlp::encode(*activation),
                    encode_optional_height(*removal),
                    alloy_rlp::encode(*key_epoch),
                ],
            )
        }
        SystemValue::Fee {
            burned,
            node_pool,
            validator_pool,
        } => (
            SystemNamespace::Fee,
            vec![
                alloy_rlp::encode(*burned),
                alloy_rlp::encode(*node_pool),
                alloy_rlp::encode(*validator_pool),
            ],
        ),
        SystemValue::Reward {
            owner,
            role,
            liability,
            index,
        } => {
            if owner.is_zero() || ![1, 2].contains(role) {
                return Err(RecordError::InvalidRecord);
            }
            (
                SystemNamespace::Reward,
                vec![
                    alloy_rlp::encode(*owner),
                    alloy_rlp::encode(*role),
                    alloy_rlp::encode(*liability),
                    alloy_rlp::encode(*index),
                ],
            )
        }
        SystemValue::Parameter { name, value } => {
            if name.is_empty() || name.len() > 64 || value.len() > 256 {
                return Err(RecordError::InvalidRecord);
            }
            (
                SystemNamespace::Parameter,
                vec![
                    alloy_rlp::encode(name.as_ref()),
                    alloy_rlp::encode(value.as_ref()),
                ],
            )
        }
        SystemValue::Task {
            epoch,
            task_id,
            node,
            content,
            request_nonce,
            deadline_height,
            work_units,
            consumed,
        } => {
            if task_id.is_zero()
                || node.is_zero()
                || content.is_zero()
                || *deadline_height == 0
                || *work_units == 0
            {
                return Err(RecordError::InvalidRecord);
            }
            (
                SystemNamespace::Task,
                vec![
                    alloy_rlp::encode(*epoch),
                    alloy_rlp::encode(*task_id),
                    alloy_rlp::encode(*node),
                    alloy_rlp::encode(*content),
                    alloy_rlp::encode(*request_nonce),
                    alloy_rlp::encode(*deadline_height),
                    alloy_rlp::encode(*work_units),
                    alloy_rlp::encode(u8::from(*consumed)),
                ],
            )
        }
        SystemValue::Evidence {
            evidence_id,
            offense_height,
            applied,
        } => {
            if evidence_id.is_zero() || *offense_height == 0 {
                return Err(RecordError::InvalidRecord);
            }
            (
                SystemNamespace::Evidence,
                vec![
                    alloy_rlp::encode(*evidence_id),
                    alloy_rlp::encode(*offense_height),
                    alloy_rlp::encode(u8::from(*applied)),
                ],
            )
        }
        SystemValue::Upgrade {
            old_version,
            new_version,
            activation_height,
            code_digest,
            migration,
        } => {
            if *old_version == 0
                || old_version >= new_version
                || *activation_height == 0
                || code_digest.is_zero()
                || migration.is_empty()
                || migration.len() > 128
            {
                return Err(RecordError::InvalidRecord);
            }
            (
                SystemNamespace::Upgrade,
                vec![
                    alloy_rlp::encode(*old_version),
                    alloy_rlp::encode(*new_version),
                    alloy_rlp::encode(*activation_height),
                    alloy_rlp::encode(*code_digest),
                    alloy_rlp::encode(migration.as_ref()),
                ],
            )
        }
    };
    if namespace != record.namespace {
        return Err(RecordError::InvalidRecord);
    }
    Ok(encode_list(&[
        alloy_rlp::encode(record.schema_version),
        alloy_rlp::encode(namespace_bytes(namespace)),
        alloy_rlp::encode(record.logical_key.as_ref()),
        encode_list(&fields),
    ]))
}
