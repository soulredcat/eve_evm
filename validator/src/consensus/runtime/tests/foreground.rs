// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::runtime::run_development_validator;
use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
    time::{Duration, Instant},
};

#[test]
fn actual_foreground_native_handshake_and_sigterm_stop_owned_processes() {
    const CASE: &str = "consensus::runtime::tests::foreground::actual_foreground_native_handshake_and_sigterm_stop_owned_processes";
    if let Some(root) = std::env::var_os("EVE_RUNTIME_WORKER_ROOT") {
        let config = super::support::config(std::path::Path::new(&root));
        run_development_validator(config).unwrap();
        return;
    }
    let root = tempfile::tempdir().unwrap();
    let error_log = std::fs::File::create(root.path().join("private-worker.log")).unwrap();
    let mut child = Command::new(std::env::current_exe().unwrap())
        .args(["--exact", CASE, "--nocapture"])
        .env("EVE_RUNTIME_WORKER_ROOT", root.path())
        .stdout(Stdio::piped())
        .stderr(Stdio::from(error_log))
        .spawn()
        .unwrap();
    let output = child.stdout.take().unwrap();
    let (sender, receiver) = std::sync::mpsc::channel();
    let reader = std::thread::spawn(move || {
        let mut output = BufReader::new(output);
        let mut line = String::new();
        while output.read_line(&mut line).is_ok_and(|count| count > 0) {
            if line.starts_with('{') {
                let _ = sender.send(line.clone());
            }
            line.clear();
        }
    });
    let readiness = receiver.recv_timeout(Duration::from_secs(45));
    let pid = rustix::process::Pid::from_raw(i32::try_from(child.id()).unwrap()).unwrap();
    let process = rustix::process::pidfd_open(pid, rustix::process::PidfdFlags::empty()).unwrap();
    rustix::process::pidfd_send_signal(&process, rustix::process::Signal::TERM).unwrap();
    let deadline = Instant::now() + Duration::from_secs(10);
    while child.try_wait().unwrap().is_none() && Instant::now() < deadline {
        std::thread::sleep(Duration::from_millis(10));
    }
    if child.try_wait().unwrap().is_none() {
        child.kill().unwrap();
    }
    let status = child.wait().unwrap();
    reader.join().unwrap();
    let diagnostics = std::fs::read_to_string(root.path().join("private-worker.log")).unwrap();
    assert!(readiness.is_ok(), "actual readiness missing: {diagnostics}");
    assert!(
        status.success(),
        "foreground shutdown failed: {diagnostics}"
    );
    let readiness: serde_json::Value = serde_json::from_str(&readiness.unwrap()).unwrap();
    assert_eq!(readiness["event"], "development_validator_ready");
    assert_eq!(
        readiness["initialization"]["security_profile"],
        "CLASSICAL_DEV"
    );
    assert_eq!(readiness["application_height"], 0);
    assert_eq!(readiness["consensus_finality"], false);
    assert_eq!(readiness["application_hash"].as_str().unwrap().len(), 64);
    let data = root.path().join("node");
    assert!(std::fs::read_dir(data).unwrap().all(|entry| {
        !entry
            .unwrap()
            .file_name()
            .to_string_lossy()
            .ends_with(".sock")
    }));
}
