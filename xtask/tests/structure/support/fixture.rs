// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::{fs, path::Path, process::Command};
use tempfile::{TempDir, tempdir};
use xtask::structure::{
    checking::check_structure::check_structure,
    types::{policy_types::StructurePolicy, report_types::StructureReport},
};

pub struct Fixture {
    directory: TempDir,
}

impl Fixture {
    pub fn new() -> Self {
        let directory = tempdir().expect("temporary repository");
        let initialized = Command::new("git")
            .args(["init", "--quiet"])
            .current_dir(directory.path())
            .output()
            .expect("Git fixture initialization");
        assert!(initialized.status.success());
        let fixture = Self { directory };
        fixture.write(
            "config/structure-policy.toml",
            "version = 1\ncurrent_bulk = 0\n",
        );
        fixture.write(".gitignore", "/target/\n");
        fixture
    }

    pub fn root(&self) -> &Path {
        self.directory.path()
    }

    pub fn write(&self, relative: &str, contents: &str) {
        let path = self.root().join(relative);
        fs::create_dir_all(path.parent().expect("fixture file parent")).unwrap();
        fs::write(path, contents).unwrap();
    }

    pub fn policy(&self, registrations: &str) {
        self.write(
            "config/structure-policy.toml",
            &format!("version = 1\ncurrent_bulk = 0\n{registrations}"),
        );
    }

    pub fn check(&self) -> StructureReport {
        check_structure(self.root(), Path::new("config/structure-policy.toml"))
            .expect("structure fixture check")
    }

    pub fn assert_pass(&self) -> StructureReport {
        let report = self.check();
        assert!(report.violations.is_empty(), "{:#?}", report.violations);
        report
    }

    pub fn assert_rejected(&self) -> StructureReport {
        let report = self.check();
        assert!(
            !report.violations.is_empty(),
            "invalid fixture was accepted"
        );
        report
    }

    pub fn cargo(&self, directory: &str, arguments: &[&str]) -> std::process::Output {
        Command::new(env!("CARGO"))
            .args(arguments)
            .env("CARGO_TARGET_DIR", self.root().join("target"))
            .current_dir(self.root().join(directory))
            .output()
            .expect("isolated fixture Cargo command")
    }
}

pub fn plain_policy() -> StructurePolicy {
    toml::from_str("version = 1\ncurrent_bulk = 0\n").unwrap()
}

pub fn source_with_lines(lines: usize, terminated: bool) -> String {
    assert!(lines >= 4);
    let mut source =
        String::from("pub fn encode_height() -> [u8; 8] {\n    1_u64.to_be_bytes()\n}\n");
    for index in 3..lines {
        source.push_str(if index % 2 == 0 {
            "\n"
        } else {
            "// Count comments too.\n"
        });
    }
    if !terminated {
        source.pop();
        if source.ends_with('\n') {
            source.push_str("// Final physical line.");
        }
    }
    source
}
