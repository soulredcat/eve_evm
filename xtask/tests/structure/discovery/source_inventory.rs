use crate::support::Fixture;
use std::process::Command;
use xtask::structure::discovery::discover_sources::discover_sources;

#[test]
fn tracked_and_new_sources_are_discovered_once_with_spaces_and_unicode() {
    let fixture = Fixture::new();
    fixture.write("docs/encoding notes.md", "Reviewed encoding contract.\n");
    fixture.write("docs/height-λ.md", "Protocol symbol fixture.\n");
    let result = Command::new("git")
        .args(["add", "--", "docs/encoding notes.md"])
        .current_dir(fixture.root())
        .output()
        .unwrap();
    assert!(result.status.success());
    fixture.write(
        "validator/src/encode_height.rs",
        "pub fn encode_height() {}\n",
    );
    let sources = discover_sources(fixture.root()).unwrap();
    for path in [
        "docs/encoding notes.md",
        "docs/height-λ.md",
        "validator/src/encode_height.rs",
    ] {
        assert_eq!(
            sources
                .iter()
                .filter(|source| source.as_str() == path)
                .count(),
            1
        );
    }
    fixture.assert_pass();
}

#[test]
fn unsupported_first_party_language_fails_with_explicit_coverage_message() {
    let fixture = Fixture::new();
    fixture.write("integration/encode_height.ts", "export const height = 1;\n");
    let report = fixture.assert_rejected();
    assert!(
        report
            .violations
            .iter()
            .any(|message| message.contains("unsupported"))
    );
}

#[test]
fn deleted_index_entries_do_not_hide_the_remaining_dirty_sources() {
    let fixture = Fixture::new();
    fixture.write(
        "validator/src/removed_operation.rs",
        "pub fn removed_operation() {}\n",
    );
    assert!(
        Command::new("git")
            .args(["add", "--", "validator/src/removed_operation.rs"])
            .current_dir(fixture.root())
            .status()
            .unwrap()
            .success()
    );
    std::fs::remove_file(fixture.root().join("validator/src/removed_operation.rs")).unwrap();
    fixture.write(
        "validator/src/encode_height.rs",
        "pub fn encode_height() {}\n",
    );
    let paths = discover_sources(fixture.root()).unwrap();
    assert!(
        !paths
            .iter()
            .any(|path| path.ends_with("removed_operation.rs"))
    );
    assert!(paths.contains(&"validator/src/encode_height.rs".to_owned()));
    fixture.assert_pass();
}
