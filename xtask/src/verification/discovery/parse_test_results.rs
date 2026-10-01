// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Context, Result, ensure};

pub fn parse_test_results(output: &str, expected: usize) -> Result<usize> {
    let mut passed = 0_usize;
    let mut summaries = 0;
    for line in output
        .lines()
        .filter(|line| line.starts_with("test result:"))
    {
        ensure!(
            line.starts_with("test result: ok. "),
            "A test binary failed"
        );
        let fields: Vec<_> = line
            .trim_start_matches("test result: ok. ")
            .split(';')
            .collect();
        ensure!(fields.len() >= 5, "Malformed test summary");
        let count: usize = fields[0]
            .trim()
            .strip_suffix(" passed")
            .context("Missing passed count")?
            .parse()?;
        for (field, suffix) in
            fields[1..5]
                .iter()
                .zip([" failed", " ignored", " measured", " filtered out"])
        {
            let count: usize = field
                .trim()
                .strip_suffix(suffix)
                .context("Missing test result count")?
                .parse()?;
            ensure!(
                count == 0,
                "Failed, ignored, measured or filtered tests cannot pass a gate"
            );
        }
        passed = passed.checked_add(count).context("Test count overflow")?;
        summaries += 1;
    }
    ensure!(
        summaries > 0 && passed > 0 && passed == expected,
        "Executed test count differs: expected {expected}, passed {passed}"
    );
    Ok(passed)
}
