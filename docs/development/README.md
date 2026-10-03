<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Development and contribution contract

Start with [architecture](../architecture/README.md), [execution status](../execution/README.md),
[roadmap](../plan/README.md), [security inventory](../security/inventory/README.md)
and the affected directory README. GitHub-only collaborators can use these public
contracts without access to local owner documents. Report missing requirements
or conflicting instructions before making a protocol assumption.

## Ownership, language and file organization

Use requires prior written permission from Redcat under [LICENSE](../../LICENSE).
Repository access does not grant another license. Every first-party publishable
file needs these notices in valid comment syntax:

```text
SPDX-FileCopyrightText: 2026 Redcat
SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
Use requires prior written permission from Redcat.
```

For JSON, generated locks or immutable data, use reviewed exact associations in
`REUSE.toml` instead of modifying payload bytes. Preserve upstream rights,
licenses and pinned material. Notices are not cryptographic signatures.

Use English in first-party source comments, documentation, configuration/API/operator
text, test descriptions, commits, issues and PRs. Preserve canonical identifiers,
fixture bytes, proper names and attributed upstream text.

Follow canonical runtime ownership and idiomatic component boundaries. Use meaningful
`runtime-or-crate/src/domain/capability/operation/function_name.rs` nesting.
One handwritten production behavior file owns one primary operation; put other
behavioral helpers in named files. Types, facades and minimal trait-delegation
adapters must remain explicit and contain no hidden unrelated behavior.

Count complete formatted physical lines, including notices and blank lines:
target at most 200; 201–400 requires decomposition review and a recorded rationale;
401–600 requires a reviewed exact-path temporary exception and split task; above
600 fails. Do not minify, raise limits or add blanket exclusions to pass.

## Collaboration workflow

The latest owner instruction permits subagents again. Assign explicit task and
file ownership before parallel work. The integrator owns shared manifests,
lockfiles and final commits; perform responsibilities sequentially when parallel
work is unavailable or would conflict. Reviews during a frozen gate are read-only:
do not edit its source, rebuild authenticated executables or control its processes.
Separate human or external-AI contributions require a scoped task and owned paths.

1. Inspect branch, HEAD and dirty files; preserve unrelated work.
2. State the exact base revision, problem, owned paths, dependencies, forbidden
   changes and required tests. Use a `codex/` branch for a scoped contribution.
3. Implement the current dependency-ready step and its positive/negative tests.
   Avoid concurrent ownership of manifests, locks and consensus interfaces.
4. Open a draft PR when publication is authorized. Report commands, actual exits,
   tested SHA, local/hosted results and limitations. Never claim another person's
   tests ran locally.
5. The integrator reviews the diff, required gates, publication policy and evidence
   before owner-authorized main integration. A draft PR is not gate acceptance.

Current work is the unresolved hosted B3 T-C07 submission failure. B4 foundations
exist in a separate local checkpoint; dependent integration requires accepted B3.
Preserve quorum, signing durability and executable identity checks. No destructive
reset, forced push, history rewrite, unsolicited
dependency upgrades or changes to unrelated processes are permitted.

## Reproduce verification

Use Linux x86_64 with the exact Rust/toolchain and native prerequisites in
`rust-toolchain.toml`, `config/ci-environment.toml` and the
[CI workflow](../../.github/workflows/foundation.yml). That workflow is the
reference for the pinned container and receipt environment. Do not substitute
mutable engine images, arbitrary tool versions or machine-specific public config.

```sh
cargo xtask provision-tools --jobs 2
cargo xtask check-structure --report local-tests/structure.json
cargo xtask check-ownership --report local-tests/ownership.json
cargo xtask verify --bulk B3
```

Provisioning creates `local-tests/toolchain-b0/provisioned-tools.json`. Verification
revalidates this receipt and passes checked tool identities to child commands;
CI explicitly exports its receipt environment before the gate. Consult the
versioned workflow for the required native packages and exact export procedure.

The complete gate retains foundations and checks structure, ownership, format,
strict Clippy, registered test discovery/execution and release builds. Required
cases are in `config/gates/groups/`. Missing, duplicate, unregistered, skipped,
ignored, filtered or zero requested cases fail. Source mutation during a frozen
gate fails. `verify --all` does not pass while later mandatory gates are missing.

Reports in ignored `local-tests/verify-*/` bind revision, source/config identity,
dirty state, compiler/tools, counts, exits and limitations. Keep raw reports
local; publish reviewed compact summaries in [execution status](../execution/README.md).
Never rebuild an authenticated running executable during a test.

## Publication review

Only Markdown files named `README.md` may be tracked. Keep `AGENTS.md`, `goal.md`,
`CONTRIBUTING.md`, detailed plans/evidence and exploratory files local. Required
reproducible tests and sanitized fixtures belong in tracked test modules or `tests/`.
Never publish raw logs, generated dependencies/builds, disposable databases,
credentials, private keys, personal environment files or machine runtime config.

Review `git diff --check`, the complete staged diff and every outgoing commit
against its destination. Verify tracked Markdown names, Redcat notices, English
prose and absence of prohibited artifacts/secrets. `.gitignore` is not a security
boundary; never force-add local files. Push only with current owner authorization.
Leave task-owned tracked changes committed and preserve unrelated edits.

Mainnet, real funds/custody/signing, paid provisioning, visibility changes,
additional license grants and irreversible genesis decisions require separate
owner approval. A checkpoint published despite failed CI remains a checkpoint.
