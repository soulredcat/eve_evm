// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::collections::BTreeSet;

/// Publish bounded registered identifiers only; panic messages and raw output stay private.
pub fn summarize_registered_test_failures(stdout: &str, registered: &[String]) -> String {
    let allowed: BTreeSet<&str> = registered
        .iter()
        .map(String::as_str)
        .filter(|name| {
            name.len() <= 512
                && name.split("::").all(|segment| {
                    segment
                        .as_bytes()
                        .first()
                        .is_some_and(|byte| byte.is_ascii_alphabetic() || *byte == b'_')
                        && segment
                            .bytes()
                            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
                })
        })
        .collect();
    let mut failed = BTreeSet::new();
    let mut failure_section = false;
    for line in stdout.lines() {
        if line == "failures:" {
            failure_section = true;
        } else if line.starts_with("test result:") {
            failure_section = false;
        }
        if let Some((name, status)) = line
            .strip_prefix("test ")
            .and_then(|line| line.split_once(" ... "))
            && status == "FAILED"
            && allowed.contains(name)
        {
            failed.insert(name);
        }
        if failure_section && line.starts_with("    ") && allowed.contains(line.trim()) {
            failed.insert(line.trim());
        }
    }
    if failed.is_empty() {
        "NO_REGISTERED_FAILURE_ID".into()
    } else {
        let omitted = failed.len().saturating_sub(32);
        let names = failed.into_iter().take(32).collect::<Vec<_>>().join(", ");
        if omitted == 0 {
            names
        } else {
            format!("{names}; {omitted} additional registered failures omitted")
        }
    }
}
