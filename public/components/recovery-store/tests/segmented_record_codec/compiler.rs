// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::{path::Path, process::Command};

fn current_library(dependencies: &Path) -> std::path::PathBuf {
    std::fs::read_dir(dependencies)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| {
            path.file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with("libeve_storage-")
                && path
                    .extension()
                    .is_some_and(|extension| extension == "rlib")
        })
        .max_by_key(|path| path.metadata().unwrap().modified().unwrap())
        .expect("current build must provide its storage dependency")
}

/// Mandatory downstream compile-boundary test; dependency failure cannot pass it.
pub fn must_reject(source: &str, expected: &str) {
    let executable = std::env::current_exe().unwrap();
    let dependencies = executable.parent().unwrap();
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("source.rs");
    std::fs::write(&path, source).unwrap();
    let compiler = std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into());
    let output = Command::new(compiler)
        .args([
            "--edition=2024",
            "--crate-name=segment_boundary",
            "--emit=metadata",
        ])
        .arg(&path)
        .arg("-o")
        .arg(directory.path().join("output.rmeta"))
        .arg("-L")
        .arg(format!("dependency={}", dependencies.display()))
        .arg("--extern")
        .arg(format!(
            "eve_storage={}",
            current_library(dependencies).display()
        ))
        .output()
        .expect("pinned compiler must be available");
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        !output.status.success(),
        "downstream boundary unexpectedly compiled"
    );
    assert!(
        stderr.contains(&format!("error[{expected}]")),
        "wrong compiler result: {stderr}"
    );
    for line in stderr.lines().filter(|line| line.starts_with("error[")) {
        assert!(
            line.starts_with(&format!("error[{expected}]")),
            "unrelated compiler failure: {stderr}"
        );
    }
    for unrelated in ["E0432", "E0433", "E0463", "E0464", "E0786"] {
        assert!(
            !stderr.contains(&format!("error[{unrelated}]")),
            "dependency failure is not privacy evidence"
        );
    }
}
