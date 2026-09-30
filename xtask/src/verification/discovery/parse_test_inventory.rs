use anyhow::{Result, ensure};
use std::collections::BTreeSet;

pub fn parse_test_inventory(output: &str, required: &[String]) -> Result<BTreeSet<String>> {
    let mut tests = BTreeSet::new();
    for line in output.lines() {
        if let Some(name) = line.strip_suffix(": test") {
            ensure!(
                !name.trim().is_empty() && tests.insert(name.to_owned()),
                "Empty or duplicate test name"
            );
        }
    }
    ensure!(
        !tests.is_empty() && !required.is_empty(),
        "Zero requested or discovered tests cannot pass"
    );
    let expected: BTreeSet<_> = required.iter().cloned().collect();
    ensure!(
        expected.len() == required.len() && !expected.contains(""),
        "Invalid required test inventory"
    );
    let missing: Vec<_> = expected.difference(&tests).collect();
    ensure!(missing.is_empty(), "Missing required tests: {missing:?}");
    let unregistered: Vec<_> = tests.difference(&expected).collect();
    ensure!(
        unregistered.is_empty(),
        "Tests lack gate registration: {unregistered:?}"
    );
    Ok(tests)
}
