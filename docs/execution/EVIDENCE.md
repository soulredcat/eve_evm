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
