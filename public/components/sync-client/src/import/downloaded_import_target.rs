// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::DownloadedAuthenticatedImport;
use eve_state::StateVersion;
pub fn downloaded_import_target<L>(download: &DownloadedAuthenticatedImport<L>) -> &StateVersion {
    &download.target
}
