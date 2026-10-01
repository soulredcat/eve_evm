// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{engine_file_identity, types::EngineImageIdentity};

pub(crate) fn engine_process_image_identity(pid: u32) -> std::io::Result<EngineImageIdentity> {
    engine_file_identity(&std::fs::File::open(format!("/proc/{pid}/exe"))?)
}
