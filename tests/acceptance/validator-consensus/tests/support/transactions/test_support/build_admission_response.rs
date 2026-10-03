// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use serde_json::{Value, json};
use sha2::{Digest, Sha256};

#[cfg(test)]
pub(in super::super) fn build_admission_response(bytes: &[u8]) -> Value {
    json!({"code": 0, "hash": hex::encode_upper(Sha256::digest(bytes))})
}
