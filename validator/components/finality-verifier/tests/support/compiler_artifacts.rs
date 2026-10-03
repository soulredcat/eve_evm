// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::{
    path::{Path, PathBuf},
    process::Command,
};

/// Select a coherent dependency pair, not independent newest crate identities.
pub fn compatible_artifacts(
    deps: &Path,
    compiler: &std::ffi::OsStr,
    directory: &Path,
) -> (PathBuf, PathBuf) {
    let mut verifier = Vec::new();
    let mut state = Vec::new();
    for entry in std::fs::read_dir(deps).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().is_none_or(|extension| extension != "rlib") {
            continue;
        }
        let name = path.file_name().unwrap().to_string_lossy();
        if name.starts_with("libeve_finality_verifier-") {
            verifier.push(path);
        } else if name.starts_with("libeve_state-") {
            state.push(path);
        }
    }
    verifier.sort_by_key(|path| std::cmp::Reverse(path.metadata().unwrap().modified().unwrap()));
    state.sort_by_key(|path| std::cmp::Reverse(path.metadata().unwrap().modified().unwrap()));
    let verifier = verifier
        .into_iter()
        .next()
        .expect("current verifier rlib is required");
    let source = directory.join("dependency_probe.rs");
    std::fs::write(
        &source,
        r#"
        use eve_finality_verifier::preflight_logical_import_wire;
        use eve_state::development_state_budget;
        fn main() { let _ = preflight_logical_import_wire(&[], &development_state_budget()); }
    "#,
    )
    .unwrap();
    for candidate in state {
        let output = Command::new(compiler)
            .args([
                "--edition=2024",
                "--crate-name=finality_dependency_probe",
                "--emit=metadata",
            ])
            .arg(&source)
            .arg("-o")
            .arg(directory.join("dependency_probe.rmeta"))
            .arg("-L")
            .arg(format!("dependency={}", deps.display()))
            .arg("--extern")
            .arg(format!("eve_finality_verifier={}", verifier.display()))
            .arg("--extern")
            .arg(format!("eve_state={}", candidate.display()))
            .output()
            .expect("pinned compiler must resolve test dependency identities");
        if output.status.success() {
            return (verifier, candidate);
        }
    }
    panic!(
        "no eve-state rlib matches the current verifier; dependency failure is not privacy evidence"
    );
}
