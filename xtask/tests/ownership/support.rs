// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::{fs, path::Path, process::Command};
use tempfile::{TempDir, tempdir};
use xtask::publication::ownership::{check_ownership::check_ownership, types::OwnershipReport};

pub const REDCAT: &str = "LicenseRef-Redcat-Permission-Only";
pub const TERMS: &str = "Copyright (c) 2026 Redcat. All rights reserved. LicenseRef-Redcat-Permission-Only. Use requires prior written permission from Redcat, subject to applicable law and mandatory platform terms.\n";

pub struct Fixture {
    directory: TempDir,
}

impl Fixture {
    pub fn new() -> Self {
        let directory = tempdir().unwrap();
        let status = Command::new("git")
            .args(["init", "--quiet"])
            .current_dir(directory.path())
            .status()
            .unwrap();
        assert!(status.success());
        let fixture = Self { directory };
        fixture.write("LICENSE", TERMS);
        fixture.write("LICENSES/LicenseRef-Redcat-Permission-Only.txt", TERMS);
        fixture.write(
            "config/structure-policy.toml",
            &notice("#", "version = 1\ncurrent_bulk = 0\n"),
        );
        fixture.reuse("");
        fixture.write(".gitignore", &notice("#", "/local-tests/\n"));
        fixture
    }

    pub fn root(&self) -> &Path {
        self.directory.path()
    }

    pub fn write(&self, path: &str, value: &str) {
        let target = self.root().join(path);
        fs::create_dir_all(target.parent().unwrap()).unwrap();
        fs::write(target, value).unwrap();
    }

    pub fn reuse(&self, entries: &str) {
        self.write(
            "REUSE.toml",
            &notice("#", &format!("version = 1\n{entries}")),
        );
    }

    pub fn annotation(&self, path: &str, license: &str, holder: &str) {
        self.reuse(&annotation(path, license, holder));
    }

    pub fn check(&self) -> OwnershipReport {
        check_ownership(self.root()).expect("parse and inspect ownership fixture")
    }

    pub fn pass(&self) -> OwnershipReport {
        let report = self.check();
        assert!(report.violations.is_empty(), "{:#?}", report.violations);
        report
    }

    pub fn reject(&self, category: &str) {
        let report = self.check();
        assert!(
            report
                .violations
                .iter()
                .any(|message| message.contains(category)),
            "{:#?}",
            report.violations
        );
    }

    pub fn pin(&self, path: &str, bytes: &str) {
        use sha2::{Digest, Sha256};
        let digest: String = Sha256::digest(bytes.as_bytes())
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect();
        let original =
            fs::read_to_string(self.root().join("config/structure-policy.toml")).unwrap();
        self.write("config/structure-policy.toml", &format!("{original}\n[[exclusions]]\npath = {path:?}\nkind = \"vendor\"\nsource = \"https://example.invalid/pinned\"\nreason = \"Preserved test input\"\nsha256 = {digest:?}\n"));
    }
}

pub fn notice(prefix: &str, body: &str) -> String {
    let lines = [
        "SPDX-FileCopyrightText: 2026 Redcat",
        "SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only",
        "Use requires prior written permission from Redcat.",
    ];
    let header = lines
        .map(|line| {
            if prefix == "html" {
                format!("<!-- {line} -->")
            } else {
                format!("{prefix} {line}")
            }
        })
        .join("\n");
    format!("{header}\n\n{body}")
}

pub fn annotation(path: &str, license: &str, holder: &str) -> String {
    let comment = if license == REDCAT {
        "Use requires prior written permission from Redcat."
    } else {
        "Preserved upstream source and terms."
    };
    format!(
        "\n[[annotations]]\npath = {path:?}\nprecedence = \"closest\"\nSPDX-FileCopyrightText = {holder:?}\nSPDX-License-Identifier = {license:?}\nSPDX-FileComment = {comment:?}\n"
    )
}
