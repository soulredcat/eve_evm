// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::{
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

struct CompileDirectory(PathBuf);

impl Drop for CompileDirectory {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(self.0.join("source.rs"));
        let _ = std::fs::remove_file(self.0.join("output.rmeta"));
        let _ = std::fs::remove_dir(&self.0);
    }
}

fn artifact(deps: &Path, library: &str) -> PathBuf {
    let prefix = format!("lib{library}-");
    std::fs::read_dir(deps)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| {
            path.file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with(&prefix)
                && path
                    .extension()
                    .is_some_and(|extension| extension == "rlib")
        })
        .max_by_key(|path| path.metadata().unwrap().modified().unwrap())
        .expect("current test build must produce the required dependency rlib")
}

/// Compile a downstream consumer; only the expected compiler error can pass.
pub fn must_reject(source: &str, expected_error: &str) {
    let executable = std::env::current_exe().unwrap();
    let deps = executable.parent().unwrap();
    let directory = std::env::temp_dir().join(format!(
        "eve-finality-compile-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir(&directory).unwrap();
    let directory = CompileDirectory(directory);
    std::fs::write(directory.0.join("source.rs"), source).unwrap();
    let compiler = std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into());
    let mut command = Command::new(compiler);
    command
        .args([
            "--edition=2024",
            "--crate-name=finality_boundary_consumer",
            "--emit=metadata",
        ])
        .arg(directory.0.join("source.rs"))
        .arg("-o")
        .arg(directory.0.join("output.rmeta"))
        .arg("-L")
        .arg(format!("dependency={}", deps.display()))
        .arg("--extern")
        .arg(format!(
            "eve_finality_verifier={}",
            artifact(deps, "eve_finality_verifier").display()
        ));
    // Only fixtures naming state require a direct dependency. Otherwise the
    // verifier's metadata supplies its exact state crate, even with mixed builds.
    if source.contains("eve_state::") {
        command.arg("--extern").arg(format!(
            "eve_state={}",
            artifact(deps, "eve_state").display()
        ));
    }
    let output = command
        .output()
        .expect("pinned Rust compiler must be available for boundary tests");
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        !output.status.success(),
        "downstream boundary unexpectedly compiled"
    );
    assert!(
        stderr.contains(&format!("error[{expected_error}]")),
        "unexpected compiler result: {stderr}"
    );
    for line in stderr.lines().filter(|line| line.starts_with("error[")) {
        assert!(
            line.starts_with(&format!("error[{expected_error}]")),
            "an unrelated compiler error cannot satisfy the boundary test: {stderr}"
        );
    }
    for unrelated in ["E0432", "E0433", "E0463", "E0464", "E0786"] {
        assert!(
            !stderr.contains(&format!("error[{unrelated}]")),
            "dependency failure is not privacy evidence: {stderr}"
        );
    }
}
