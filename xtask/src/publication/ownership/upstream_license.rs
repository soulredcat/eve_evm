// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

pub(super) fn upstream_license(path: &str) -> Option<&'static str> {
    if let Some(file) =
        path.strip_prefix("validator/components/consensus-comet/tests/fixtures/zip215-upstream/")
    {
        match file {
            "zip215_cases-upstream_speccheck_0.json"
            | "zip215_cases-upstream_speccheck_1.json"
            | "zip215_cases-upstream_speccheck_2.json"
            | "zip215_cases-upstream_speccheck_3.json"
            | "zip215_cases-upstream_speccheck_4.json"
            | "zip215_cases-upstream_speccheck_5.json"
            | "zip215_cases-upstream_speccheck_6.json"
            | "zip215_cases-upstream_speccheck_7.json"
            | "zip215_cases-upstream_speccheck_8.json"
            | "zip215_cases-upstream_speccheck_9.json"
            | "zip215_cases-upstream_speccheck_10.json"
            | "zip215_cases-upstream_speccheck_11.json"
            | "NOTICE" => Some("Apache-2.0 AND BSD-3-Clause"),
            "LICENSE" => Some("BSD-3-Clause"),
            "LICENSE-NOVI" => Some("Apache-2.0"),
            _ => None,
        }
    } else if path.starts_with("validator/components/consensus-comet/vendor/cometbft/")
        || path == "LICENSES/Apache-2.0.txt"
    {
        Some("Apache-2.0")
    } else if path.starts_with("validator/components/consensus-comet/vendor/gogoproto/")
        || path == "LICENSES/BSD-3-Clause.txt"
    {
        Some("BSD-3-Clause")
    } else if path.starts_with("validator/components/authentication/tests/fixtures/nist-acvp/")
        || path == "LICENSES/LicenseRef-NIST-ACVP-Upstream.txt"
    {
        Some("LicenseRef-NIST-ACVP-Upstream")
    } else {
        None
    }
}
