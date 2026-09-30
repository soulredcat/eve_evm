# Evidence index

## Current evidence

Component code, dependency/build checks and bounded foundation fixtures have been executed; records below distinguish historical checkpoints from current integration. Complete role runtimes, distributed fault acceptance, mainnet deployment and throughput targets remain unachieved.

The repository documentation is an implementation contract. Its existence is not evidence that consensus, EVM compatibility, storage recovery, tokenomics or 1M TPS works.

## B0-20261001 — Verified foundation bulk

[Full reviewed record](B0-20261001.md): integrated `cargo xtask verify --bulk B0`
including SEC0/INT0 exits 0, 205 cases pass with none ignored/failed, strict lint,
format, structure and workspace release build pass. Runtime/devnet/security/bridge/
standalone/capacity targets remain unachieved; GitHub CI execution is not inferred.

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

## LAYOUT-20260930-03 — Absolute role ownership and source relocation

Scope: source-preserving placement and initial compilation/structure checks on dirty branch `codex/implementation-foundation`, base HEAD `5dc49bd24d833188719a9b9fb94bb4981bf00f71`. This is an unfinished B0 checkpoint, not a completed runtime, security profile or standalone role package.

- Removed the empty root `crates/` after moving its source to `public/components/recovery-store/`, `validator/components/authentication/` and `validator/components/execution/`. Updated Cargo members, source policy, role READMEs and planning paths. No implementation was discarded.
- Added absolute role placement and independent copy/build/run requirements to `AGENTS.md`, with decisions D35/D36. Master-host composition does not change master/validator authority. Copy-ready role packages must include all local dependencies, lockfile/toolchain, sanitized examples and notices without paths escaping the copied directory.
- `cargo +1.97.1 metadata --locked --no-deps --format-version 1`: exit 0, all four package manifests resolve at the declared role/tool paths. Root `crates/` is absent. Local record: `local-tests/role-layout-verification.json`.
- `cargo +1.97.1 check -p xtask --locked`: exit 0 after correcting SHA-256 output encoding. `cargo +1.97.1 check -p eve-evm --locked`: exit 0 after explicit REVM handler error typing and the reviewed `Option<u8>` transaction-type API correction.
- `cargo +1.97.1 run --locked -p xtask -- check-structure --report local-tests/role-relocation-structure.json`: exit 0, 200 files, zero violations/warnings, three exact generated/vendor exclusions. Extracted block-environment population and a named proposer-fee suppression operation behind a reviewed thin trait adapter; no line limits or assertions were weakened.
- `git diff --check`: exit 0; 26 local Markdown targets in the changed role guidance resolved. Required NIST fixtures reside inside the authentication component; the derived ML-DSA-65 external/pure fixture preserves all 15 official cases (3 valid, 12 invalid). This extraction/placement is not execution of the crypto tests.
- Formatter/lint/release, T-L01–T-L06 boundary/regression tests and CI, integrated EVM/crypto/storage/runtime tests, standalone role copy/build/run and complete B0/SEC0/INT0 gates remain unfinished. No mainnet, real custody/signing keys, paid resources or GitHub push were used.

## PUB-20260930-04 — Tested foundation publication checkpoint

The owner explicitly authorized a GitHub push of branch `codex/implementation-foundation`. This checkpoint is based on HEAD `5dc49bd24d833188719a9b9fb94bb4981bf00f71`; publication does not close B0/SEC0 or grant production approval. Full role runtimes and standalone copy/build/run remain incomplete.

- Rust 1.97.1; Cargo.lock SHA-256 `a23eec032e81e1bbbbcc5b164e5f1575132d558fa8db0b9aa7743bae49906ed1`; reviewed structure policy SHA-256 `2145a81e11db6f1d7ba7cb91dd66bc5da1c37fbab7c7e719797f163ba44fbf9a`.
- Windows GNU `cargo +1.97.1 test -p eve-crypto -p eve-evm --locked`: exit 0; 29 authentication and 8 EVM integration tests pass, zero failed/ignored. The 15 official ML-DSA-65 cases and both-direction cross-implementation checks run. EVM tests cover exact fee allocation, typed envelopes, deterministic execution, revert/out-of-gas, deployment and invalid-block atomicity. Test-only signing fixtures are disposable and are not runtime defaults.
- Linux/WSL `cargo test -p eve-storage --locked`: exit 0; 12 integration tests pass, zero failed/ignored. Tests cover bounded storage, durable/idempotent records, snapshot/checkpoint consistency, namespace/encoding validation and abrupt process-exit recovery. This is not power-loss certification or authenticated distributed recovery.
- `cargo +1.97.1 fmt --all -- --check`; Windows GNU `cargo +1.97.1 clippy -p xtask -p eve-crypto -p eve-evm --all-targets --locked -- -D warnings`; Linux/WSL `cargo clippy -p eve-storage --all-targets --locked -- -D warnings`: exit 0. Test helpers were separated by use and needless lifetimes/clones corrected without disabling lints or assertions.
- Windows GNU `cargo +1.97.1 build -p xtask -p eve-crypto -p eve-evm --release --locked`; Linux/WSL `cargo build -p eve-storage --release --locked`: exit 0. Linux native storage uses clang-19/libclang-19, bundled RocksDB 11.8.1, three build jobs and the task-owned target cache.
- `cargo +1.97.1 run --locked -p xtask -- check-structure --report local-tests/publish-structure.json`: exit 0, current scan has zero violations and three exact generated/vendor exclusions. Mandatory T-L01–T-L06 boundary/CI/package acceptance remains unfinished; this scan is not full B0 acceptance.
- `cargo-audit audit --file Cargo.lock --json`: exit 0; zero known vulnerabilities, five unmaintained warnings: derivative 2.2.0 (RUSTSEC-2024-0388), paste 1.0.15 (RUSTSEC-2024-0436), pqcrypto-internals 0.2.11 (RUSTSEC-2026-0163), pqcrypto-mldsa 0.1.2 (RUSTSEC-2026-0166), pqcrypto-traits 0.3.5 (RUSTSEC-2026-0162). The PQClean wrappers are dev-only interoperability checks; a maintained reviewed cross-verification path and dependency review remain open before full SEC0/production acceptance. No warning was suppressed.
- Bounded publication review found no production secrets or prohibited local/build/database/environment paths. NIST source/derived fixture identities and full notices were checked. This is not a complete security audit or language certification. Raw logs, local reports and build outputs stay ignored; all first-party published prose is English.
- Source/fixture, secret-exposure, whitespace, final index and outgoing-history checks precede the authorized non-force push. No PR merge, production deployment, funds, paid infrastructure or release approval is part of publication. Full consensus/interop/security/capacity evidence remains open.

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
