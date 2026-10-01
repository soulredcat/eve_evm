// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use alloy_primitives::Address;
use eve_state::{SystemRecord, SystemValue};

pub(super) fn matches_genesis_validator_record(
    record: &SystemRecord,
    owner: &Address,
    key: &[u8; 32],
    power: i64,
) -> bool {
    match &record.value {
        SystemValue::Validator {
            owner: recorded_owner,
            key: recorded_key,
            power: recorded_power,
            activation,
            removal,
            key_epoch,
        } => {
            recorded_owner == owner
                && recorded_key == key
                && i64::try_from(*recorded_power).ok() == Some(power)
                && *activation == 1
                && removal.is_none()
                && *key_epoch == 0
        }
        _ => false,
    }
}
