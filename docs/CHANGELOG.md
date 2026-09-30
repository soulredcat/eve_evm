# Documentation change record

## 2026-09-30 — Local test isolation and English collaboration

- Created ignored root `local-tests/` for machine-local exploratory tests and raw output; its contents must never be committed or pushed.
- Added `.gitignore` for local tests, raw artifacts, coverage, generated output, and personal environment files while retaining sanitized environment examples.
- Added `CONTRIBUTING.md` and absolute publication/language rules in `AGENTS.md`, the goal and decision register.
- Preserved shared reproducible tests and sanitized fixtures in tracked test areas for collaborators.
- Updated status and handoff without marking any implementation bulk or runtime gate complete. No GitHub push is authorized or performed by this task.

## 2026-09-30 — Goal-driven implementation package

Starting from documentation baseline `1d8b73c7eec223a1460c9e09cfb2d18a5627dee6`:

- Added root goal.md and AGENTS.md with dependency-aware bulk execution, evidence gates and resumption rules.
- Reconciled README and plans 00–11 with validator-owned finality and master follower-only production behavior.
- Added plans 12–24 covering protocol, state, networking, economics, RPC, security, acceptance, capacity, layout, backlog and decisions.
- Added master/public planning roots and updated validator responsibilities.
- Added five specialist role instructions, progress/evidence/handoff documents and primary references.
- Added tests/benchmarks planning entry points, not fictitious implementation or results.

Important corrections: no master emergency finality, no RAM-only validator signing, no linear TPS claim from adding replicas, no unauthenticated delta/root acceptance, and no conflation of fast snapshot import with independent execution proof. The ABCI next-height application-hash binding and EVE-specific fee redistribution are explicit.

All runtime/build/test/benchmark statuses remain NOT_STARTED or NOT_RUN. Mainnet, real funds, licenses/visibility changes and paid infrastructure remain outside this documentation update's authorization.
