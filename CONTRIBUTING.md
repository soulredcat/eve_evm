# Contributing to EVE EVM

The repository contains specifications and a pinned Rust workspace with verified B0/SEC0/INT0 foundations and B1 complete-state, durable recovery and development composition. Complete node runtimes, standalone distributions and later acceptance gates remain unfinished. Start with `AGENTS.md`, `goal.md`, `docs/plan/README.md`, and `docs/execution/HANDOFF.md`.

## Required language

Use English for all first-party shared documentation, code comments, API descriptions, operator messages, configuration explanations, test descriptions, commit messages, pull requests, and issue text. Use descriptive English names where naming conventions allow. Preserve protocol identifiers, proper names, canonical test-vector bytes, and attributed upstream material without changing their meaning.

## Local tests and shared tests

| Location | Purpose | Git policy |
|---|---|---|
| `local-tests/` | Exploratory tests, debugging scripts, disposable data, and raw test output | Local only; never stage, commit, force-add, or push |
| `tests/` and runtime test modules | Required reproducible regression, security, integration, and acceptance test source and sanitized deterministic fixtures | Reviewed and versioned with the implementation |
| `artifacts/` and `coverage/` | Generated run output and coverage reports | Local only |
| `docs/execution/` | Compact reviewed status, evidence summaries, checksums, and reproduction instructions | Versioned; identify local-only evidence explicitly |

Create `local-tests/` on your machine when needed. Git does not distribute ignored directories. Never hide a mandatory test or its only required fixture in local storage. Do not blanket-ignore `tests/`, JSON fixtures, or dependency lockfiles.

## Absolute publication rules

GitHub must contain reviewed, coherent source and documentation in English. Do not publish local experiments, raw logs, build output, temporary databases, credentials, private keys, personal environment files, or machine-specific configuration. Keep local runtime data and generated credentials under ignored task-owned storage.

Never use `git add -f` to publish an ignored local file. Git ignore rules do not remove already tracked files and do not prevent a forced add; review the index and every outgoing commit before publication. If prohibited material is already tracked or committed, stop publication and report it. Preserve user work; do not rewrite history or delete data without authorization.

Before committing, inspect:

```sh
git status --short --ignored
git diff --check
git diff --cached --check
git diff --cached --name-only
git diff --cached
git ls-files -- local-tests/ artifacts/ coverage/
git check-ignore -v -- local-tests/README.md
```

The `git ls-files` command must produce no paths. Also inspect staged environment files and generated output outside those roots. Keep sanitized `.env.example` files and reproducibility inputs available to collaborators. Leave the task-owned checkout/worktree clean after committing. Preserve and report unrelated edits without staging or reverting them for cleanliness; isolate publication work when necessary.

Before an authorized push, inspect every commit and diff in the outgoing range against the actual destination ref. Removing a prohibited file in the last commit does not remove it from an earlier outgoing commit. Run the applicable implementation gates and review for secrets, local artifacts, and English prose. Report an unavailable scanner as `NOT_RUN`.

Passing a publication review does not grant push authorization. Push only when the owner's current instruction explicitly includes it. Do not force-push or bypass required checks.

## Implementation and evidence

Follow the dependency-aware B0–B11, SEC0–SEC3, and INT0–INT3 queues and the function-file policy in plan 25. Keep required tests with the changes they verify. Missing tests and unavailable gates remain visible; local experiments cannot replace shared acceptance evidence.

`cargo xtask check-structure` and required T-L01–T-L06 boundary/regression coverage are implemented. Complete local B0 and B1 gates pass; CI invokes the current B1 gate while hosted results are recorded separately in `docs/execution/`. Documentation, ignore checks and a clean Git index do not satisfy a runtime, security or capacity gate.
