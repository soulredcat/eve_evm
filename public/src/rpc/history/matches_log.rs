// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::LogFilter;
use alloy_primitives::{Address, B256};
use serde_json::Value;
pub(crate) fn matches_log(filter: &LogFilter, log: &Value) -> bool {
    if filter.addresses.as_ref().is_some_and(|addresses| {
        !serde_json::from_value::<Address>(log["address"].clone())
            .is_ok_and(|address| addresses.contains(&address))
    }) {
        return false;
    }
    let Some(topics) = log["topics"].as_array() else {
        return false;
    };
    filter.topics.iter().enumerate().all(|(index, expected)| {
        expected.as_ref().is_none_or(|expected| {
            topics.get(index).is_some_and(|topic| {
                serde_json::from_value::<B256>(topic.clone())
                    .is_ok_and(|topic| expected.contains(&topic))
            })
        })
    })
}
