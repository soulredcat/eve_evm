// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support;

use eve_storage::state::{
    capture_state_snapshot, commit_state, open_state_repository, read_snapshot_commit, state_reader,
};
use std::{
    path::Path,
    process::{Child, Command, Stdio},
    time::{Duration, Instant},
};
use support::{
    execution::executed_commit,
    storage::{budget, database_path, local_directory},
};

struct ChildGuard(Child);

impl Drop for ChildGuard {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

#[test]
fn ts05_actual_process_termination_preserves_complete_old_or_new_commit() {
    if let Some(stage) = std::env::var_os("EVE_B1_ACCEPTANCE_CHILD_STAGE") {
        child(stage.to_str().unwrap());
        unreachable!("child waits until this test kills it");
    }
    let (genesis, next) = executed_commit();
    for (stage, delay_ms) in [
        ("before", 0),
        ("race", 0),
        ("race", 1),
        ("race", 5),
        ("after", 0),
    ] {
        let directory = local_directory("process-state-");
        let path = database_path(&directory);
        let ready = directory.path().join("ready.txt");
        let log = std::fs::File::create(directory.path().join("child.log")).unwrap();
        let mut child = ChildGuard(
            Command::new(std::env::current_exe().unwrap())
                .args([
                    "--exact",
                    "process_atomicity::ts05_actual_process_termination_preserves_complete_old_or_new_commit",
                    "--nocapture",
                ])
                .env("EVE_B1_ACCEPTANCE_CHILD_STAGE", stage)
                .env("EVE_B1_ACCEPTANCE_CHILD_DIRECTORY", directory.path())
                .stdout(Stdio::from(log.try_clone().unwrap()))
                .stderr(Stdio::from(log))
                .spawn()
                .unwrap(),
        );
        wait_for_ready(&ready, stage, &mut child.0);
        std::thread::sleep(Duration::from_millis(delay_ms));
        child.0.kill().unwrap();
        let status = child.0.wait().unwrap();
        assert!(
            !status.success(),
            "process must be terminated, not gracefully closed"
        );
        let repository = open_state_repository(&path, &genesis, budget()).unwrap();
        let reader = state_reader(&repository);
        let snapshot = capture_state_snapshot(&reader).unwrap();
        let version = snapshot.version();
        assert!(version.height == 0 || version.height == 1);
        if stage == "before" {
            assert_eq!(version.height, 0);
        }
        if stage == "after" {
            assert_eq!(version.height, 1);
        }
        let restored = read_snapshot_commit(&snapshot, version.height)
            .unwrap()
            .unwrap();
        assert_eq!(
            restored,
            if version.height == 0 {
                genesis.clone()
            } else {
                next.clone()
            }
        );
        assert_eq!(
            read_snapshot_commit(&snapshot, 0).unwrap(),
            Some(genesis.clone())
        );
    }
}

fn child(stage: &str) {
    let directory = std::env::var_os("EVE_B1_ACCEPTANCE_CHILD_DIRECTORY").unwrap();
    let directory = Path::new(&directory).canonicalize().unwrap();
    let repository = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(3)
        .unwrap()
        .canonicalize()
        .unwrap();
    assert!(directory.starts_with(repository.join("local-tests/b1-acceptance")));
    let (genesis, next) = executed_commit();
    let mut store = open_state_repository(&directory.join("database"), &genesis, budget()).unwrap();
    let ready = directory.join("ready.txt");
    if stage == "before" {
        std::fs::write(&ready, stage).unwrap();
    } else if stage == "race" {
        // Publication of this test handshake precedes the real sync call.
        // The parent races termination; this does not locate an internal fsync boundary.
        std::fs::write(&ready, stage).unwrap();
        commit_state(&mut store, &next).unwrap();
    } else if stage == "after" {
        let ack = commit_state(&mut store, &next).unwrap();
        assert_eq!(ack.store_head, next.target);
        std::fs::write(&ready, stage).unwrap();
    } else {
        panic!("unsupported test child stage");
    }
    loop {
        std::thread::sleep(Duration::from_millis(100));
    }
}

fn wait_for_ready(path: &Path, stage: &str, child: &mut Child) {
    let started = Instant::now();
    loop {
        if std::fs::read_to_string(path).is_ok_and(|contents| contents == stage) {
            return;
        }
        assert!(
            child.try_wait().unwrap().is_none(),
            "child exited before readiness; inspect its ignored log"
        );
        assert!(
            started.elapsed() < Duration::from_secs(20),
            "process readiness timed out"
        );
        std::thread::sleep(Duration::from_millis(1));
    }
}
