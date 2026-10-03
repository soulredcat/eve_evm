// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

#[test]
fn process_image_binding_rejects_unreadable_and_different_executables_with_fixed_codes() {
    let directory = tempfile::tempdir().unwrap();
    let (config, digest) = super::support::config(directory.path());
    let image = crate::development::engine::verification::verify_test_engine_image(
        &config.comet_binary,
        digest,
    )
    .unwrap();
    let absent =
        crate::development::engine::verification::validate_engine_image_binding(&image, u32::MAX)
            .err()
            .unwrap();
    assert_eq!(absent.kind(), std::io::ErrorKind::NotFound);
    assert_eq!(absent.to_string(), "ENGINE_PROCESS_IMAGE_UNREADABLE");
    let different = crate::development::engine::verification::validate_engine_image_binding(
        &image,
        std::process::id(),
    )
    .err()
    .unwrap();
    assert_eq!(different.kind(), std::io::ErrorKind::PermissionDenied);
    assert!(matches!(
        different.to_string().as_str(),
        "ENGINE_PROCESS_DEVICE_MISMATCH" | "ENGINE_PROCESS_INODE_MISMATCH"
    ));
}

#[test]
fn caller_image_that_never_leaves_exec_times_out_as_incomplete() {
    let incomplete = crate::development::engine::verification::await_engine_exec(
        std::process::id(),
        std::time::Duration::from_millis(50),
    )
    .err()
    .unwrap();
    assert_eq!(incomplete.kind(), std::io::ErrorKind::TimedOut);
    assert_eq!(incomplete.to_string(), "ENGINE_PROCESS_EXEC_INCOMPLETE");
}

#[test]
fn completed_exec_returns_the_spawned_image_without_waiting_for_the_limit() {
    let sleep = std::env::split_paths(&std::env::var_os("PATH").unwrap())
        .map(|directory| directory.join("sleep"))
        .find(|path| path.is_file())
        .unwrap();
    let expected = crate::development::engine::verification::engine_file_identity(
        &std::fs::File::open(&sleep).unwrap(),
    )
    .unwrap();
    let mut child = std::process::Command::new(&sleep)
        .arg("30")
        .spawn()
        .unwrap();
    let started = std::time::Instant::now();
    let identity = crate::development::engine::verification::await_engine_exec(
        child.id(),
        std::time::Duration::from_secs(30),
    );
    child.kill().unwrap();
    child.wait().unwrap();
    assert!(identity.unwrap() == expected);
    assert!(started.elapsed() < std::time::Duration::from_secs(5));
}
