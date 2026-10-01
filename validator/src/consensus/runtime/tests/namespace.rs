// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::runtime::{namespace::open_node_namespace, types::NodeIdentity};
use std::os::unix::fs::{PermissionsExt, symlink};

fn identity() -> NodeIdentity {
    NodeIdentity {
        schema: 1,
        genesis_hash: "01".repeat(32),
        public_key: "02".repeat(32),
        engine_sha256: "03".repeat(32),
    }
}

#[test]
fn private_namespace_reopens_only_after_actual_lease_release() {
    let root = tempfile::tempdir().unwrap();
    let data = root.path().join("node");
    let lease = open_node_namespace(&data, &identity()).unwrap();
    assert!(open_node_namespace(&data, &identity()).is_err());
    drop(lease);
    assert!(open_node_namespace(&data, &identity()).is_ok());
    assert_eq!(
        std::fs::metadata(data).unwrap().permissions().mode() & 0o777,
        0o700
    );
}

#[test]
fn changed_genesis_key_and_binary_identity_cannot_reopen() {
    let root = tempfile::tempdir().unwrap();
    let data = root.path().join("node");
    drop(open_node_namespace(&data, &identity()).unwrap());
    for changed in [
        NodeIdentity {
            genesis_hash: "04".repeat(32),
            ..identity()
        },
        NodeIdentity {
            public_key: "04".repeat(32),
            ..identity()
        },
        NodeIdentity {
            engine_sha256: "04".repeat(32),
            ..identity()
        },
    ] {
        assert!(open_node_namespace(&data, &changed).is_err());
    }
}

#[test]
fn unmarked_nonempty_or_insecure_parent_is_rejected() {
    let root = tempfile::tempdir().unwrap();
    let data = root.path().join("node");
    std::fs::create_dir(&data).unwrap();
    assert!(open_node_namespace(&data, &identity()).is_err());
    std::fs::set_permissions(&data, std::fs::Permissions::from_mode(0o700)).unwrap();
    std::fs::write(data.join("foreign"), b"preserve").unwrap();
    assert!(open_node_namespace(&data, &identity()).is_err());
    assert_eq!(std::fs::read(data.join("foreign")).unwrap(), b"preserve");
}

#[test]
fn namespace_rejects_symlinks_traversal_and_foreign_repository_child() {
    let root = tempfile::tempdir().unwrap();
    let data = root.path().join("node");
    drop(open_node_namespace(&data, &identity()).unwrap());
    symlink(root.path(), data.join("state")).unwrap();
    assert!(open_node_namespace(&data, &identity()).is_err());
    assert!(open_node_namespace(&root.path().join("node/../other"), &identity()).is_err());
    let alias = root.path().join("alias");
    symlink(&data, &alias).unwrap();
    assert!(open_node_namespace(&alias, &identity()).is_err());
}
