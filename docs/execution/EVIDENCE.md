# Evidence index

## Current evidence

No runtime code, build, test, deployment, fault injection or benchmark has been executed as part of this planning package. There are no passing runtime evidence records yet.

The repository documentation is an implementation contract. Its existence is not evidence that consensus, EVM compatibility, storage recovery, tokenomics or 1M TPS works.

## PUB-20260930-01 — Local test isolation and publication policy

Scope: repository hygiene and English collaboration only. This record belongs to the local policy commit based on clean branch `main` at `027ef9dc5779098f1192c56308cf7eb6c21bd13e`; it does not close B0, SEC0, INT0, or any runtime gate.

- Implemented root `.gitignore`, ignored machine-local `local-tests/`, and English publication rules in `CONTRIBUTING.md`, `AGENTS.md`, the goal, test guidance, and decision register.
- Verified eight local/output/environment path cases with `git check-ignore -v --no-index -- <path>`: exit code 0 in every case, including a nested `local-tests/` path.
- Verified five shared-path cases with `git check-ignore -q --no-index -- <path>`: expected exit code 1 (not ignored) for `tests/README.md`, a JSON fixture path, `Cargo.lock`, `.env.example`, and `.env.test.example`.
- `git ls-files -- local-tests/ artifacts/ coverage/`: exit code 0, no tracked paths.
- `git diff --check`: exit code 0, no whitespace errors.
- The PowerShell verification batch completed with exit code 0. Its 15 Git checks, exact outputs, and SHA-256 identities/physical-line counts for 12 changed policy documents/configuration files are recorded in `local-tests/publication-policy-checks.json`. Every checked file remains below 200 physical lines.
- Local-only raw report SHA-256: `567882fc85daa94b59940b501e977a2449921beb88f0f188e0265bee66ae3f35`. The report is not available from GitHub; reproduce ignore checks using the commands in `CONTRIBUTING.md`.
- Changed first-party prose was reviewed in English. No automated language or secret-scanner certification is claimed. Ignore rules are bypassable; the index and every outgoing commit still require review.
- Runtime/build/security/capacity tests: `NOT_RUN`. `cargo xtask check-structure`: `NOT_IMPLEMENTED`. No runtime processes, keys, funds, or GitHub push were used.

## DOC-20260930-02 — Regional masters and public persistence

Scope: English documentation and acceptance registration only, based on clean branch `main` at `f02719ecd6e3e1a0ab6bd6d953e6161d7d8c8ca9`. The commit containing this record adds plan 32 and aligns the existing specifications; it does not implement a runtime or complete an implementation bulk.

- Recorded decisions D29–D33: default public RAM working state with durable recovery blocks/checkpoints and isolated bounded storage workers; eligible nearby logical sync endpoints and operational zones; intended one/two/ten independent master topology.
- Preserved validator finality, synchronous anti-double-sign persistence, exact EVM/fee semantics, authenticated H/H+1 state binding, last-copy retention, and the secured 1M mixed-workload release gate.
- Registered T-N09/T-N10 in B4, T-N11 in B6, and T-N12 in B8; B6/B8/B9 include the relevant integrated reruns. These four runtime tests are `NOT_IMPLEMENTED` / `NOT_RUN`.
- `powershell -NoProfile -File local-tests/validate-documentation.ps1`: exit code 0. Checked 54 repository Markdown files, 139 relative links/anchors, all T-N01–T-N12 definitions, four explicit new-case trace documents, and the B4/B6/B8 gate mapping. No missing local links, anchors or case mappings; maximum complete physical line count was 158, below the 200-line target. This local documentation audit is not `cargo xtask check-structure` or runtime acceptance.
- `git diff --check`, `git diff --cached --check`, `git ls-files -- local-tests/ artifacts/ coverage/`, and `git check-ignore -v --no-index -- local-tests/regional-public-docs-validation.json local-tests/validate-documentation.ps1`: exit code 0. No prohibited local paths were tracked; the report/checker are ignored. Shared tests/fixtures remain versioned.
- Local-only report `local-tests/regional-public-docs-validation.json` SHA-256: `45cd348c55858f568420ee2bc1cc1ee73be6ea4518751aa99759813a7f01d873`. Local checker SHA-256: `f541dde4b919ed95165a3b3efa5927ef9aa49d5240b69609afdd3f5265064a18`. The report records the exact reproduction command, base revision and SHA-256 identities of 25 changed documents; this evidence file is excluded from the identity bundle to avoid self-reference, but its links and line count are checked. These local artifacts are unavailable from GitHub.
- Protocol/reviewer and correctness/security agents reviewed the integrated contracts. Clarifications preserve valid older replay history, explicit `NOT_READY` on an unavailable recovery tail, and the distinct full `StateStore` versus public checkpoint/replay durability contracts. English first-party prose was reviewed; no automatic language/security certification or independent audit is claimed.
- Runtime/build/security/capacity tests: `NOT_RUN`; `cargo xtask check-structure`: `NOT_IMPLEMENTED`. No production deployment, background runtime process, funds, keys, or push are part of this documentation task.

## Record format for future runs

Each actual run record must include:

- Run ID, bulk/task IDs and covered requirements/test IDs.
- Tested commit and dirty-diff digest if applicable.
- Genesis/config/dependency/fixture identity.
- Environment and exact topology; distinguish local emulation from real hosts.
- Exact commands, exit codes, selected/executed test counts and pass/fail/not-run results.
- Expected invariant and observed outcome.
- Sanitized artifact paths/URLs, checksums and reproduction command.
- Known limitations, failed cases and next repair action.

Large artifacts stay outside Git with a verifiable manifest/reference. Local-only artifacts must be identified as local-only. Do not invent CI links, log files, independent audit results or downloaded benchmark evidence.

A new source/config change invalidates evidence for affected behavior until the relevant gate is rerun. The integrator verifies this before a bulk becomes DONE.
