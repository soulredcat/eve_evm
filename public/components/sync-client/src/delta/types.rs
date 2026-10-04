// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_state::StateVersion;

/// Checked download integrity only; advertised versions are untrusted source metadata.
#[derive(Debug)]
pub struct DownloadedStateDelta {
    pub parent: StateVersion,
    pub target: StateVersion,
    pub durable_tip: StateVersion,
    pub body_sha256: [u8; 32],
    pub bytes: Vec<u8>,
}
