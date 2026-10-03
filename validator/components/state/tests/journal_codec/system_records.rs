// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_state::{Address, B256, Bytes, SystemNamespace, SystemRecord, SystemValue, U256};

pub fn system_records() -> Vec<SystemRecord> {
    let owner = Address::repeat_byte(1);
    let hash = B256::repeat_byte(2);
    let values = [
        (
            SystemNamespace::Validator,
            SystemValue::Validator {
                owner,
                key: [3; 32],
                power: 1,
                activation: 2,
                removal: Some(3),
                key_epoch: 4,
            },
        ),
        (
            SystemNamespace::Fee,
            SystemValue::Fee {
                burned: U256::MAX,
                node_pool: U256::ZERO,
                validator_pool: U256::from(1),
            },
        ),
        (
            SystemNamespace::Reward,
            SystemValue::Reward {
                owner,
                role: 1,
                liability: U256::from(1),
                index: U256::MAX,
            },
        ),
        (
            SystemNamespace::Task,
            SystemValue::Task {
                epoch: 0,
                task_id: hash,
                node: owner,
                content: hash,
                request_nonce: u64::MAX,
                deadline_height: 1,
                work_units: 1,
                consumed: true,
            },
        ),
        (
            SystemNamespace::Evidence,
            SystemValue::Evidence {
                evidence_id: hash,
                offense_height: 1,
                applied: true,
            },
        ),
        (
            SystemNamespace::Upgrade,
            SystemValue::Upgrade {
                old_version: 1,
                new_version: 2,
                activation_height: 3,
                code_digest: hash,
                migration: Bytes::from_static(b"migration"),
            },
        ),
    ];
    let mut records = values
        .into_iter()
        .map(|(namespace, value)| SystemRecord {
            schema_version: 1,
            namespace,
            logical_key: Bytes::from_static(b"key"),
            value,
        })
        .collect::<Vec<_>>();
    records.push(super::fixtures::parameter_record());
    records
}
