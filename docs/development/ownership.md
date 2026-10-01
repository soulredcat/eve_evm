<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Redcat ownership notices

The owner requires an ownership and copyright notice for every first-party
publishable file. Covered original material uses `2026 Redcat` and
`LicenseRef-Redcat-Permission-Only`. Use, execution, copying, modification,
derivative works and distribution require prior written permission from Redcat,
subject to rights already granted by applicable law, mandatory hosting platform
terms or a separate applicable license. The complete terms are in
[the permission-only text](../../LICENSES/LicenseRef-Redcat-Permission-Only.txt);
the [root notice](../../LICENSE) explains repository scope.

This is a reserved-rights notice, with no open-source grant. It is not a digital
signature, certificate, identity verification, access control or technical
restriction. Publication does not prevent copying, and public hosting terms may
require rights to view or fork the repository. No repository visibility change,
real signing key or production release is authorized by adding these notices.

## Formats and placement

Commentable first-party files carry these three lines at the top, using the
format's existing comment syntax, followed by a blank line:

<!-- REUSE-IgnoreStart -->
```text
SPDX-FileCopyrightText: 2026 Redcat
SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
Use requires prior written permission from Redcat.
```
<!-- REUSE-IgnoreEnd -->

| Format | Comment placement |
|---|---|
| Rust, TypeScript and first-party Solidity | Three `//` comment lines |
| TOML, YAML, `.gitignore` and `.gitattributes` | Three `#` comment lines |
| Markdown | Three separate HTML comment lines |
| JSON, generated locks and canonical checksum manifests | Exact file association in root `REUSE.toml`; preserve payload bytes |
| Root `LICENSE` and `LICENSES/` texts | Plain license text; preserve upstream license texts |

Keep a shebang, byte-order mark or required format preamble valid if one is
introduced. Replace an obsolete first-party Solidity `UNLICENSED` marker rather
than publishing contradictory license identifiers. Do not modify upstream
headers, add comments to JSON or duplicate cryptographic implementations.

The [REUSE specification 3.3](https://reuse.software/spec-3.3/) supports exact
`REUSE.toml` file associations when inline comments are undesirable or impossible.
Its `closest` precedence preserves inline declarations and avoids overriding
upstream notices. [SPDX 2.3 license expressions](https://spdx.github.io/spdx-spec/v2.3/SPDX-license-expressions/)
support custom `LicenseRef-` identifiers. The identifier refers to this project's
actual permission terms; it does not imply SPDX approval or an open-source license.

## Scope and upstream preservation

Root `REUSE.toml` lists each file requiring external metadata by exact path.
It uses no broad JSON or vendor glob. New noncommentable files require a reviewed
annotation. Ignore rules keep local tests, dependency trees, raw output and build
artifacts outside the publishable inventory; tracked files still require review.

| Material | Ownership and license treatment |
|---|---|
| First-party source, prose, tests and configuration | Redcat notice and permission-only terms |
| First-party JSON state/protocol fixtures | Redcat association in `REUSE.toml`; exact protocol/reference bytes preserved |
| Corpus path/checksum inventories | Redcat original selection/schema only; factual digests, upstream paths and CC0 payload rights remain unrestricted |
| Cargo/npm generated dependency locks | Redcat project-specific selection only; no claim to dependency code or factual identities |
| CometBFT protobufs and upstream `LICENSE` | Exact upstream Apache-2.0 material; unchanged bytes and adjacent attribution |
| Cosmos gogoproto source and upstream `LICENSE` | Exact upstream BSD-3-Clause material; unchanged bytes and copyright notices |
| NIST ACVP input/result/derived-case JSON and notice | Preserved NIST material and actual upstream terms; no Redcat permission condition on source inputs or expected outcomes |

NIST's [pinned source notice](https://github.com/usnistgov/ACVP-Server/blob/a7f283cdc87d2d6dd93c1bac59e5622c5f9f8324/README.md#license)
is represented by `LicenseRef-NIST-ACVP-Upstream`, with its actual text preserved
under `LICENSES/`. The existing source notice differs from the SPDX `NIST-PD`
and `NIST-PD-fallback` templates; do not replace it with a different grant.
`SPDX-FileCopyrightText = "NOASSERTION"` avoids asserting a global copyright
status or an invented holder; the provenance comment identifies NIST as its source.

The NIST notice's project-written provenance introduction remains attached to
the upstream notice without imposing Redcat restrictions on that composite file.
The derived case join is attributed upstream material, including its schema
adaptation. This conservative treatment does not assert proprietary ownership
over public inputs, official outcomes or preserved legal text.

Comet/gogoproto generated Rust output remains in untracked Cargo `OUT_DIR`.
Downloaded external Ethereum reference payloads and compiler/dependency output
remain in ignored local storage. Their upstream licenses still apply there.
Notices on project-authored inventories do not relicense upstream payloads.
Third-party licenses in `LICENSES/` reproduce existing upstream terms; they do not
grant those terms to first-party EVE material.

## Change and verification rules

Include notices in every newly added first-party file. Count notice comments and
blank lines toward plan 25's complete physical-line limits. Meaningfully split a
file or record a permitted reviewed rationale when a header crosses a threshold;
do not remove comments, minify or raise limits to hide the increase.

Keep immutable fixtures, generated locks and upstream inputs byte-for-byte intact
unless their separate update process is explicitly invoked. A header changes
first-party source identity: refresh any pinned handwritten-generator digest only
after reviewing the exact notice-only change, then rerun the affected structure
and build gates. External annotation avoids changing checksum-manifest bytes.

Review the publishable Git inventory, every inline notice, exact external
association, complete license texts and preserved upstream digests before
publication. Ownership checks do not replace correctness/security/structure
gates or full standalone distribution license review. Report actual automated
coverage and missing checks; do not claim complete REUSE conformance solely from
the presence of a `REUSE.toml` file.

`cargo xtask check-ownership --report local-tests/ownership-report.json` implements
this repository policy over tracked and new non-ignored files. It reports inline
first-party, annotated first-party, upstream and license-text coverage, and fails
for missing/conflicting notices, unsafe/stale/overlapping annotation paths,
unsupported metadata, missing texts, upstream Redcat claims or changed registered
upstream digests. Its isolated tests are under `xtask/tests/ownership/`.
Supported external annotations use single license identifiers and `closest`
precedence. This bounded gate does not validate every REUSE/SPDX feature, decide
copyright ownership, replace legal review or certify dependency distributions.
