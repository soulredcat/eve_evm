// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{TestFixture, create_fixture};

pub(in crate::consensus) fn temporary_fixture() -> (tempfile::TempDir, TestFixture) {
    let repository = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap();
    let root = repository.join("local-tests/b3-preparation/signing-unit");
    std::fs::create_dir_all(&root).unwrap();
    let directory = tempfile::Builder::new()
        .prefix("case-")
        .tempdir_in(root)
        .unwrap();
    let fixture = create_fixture(directory.path());
    (directory, fixture)
}
