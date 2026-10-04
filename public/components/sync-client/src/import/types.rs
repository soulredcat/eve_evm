// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_state::StateVersion;

/// Untrusted assembled wire. Actual caller-provided leases survive its owned bytes.
pub struct DownloadedAuthenticatedImport<L> {
    pub(super) wire: Vec<u8>,
    pub(super) target: StateVersion,
    pub(super) _leases: [L; 2],
}
