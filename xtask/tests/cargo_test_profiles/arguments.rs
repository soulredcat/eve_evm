// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use xtask::verification::orchestration::cargo_test_arguments::cargo_test_arguments;

#[test]
fn ordinary_and_doc_profiles_preserve_exact_mandatory_cargo_selection() {
    assert_eq!(
        cargo_test_arguments("fixture", "", false, false),
        ["test", "--locked", "--all-targets", "-p", "fixture"]
    );
    assert_eq!(
        cargo_test_arguments("fixture", "", false, true),
        [
            "test",
            "--locked",
            "--all-targets",
            "-p",
            "fixture",
            "--release"
        ]
    );
    assert_eq!(
        cargo_test_arguments("fixture", "", true, false),
        ["test", "--locked", "-p", "fixture", "--doc"]
    );
    assert_eq!(
        cargo_test_arguments("fixture", "", true, true),
        ["test", "--locked", "-p", "fixture", "--doc", "--release"]
    );
}

#[test]
fn feature_names_remain_borrowed_and_identical_across_every_profile() {
    let package = String::from("fixture");
    let features = String::from("development-acceptance,fixture-mode");
    for docs in [false, true] {
        for release in [false, true] {
            let arguments = cargo_test_arguments(&package, &features, docs, release);
            let package_index = arguments.iter().position(|value| *value == "-p").unwrap() + 1;
            let features_index = arguments
                .iter()
                .position(|value| *value == "--features")
                .unwrap()
                + 1;
            assert_eq!(arguments[package_index].as_ptr(), package.as_ptr());
            assert_eq!(arguments[features_index].as_ptr(), features.as_ptr());
            assert_eq!(
                arguments
                    .iter()
                    .filter(|value| **value == "--locked")
                    .count(),
                1
            );
            assert_eq!(
                arguments
                    .iter()
                    .filter(|value| **value == "--release")
                    .count(),
                usize::from(release)
            );
            assert_eq!(
                arguments
                    .iter()
                    .filter(|value| **value == "--features")
                    .count(),
                1
            );
        }
    }
}

#[test]
fn listing_and_execution_share_prefix_without_profile_specific_test_skipping() {
    for docs in [false, true] {
        for release in [false, true] {
            let base = cargo_test_arguments("fixture", "feature-one", docs, release);
            assert!(!base.iter().any(|value| matches!(
                *value,
                "--" | "--skip" | "--ignored" | "--nocapture" | "--list"
            )));
            let mut listing = base.clone();
            listing.extend(["--", "--list"]);
            let mut execution = base.clone();
            execution.extend(["--", "--test-threads=1", "--nocapture"]);
            assert_eq!(&listing[..base.len()], &execution[..base.len()]);
            assert_eq!(&listing[base.len()..], ["--", "--list"]);
            assert_eq!(
                &execution[base.len()..],
                ["--", "--test-threads=1", "--nocapture"]
            );
        }
    }
}
