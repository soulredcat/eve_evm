// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

pub(super) fn upstream_license(path: &str) -> Option<&'static str> {
    if path.starts_with("validator/components/consensus-comet/vendor/cometbft/")
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
