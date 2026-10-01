// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::MempoolLimits;
use std::time::Duration;
impl Default for MempoolLimits {
    fn default() -> Self {
        Self {
            maximum_transactions: 10_000,
            maximum_bytes: 64 * 1_048_576,
            maximum_per_sender: 64,
            maximum_nonce_gap: 64,
            ttl: Duration::from_secs(300),
        }
    }
}
