// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

pub(in crate::development::engine) struct EngineLease {
    pub(super) file: std::fs::File,
    pub(super) locked: bool,
}
