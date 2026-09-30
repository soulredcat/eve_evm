use crate::verification::{
    commands::require_command_success::require_command_success,
    types::report_types::VerificationReport,
};
use anyhow::{Result, ensure};
use std::path::Path;

pub(super) fn record_native_environment(
    root: &Path,
    artifacts: &Path,
    report: &mut VerificationReport,
) -> Result<()> {
    for (program, argument, required) in [
        ("clang-19", "--version", "19.1.7"),
        ("clang++-19", "--version", "19.1.7"),
        ("gcc", "--version", "14.2.0"),
        ("g++", "--version", "14.2.0"),
        ("make", "--version", "4.4.1"),
        ("perl", "-v", "v5.40.1"),
        ("git", "--version", "git version"),
        ("curl", "--version", "curl"),
    ] {
        let version = require_command_success(root, artifacts, report, program, &[argument])?;
        ensure!(
            version.contains(required),
            "Native tool {program} differs from B0 build pin"
        );
    }
    require_command_success(
        root,
        artifacts,
        report,
        "dpkg-query",
        &[
            "-W",
            "gcc",
            "g++",
            "make",
            "perl",
            "clang-19",
            "libclang-19-dev",
            "time",
            "git",
            "curl",
        ],
    )?;
    Ok(())
}
