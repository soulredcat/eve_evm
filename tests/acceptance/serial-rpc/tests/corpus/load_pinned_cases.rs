// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::{CorpusInventory, FixtureCase};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, path::PathBuf};

pub fn load_pinned_cases() -> Vec<(String, FixtureCase)> {
    let root = PathBuf::from(
        std::env::var_os("EVE_SHANGHAI_FIXTURES")
            .expect("EVE_SHANGHAI_FIXTURES must name the verified immutable extraction root"),
    );
    let metadata = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/corpus/shanghai");
    let mut cases = Vec::new();
    let mut files = 0;
    for family in [
        "berlin",
        "byzantium",
        "cancun",
        "constantinople",
        "frontier",
        "homestead",
        "istanbul",
        "london",
        "osaka",
        "ported_static",
        "shanghai",
    ] {
        let inventory: CorpusInventory = serde_json::from_slice(
            &std::fs::read(metadata.join(format!("{family}.json"))).unwrap(),
        )
        .unwrap();
        assert_eq!(inventory.schema_version, 1);
        assert_eq!(
            inventory.source_revision,
            "abbe05777ab83fb94ce18c425daaa7ab79e779c1"
        );
        assert_eq!(
            inventory.archive_sha256,
            "1280540950a4c3470a421416b6f35458a9b635827265c29e5aef1ae839ae1788"
        );
        assert_eq!(inventory.target_fork, "Shanghai");
        for (relative, digest) in inventory.files {
            assert!(relative.starts_with("fixtures/state_tests/for_shanghai/"));
            assert!(!relative.contains("..") && !relative.contains('\\'));
            let path = root.join(&relative);
            let file_metadata = std::fs::symlink_metadata(&path).unwrap();
            assert!(file_metadata.is_file() && !file_metadata.file_type().is_symlink());
            assert!(file_metadata.len() <= 64 * 1024 * 1024);
            let bytes = std::fs::read(&path).unwrap();
            assert_eq!(hex::encode(Sha256::digest(&bytes)), digest, "{relative}");
            let document: BTreeMap<String, FixtureCase> = serde_json::from_slice(&bytes)
                .unwrap_or_else(|error| panic!("{relative}: {error}"));
            for (name, case) in document {
                assert_eq!(case.post.keys().collect::<Vec<_>>(), ["Shanghai"]);
                assert_eq!(case.post["Shanghai"].len(), 1, "{name}");
                assert_eq!(
                    case.env.gas_limit,
                    alloy_primitives::U256::from(120_000_000)
                );
                cases.push((format!("{relative}::{name}"), case));
            }
            files += 1;
        }
    }
    assert_eq!(
        files, 194,
        "The entire pinned Shanghai state workload must be discovered"
    );
    assert_eq!(cases.len(), 3495, "No fixture-family sampling is allowed");
    cases
}
