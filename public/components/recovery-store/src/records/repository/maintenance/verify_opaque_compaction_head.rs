// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::opening::validate_opaque_namespace;
use crate::records::{
    OpaqueRecordCursor, OpaqueRecordRepository, opaque_record_bootstrap_cursor,
    opaque_record_cursor,
};
use anyhow::{Result, ensure};

pub(super) fn verify_opaque_compaction_head(
    repository: &OpaqueRecordRepository,
    expected: OpaqueRecordCursor,
) -> Result<()> {
    ensure!(
        opaque_record_cursor(repository)? == expected,
        "opaque maintenance RAM head changed"
    );
    let actual = validate_opaque_namespace(
        &repository.database,
        repository.identity,
        opaque_record_bootstrap_cursor(repository),
        &repository.budget,
    )?;
    ensure!(
        actual == expected,
        "opaque maintenance retained namespace/head changed"
    );
    Ok(())
}
