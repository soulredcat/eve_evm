// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use serde_json::Value;
pub(crate) fn quantity(value: impl std::fmt::LowerHex) -> Value {
    Value::String(format!("0x{value:x}"))
}
