use super::super::clients::compute_client_tree_digest;

#[test]
fn client_tree_digest_detects_changed_bytes_paths_and_added_files() {
    let directory = tempfile::tempdir().unwrap();
    std::fs::write(directory.path().join("a.js"), b"public API fixture").unwrap();
    let initial = compute_client_tree_digest(directory.path()).unwrap();
    assert_eq!(
        compute_client_tree_digest(directory.path()).unwrap(),
        initial
    );
    std::fs::write(directory.path().join("a.js"), b"modified API fixture").unwrap();
    assert_ne!(
        compute_client_tree_digest(directory.path()).unwrap(),
        initial
    );
    std::fs::write(directory.path().join("a.js"), b"public API fixture").unwrap();
    std::fs::rename(directory.path().join("a.js"), directory.path().join("b.js")).unwrap();
    assert_ne!(
        compute_client_tree_digest(directory.path()).unwrap(),
        initial
    );
    std::fs::rename(directory.path().join("b.js"), directory.path().join("a.js")).unwrap();
    std::fs::write(directory.path().join("extra.js"), b"extra fixture").unwrap();
    assert_ne!(
        compute_client_tree_digest(directory.path()).unwrap(),
        initial
    );
}
