// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::StateView;
use alloy_primitives::B256;
use eve_protocol_config::records::SystemRecord;

pub fn read_system(view: &StateView, key: B256) -> Option<&SystemRecord> {
    view.state.system.get(&key)
}
