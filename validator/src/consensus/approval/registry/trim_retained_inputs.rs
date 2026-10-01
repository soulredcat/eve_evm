// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    RetainedInput,
    types::{MAXIMUM_RAW_BYTES, MAXIMUM_RAW_INPUTS},
};
use crate::consensus::approval::ApprovalError;
use std::{collections::BTreeMap, sync::Arc};

pub(super) fn trim_retained_inputs(
    raw: &mut BTreeMap<[u8; 32], Arc<RetainedInput>>,
    protected: &[Option<[u8; 32]>],
) -> Result<(), ApprovalError> {
    loop {
        let bytes = raw
            .values()
            .try_fold(0_usize, |sum, value| sum.checked_add(value.bytes))
            .ok_or(ApprovalError::Unavailable {
                reason: "retained input bytes overflow",
                cause: None,
            })?;
        if raw.len() <= MAXIMUM_RAW_INPUTS && bytes <= MAXIMUM_RAW_BYTES {
            return Ok(());
        }
        let removable = raw
            .iter()
            .filter(|(hash, _)| !protected.contains(&Some(**hash)))
            .min_by_key(|(_, value)| value.insertion)
            .map(|(hash, _)| *hash);
        let Some(hash) = removable else {
            return Err(ApprovalError::Unavailable {
                reason: "protected retained input capacity unavailable",
                cause: None,
            });
        };
        raw.remove(&hash);
    }
}
