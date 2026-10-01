<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Publishable ownership verification

Canonical owner: repository tooling. This module verifies the approved Redcat
notice scheme and preserved upstream licensing over the existing Git publishable
inventory. It has no chain execution, signing, finality or runtime authority.

`check_ownership::check_ownership(&Path)` returns a serialized report with exact
file classifications, coverage counters and violations. Configuration/discovery
failures return an error. CLI and bulk integration must fail when either the
operation fails or its report contains violations; writing a report is not a pass.

| Operation | Responsibility |
|---|---|
| `load_annotations` | Parse the reviewed exact-path REUSE associations and reject unsafe, overlapping or stale registrations |
| `inspect_ownership_file` | Read a safely resolved inventory path and select its licensing category |
| `verify_inline_notice` | Check the exact three-line first-party notice and its blank separator with BOM/shebang handling |
| `notice_markers` | Inspect comment declarations while excluding explicitly marked Markdown examples |
| `validate_annotation` | Enforce first-party or known-upstream attribution without using metadata to conceal source notices |
| `validate_license_texts` | Require referenced plain license texts and Redcat's law/platform preservation clauses |
| `verify_upstream_digest` | Compare reviewed exact upstream identities from structure-policy vendor pins |
| `upstream_license` | Identify the known CometBFT, Cosmos gogoproto and NIST licensing scopes |
| `value_items` | Normalize the reviewed single/list TOML metadata representation |

Dependencies point from publication tooling to the existing source inventory,
safe path resolver, policy reader and digest operation in structure tooling.
Production role packages do not depend on this module. Tests live under
`xtask/tests/ownership/` and create isolated disposable Git repositories.

Supported annotations use exact paths, `closest` precedence and single license
identifiers, with one reviewed ZIP-215 upstream Apache/BSD conjunction. Its two
separate license texts are required; reversed/OR/nested expressions fail. First-party
material still requires the single Redcat identifier. Supported inline formats
are Rust, TypeScript, Solidity, TOML, YAML, Markdown and the two Git configuration
files. New formats or upstream families require a reviewed extension; unknown
formats fail. This is a repository policy gate, not full REUSE conformance,
arbitrary SPDX-expression validation, copyright adjudication or legal approval.

Known immutable Comet/gogoproto files, copied Apache/BSD license texts and original
NIST input/result JSON and exact ZIP-215 upstream files require vendor pins. All additional vendor pins in
the structure policy are also checked. The NIST derived-case join and adjacent
provenance notice remain upstream-attributed; no hash guarantee is claimed for an
unregistered upstream file. Complete distribution dependency notices remain a
separate release/standalone packaging responsibility.
