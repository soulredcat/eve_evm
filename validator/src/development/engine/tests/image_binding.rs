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
