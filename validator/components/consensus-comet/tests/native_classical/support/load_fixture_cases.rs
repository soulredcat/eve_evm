// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use serde_json::Value;

pub fn load_fixture_cases(domain: &str, key: &str) -> Vec<Value> {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let directory = if key == "zip215_cases" {
        root.join("zip215-upstream")
    } else {
        root.join("native-classical").join(domain)
    };
    let mut paths: Vec<_> = std::fs::read_dir(directory)
        .expect("versioned native fixture directory")
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|value| value == "json"))
        .collect();
    paths.sort();
    assert!(
        !paths.is_empty(),
        "native fixture discovery must not silently select zero cases"
    );
    let mut cases = Vec::new();
    for path in paths {
        let fixture: Value = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
        assert_eq!(
            fixture["source_revision"],
            "0880b4d378f347ab16e54ec677ff50d803f37d62"
        );
        cases.extend(
            fixture[key]
                .as_array()
                .expect("exact native fixture schema")
                .iter()
                .cloned(),
        );
    }
    cases
}
