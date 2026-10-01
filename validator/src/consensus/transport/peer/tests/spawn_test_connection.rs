// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    super::{EngineChannel, authenticate_engine_peer},
    AuthenticatedTestPeer,
    types::TestEngineConnection,
};
use sha2::{Digest, Sha256};
use std::{
    io::{ErrorKind, Read, Write},
    os::unix::{fs::PermissionsExt, net::UnixListener},
    process::{Command, Stdio},
    time::{Duration, Instant},
};

pub(super) fn spawn_test_connection(mode: &str) -> TestEngineConnection {
    let directory = tempfile::tempdir().unwrap();
    std::fs::set_permissions(directory.path(), std::fs::Permissions::from_mode(0o700)).unwrap();
    let path = directory.path().join("engine.sock");
    // A concurrent build may replace the shared target executable. Launch this test's immutable copy.
    let executable = directory.path().join("owned-test-worker");
    std::fs::copy("/proc/self/exe", &executable).unwrap();
    std::fs::set_permissions(&executable, std::fs::Permissions::from_mode(0o700)).unwrap();
    let mut file = std::fs::File::open(&executable).unwrap();
    let mut digest = Sha256::new();
    let mut buffer = [0; 16_384];
    loop {
        let count = file.read(&mut buffer).unwrap();
        if count == 0 {
            break;
        }
        digest.update(&buffer[..count]);
    }
    let listener = UnixListener::bind(&path).unwrap();
    listener.set_nonblocking(true).unwrap();
    let expected_sha256 = digest.finalize().into();
    let image =
        crate::development::engine::verify_test_engine_image(&executable, expected_sha256).unwrap();
    let child = Command::new(&executable)
        .args([
            "--exact",
            "consensus::transport::peer::tests::native_test_peer_worker::native_test_peer_worker",
        ])
        .env("EVE_NATIVE_TEST_PEER_PATH", &path)
        .env("EVE_NATIVE_TEST_PEER_MODE", mode)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let mut fixture = TestEngineConnection {
        stream: None,
        child: Some(child),
        directory: Some(directory),
        expected_sha256,
        image,
        executable,
    };
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        match listener.accept() {
            Ok((stream, _)) => {
                fixture.stream = Some(stream);
                break;
            }
            Err(error) if error.kind() == ErrorKind::WouldBlock => {
                assert!(
                    fixture
                        .child
                        .as_mut()
                        .unwrap()
                        .try_wait()
                        .unwrap()
                        .is_none(),
                    "test peer exited before connect"
                );
                assert!(Instant::now() < deadline, "test peer connect deadline");
                std::thread::sleep(Duration::from_millis(5));
            }
            Err(error) => panic!("test Unix accept failed: {error}"),
        }
    }
    fixture
}

pub(in crate::consensus) fn spawn_authenticated_test_peer(
    channel: EngineChannel,
) -> AuthenticatedTestPeer {
    let mut fixture = spawn_test_connection("hold");
    let mut control = fixture.stream.as_ref().unwrap().try_clone().unwrap();
    let peer = authenticate_engine_peer(
        fixture.stream.take().unwrap(),
        fixture.child.as_mut().unwrap(),
        &fixture.image,
        channel,
        Duration::from_secs(5),
        Duration::from_secs(5),
    )
    .unwrap();
    control.write_all(b"!").unwrap();
    AuthenticatedTestPeer {
        peer,
        child: fixture.child.take().unwrap(),
        _directory: fixture.directory.take().unwrap(),
    }
}
