// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::StringValues;

pub(super) fn value_items(values: &StringValues) -> &[String] {
    match values {
        StringValues::One(value) => std::slice::from_ref(value),
        StringValues::Many(values) => values,
    }
}
