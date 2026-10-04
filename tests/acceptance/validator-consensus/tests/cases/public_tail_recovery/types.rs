// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

#[derive(Clone, Copy)]
pub(super) struct DurablePrefix {
    pub height: u64,
    pub marker_sequence: u64,
    pub acknowledged_physical_sequence: u64,
}
