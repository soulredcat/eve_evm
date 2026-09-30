use super::super::artifacts::{validate_archive_links, validate_archive_members};

#[test]
fn rejects_traversal_foreign_absolute_and_ambiguous_archive_members() {
    for input in [
        "go/../escape",
        "/go/file",
        "foreign/file",
        "go/path\twith-tab",
        "go/path\\escape",
        "",
    ] {
        assert!(validate_archive_members(input, "go").is_err(), "{input:?}");
    }
    assert!(validate_archive_members("go/\ngo/bin/go\ngo/src/main.go\n", "go").is_ok());
    // Pinned Comet includes one ordinary workflow filename ending in a space.
    assert!(validate_archive_members("go/.github/workflows/e2e-nightly-38x.yml \n", "go").is_ok());
}

#[test]
fn permits_contained_links_and_rejects_escaping_or_special_entries() {
    let good = "lrwxrwxrwx 1/1 0 2026-09-30 00:00:00 node/bin/npm -> ../lib/npm.js\n";
    assert!(validate_archive_links(good, "node").is_ok());
    for input in [
        "lrwxrwxrwx 1/1 0 2026-09-30 00:00:00 node/bin/npm -> ../../outside\n",
        "lrwxrwxrwx 1/1 0 2026-09-30 00:00:00 node/bin/npm -> /etc/passwd\n",
        "hrw-r--r-- 1/1 0 2026-09-30 00:00:00 node/file link to other/file\n",
        "prw-r--r-- 1/1 0 2026-09-30 00:00:00 node/fifo\n",
        "broken metadata\n",
    ] {
        assert!(validate_archive_links(input, "node").is_err(), "{input:?}");
    }
}
