// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    context::{final_request, final_source, process_request},
    fixture::{reopen_application, test_application, transaction},
};
use crate::consensus::{
    application::{
        application_info, commit_application, finalize_block, types::SimulatedApplicationFailure,
    },
    transport::peer::{EngineChannel, tests::spawn_authenticated_test_peer},
};
use std::process::Command;

#[test]
fn process_exit_at_real_sync_recovers_complete_application_and_metadata() {
    if let Some(root) = std::env::var_os("EVE_APPLICATION_PROCESS_WORKER_ROOT") {
        let allowed = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("local-tests/b3-preparation/application-unit")
            .canonicalize()
            .unwrap();
        let path = std::path::Path::new(&root).canonicalize().unwrap();
        assert!(path.starts_with(allowed));
        let (genesis, spec) = super::genesis::application_test_genesis(false);
        let mut application = reopen_application(&path, genesis, &spec);
        let decision = final_request(&process_request(&application, 1, vec![transaction()]));
        let source = final_source(&application, &decision);
        finalize_block(&mut application, source, &decision).unwrap();
        application.simulated_failure = Some(SimulatedApplicationFailure::ExitAfterState);
        let peer = spawn_authenticated_test_peer(EngineChannel::Application);
        commit_application(&mut application, &peer.peer).unwrap();
        panic!("test worker did not exit at actual sync boundary");
    }
    let fixture = test_application(false);
    let root = fixture.root;
    let genesis = fixture.genesis;
    let spec = fixture.spec;
    drop(fixture.application);
    let status = Command::new(std::env::current_exe().unwrap()).args(["--exact", "consensus::application::tests::process_recovery::process_exit_at_real_sync_recovers_complete_application_and_metadata", "--nocapture"]).env("EVE_APPLICATION_PROCESS_WORKER_ROOT", root.path()).status().unwrap();
    assert_eq!(status.code(), Some(67));
    let mut recovered = reopen_application(root.path(), genesis, &spec);
    assert_eq!(application_info(&recovered).unwrap().last_block_height, 1);
    let decision = final_request(&process_request(&recovered, 1, vec![transaction()]));
    let source = final_source(&recovered, &decision);
    finalize_block(&mut recovered, source, &decision).unwrap();
    let peer = spawn_authenticated_test_peer(EngineChannel::Application);
    commit_application(&mut recovered, &peer.peer).unwrap();
    assert_eq!(application_info(&recovered).unwrap().last_block_height, 1);
}
