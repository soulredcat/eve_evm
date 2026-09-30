use crate::verification::types::report_types::CommandEvidence;
use crate::verification::types::report_types::VerificationReport;
use anyhow::{Context, Result, bail};
use sha2::{Digest, Sha256};
use std::{path::Path, process::Command, time::Instant};

pub fn run_recorded_command(
    root: &Path,
    artifacts: &Path,
    report: &mut VerificationReport,
    program: &str,
    arguments: &[&str],
) -> Result<(Option<i32>, String)> {
    println!("Running {program} {}", arguments.join(" "));
    let started = Instant::now();
    let output = Command::new(program)
        .envs(&report.tool_environment)
        .args(arguments)
        .current_dir(root)
        .output();
    let (code, stdout_bytes, stderr_bytes, spawn_error) = match output {
        Ok(output) => (output.status.code(), output.stdout, output.stderr, None),
        Err(error) => (
            None,
            Vec::new(),
            error.to_string().into_bytes(),
            Some(error.to_string()),
        ),
    };
    let index = report.commands.len();
    let stdout = format!("command-{index:03}.stdout.txt");
    let stderr = format!("command-{index:03}.stderr.txt");
    std::fs::write(artifacts.join(&stdout), &stdout_bytes)?;
    std::fs::write(artifacts.join(&stderr), &stderr_bytes)?;
    report.commands.push(CommandEvidence {
        program: program.into(),
        arguments: arguments.iter().map(|arg| (*arg).into()).collect(),
        exit_code: code,
        spawn_error: spawn_error.clone(),
        elapsed_ms: started.elapsed().as_millis(),
        stdout,
        stderr,
        stdout_sha256: Sha256::digest(&stdout_bytes)
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect(),
        stderr_sha256: Sha256::digest(&stderr_bytes)
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect(),
    });
    if let Some(error) = spawn_error {
        bail!("Start {program} {} failed: {error}", arguments.join(" "));
    }
    Ok((
        code,
        String::from_utf8(stdout_bytes).context("Command output must be UTF-8")?,
    ))
}
