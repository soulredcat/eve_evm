use crate::verification::{
    artifacts::{create_artifact_directory::create_artifact_directory, save_report::save_report},
    types::report_types::VerificationReport,
};
use anyhow::{Result, bail};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn run_verify(root: &Path, requested: Vec<String>, all: bool) -> Result<()> {
    let root = root.canonicalize()?;
    let artifacts = create_artifact_directory(&root)?;
    println!("Local-only verification evidence: {}", artifacts.display());
    let mut report = VerificationReport {
        schema_version: 1,
        started_unix_ms: SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis(),
        host_platform: format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH),
        topology: "Single-host component fixtures; one Comet engine API fixture, no complete role devnet".into(),
        genesis_identity: "No authorized runtime/mainnet genesis; development byte vectors and synthetic interop genesis reservations only".into(),
        profile_activation: "Classical development only; hybrid consensus activation unsupported and rejected".into(),
        key_epochs: "No runtime key ceremony/activation; explicitly disposable primitive/API-fixture identities".into(),
        requested,
        status: "FAIL".into(),
        profile: "FOUNDATION_ONLY_NOT_SECURITY_PROFILE_ACCEPTED".into(),
        artifact_directory: artifacts.display().to_string(),
        ..Default::default()
    };
    let outcome =
        super::run_selected_gates::run_selected_gates(&root, &artifacts, &mut report, all);
    match outcome {
        Ok(()) => report.status = "PASS".into(),
        Err(error) => report.errors.push(format!("{error:#}")),
    }
    report.completed_unix_ms = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
    save_report(&artifacts, &report)?;
    println!(
        "{}: {} executed tests; report {}",
        report.status,
        report.test_count,
        artifacts.join("report.json").display()
    );
    if report.status != "PASS" {
        bail!("Verification failed: {}", report.errors.join("; "));
    }
    Ok(())
}
