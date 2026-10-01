<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Contributing to EVE EVM

The repository contains a pinned EVE workspace with verified foundations and complete-state/recovery development composition. Complete network roles, standalone distributions and later gates remain unfinished. D40 defers external-chain programs until EVE testnet. Start with `AGENTS.md`, `goal.md`, the plan index and execution handoff.

## Ownership and prior permission

First-party material is Copyright (c) 2026 Redcat, all rights reserved, under `LicenseRef-Redcat-Permission-Only`. Use, execution, copying, modification and distribution require prior written permission from Redcat, subject to applicable law, mandatory hosting terms and separately applicable licenses. Repository access does not grant additional permission. See [LICENSE](LICENSE) and [ownership rules](docs/development/ownership.md).

Every first-party file needs its valid inline notice or exact `REUSE.toml` association. Preserve upstream ownership/licenses and canonical fixture bytes. Run `cargo xtask check-ownership`; its required coverage does not replace structure, correctness or security gates. Contributions must respect the owner's permission and identify any separately licensed upstream material.

## Required language

The owner additionally requires that future GitHub publications, after the current
owner-paused B3 checkpoint PR and when development resumes, contain no `.md` file
except `README.md`. This checkpoint is explicitly exempt by the owner's timing
choice. Before the next publication, migrate other Markdown documents without
losing instructions/specifications/evidence, repair references/gate inputs and
implement the corresponding publication check. Migration is NOT_IMPLEMENTED now;
do not claim the current tree satisfies the future filename rule.

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

Follow B0–B11, core SEC0/SEC1/SEC3 and plan 25. D40 removes SEC2/INT0–INT3 from mandatory core work until a separately scoped post-testnet program. Preserve core tests and historical evidence; deferred coverage cannot be called passed. Local experiments cannot replace shared acceptance.

`cargo xtask check-structure` and required T-L01–T-L06 boundary/regression coverage are implemented. Complete local B0 and B1 gates pass; CI invokes the current B1 gate while hosted results are recorded separately in `docs/execution/`. Documentation, ignore checks and a clean Git index do not satisfy a runtime, security or capacity gate.
