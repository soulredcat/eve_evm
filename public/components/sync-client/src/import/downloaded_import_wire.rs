// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::DownloadedAuthenticatedImport;
pub fn downloaded_import_wire<L>(download: &DownloadedAuthenticatedImport<L>) -> &[u8] {
    &download.wire
}
