use crate::provisioning::{
    artifacts::{compute_artifact_digest, extract_pinned_archive, fetch_pinned_artifact},
    execution::run_checked_command,
    paths::resolve_contained_path,
    receipts::{load_verified_receipt, write_tool_receipt},
    types::{ArtifactPin, ProvisionedArtifact},
};
use anyhow::{Result, ensure};
use std::{path::Path, process::Command};

pub fn provision_comet(
    output: &Path,
    pin: &ArtifactPin,
    go: &ProvisionedArtifact,
    jobs: usize,
) -> Result<ProvisionedArtifact> {
    let recipe = format!(
        "go1.27.1-readonly-trimpath-no-host-vcs-CGO1-clang19-v2;{}",
        pin.source_identity
    );
    if let Some(receipt) = load_verified_receipt(output, "comet", pin, &recipe, &["version"])? {
        return Ok(receipt);
    }
    let archive = fetch_pinned_artifact(output, &format!("comet-{}.artifact", pin.sha256), pin)?;
    let source = extract_pinned_archive(output, &archive, "comet", pin)?;
    let go_mod = compute_artifact_digest(&source.join("go.mod"))?;
    let go_sum = compute_artifact_digest(&source.join("go.sum"))?;
    let cache = resolve_contained_path(output, Path::new("go-cache"))?;
    let modules = resolve_contained_path(output, Path::new("go-module-cache"))?;
    std::fs::create_dir_all(&cache)?;
    std::fs::create_dir_all(&modules)?;
    let target = resolve_contained_path(&source, Path::new(&pin.executable))?;
    let linker = format!(
        "-ldflags=-X=github.com/cometbft/cometbft/version.TMGitCommitHash={}",
        pin.source_identity
    );
    let mut build = Command::new(&go.executable);
    super::configure_go_source_build(&mut build, jobs);
    run_checked_command(
        build
            .current_dir(&source)
            .args([&linker, "-o"])
            .arg(target)
            .arg("./cmd/cometbft")
            .env("GOTOOLCHAIN", "local")
            .env("GOCACHE", &cache)
            .env("GOMODCACHE", &modules)
            .env("GOPROXY", "https://proxy.golang.org")
            .env("GOSUMDB", "sum.golang.org")
            .env("CGO_ENABLED", "1")
            .env("CC", "clang-19"),
        output,
        "comet-build",
    )?;
    ensure!(
        compute_artifact_digest(&source.join("go.mod"))? == go_mod
            && compute_artifact_digest(&source.join("go.sum"))? == go_sum,
        "engine build changed locked module inputs"
    );
    run_checked_command(
        Command::new(&go.executable)
            .current_dir(&source)
            .args(["mod", "verify"])
            .env("GOTOOLCHAIN", "local")
            .env("GOMODCACHE", &modules),
        output,
        "comet-module-verify",
    )?;
    write_tool_receipt(output, "comet", pin, &recipe, &["version"])
}
