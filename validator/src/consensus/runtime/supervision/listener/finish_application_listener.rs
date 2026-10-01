// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::types::ApplicationListener;

/// Destructor fallback preserves foreign endpoints; foreground completion checks cleanup explicitly.
pub(in crate::consensus::runtime) fn finish_application_listener(listener: &ApplicationListener) {
    let _ = super::super::cleanup_application_listener::cleanup_application_listener(listener);
}
