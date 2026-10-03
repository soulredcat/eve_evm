<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Testing and acceptance

Required reproducible tests and sanitized fixtures are versioned with the source.
Read the [development contract](../development/README.md), the affected component
README and [execution status](../execution/README.md) before choosing a test scope.
Exploratory scripts and raw output remain local; a collaborator does not need
private owner documents to use the shared source and registered gate inputs.

## Published test ownership

| Location | Required responsibility |
|---|---|
| Runtime/component test modules | Operation regressions, invariants, negative inputs and resource bounds |
| [State recovery acceptance](../../tests/acceptance/state-recovery/README.md) | Real canonical execution, durable identity, corruption, snapshots and process recovery |
| [Serial RPC acceptance](../../tests/acceptance/serial-rpc/README.md) | Shanghai corpus, signed RPC/client/contract flows and restart/resource behavior |
| [Validator consensus acceptance](../../tests/acceptance/validator-consensus/README.md) | Actual independent validator/engine processes, quorum, partitions, signing recovery and historical commitments |
| `xtask/tests/` | Structure, ownership, exact discovery, source identity and evidence integrity |
| `config/gates/` | Implemented bulk selection, exact test catalogs, prerequisites and missing requirement status |

Public owns access/persistence tests; validator owns execution, signing and finality
tests; master owns follower/archive tests. Reuse canonical components. A master
or source response cannot replace validator finality or execution validation.

## Run registered verification

Use Linux x86_64 and the exact pins in `rust-toolchain.toml`,
`config/ci-environment.toml`, `config/tool-pins.toml` and the
[workflow](../../.github/workflows/foundation.yml). Provision and revalidate the
task-local tools; use the receipt environment as exported by that workflow.

```sh
cargo xtask provision-tools --jobs 2
cargo xtask check-structure --report local-tests/structure.json
cargo xtask check-ownership --report local-tests/ownership.json
cargo xtask verify --bulk B3
```

The full gate checks format, strict Clippy, registered discovery/execution,
structure, ownership and release builds. A focused test is useful diagnosis;
it does not replace the full gate. Missing, duplicate, unregistered, skipped,
ignored, filtered or zero requested cases fail complete acceptance. Commands
for later devnet, packaging or benchmarking work must remain labelled
`NOT_IMPLEMENTED` until their actual CLI and required coverage exist.

Test one stable source snapshot. Record its commit, dirty state, source/config
digests, tools, profile, topology, exact commands, exits, counts and limitations.
Do not overwrite an authenticated executable while its process is running.
Independent work can continue in a separate checkout and target directory.

## Required assertions and fault scope

Preserve deterministic EVM semantics, serial/replay equivalence, atomic state
transitions, receipts, fee conservation and the 40/30/30 split. Validator tests
must retain execution-before-vote, exact historical sets, strict weighted quorum,
durable anti-double-sign state and H/H+1 application anchoring. More public nodes
never grant voting power; master availability never chooses finality.

Recovery tests must distinguish applied, durable and authenticated heights,
exercise bounded storage stalls and unavailable history, and preserve the last
recoverable finalized copy. A process SIGKILL is process-crash evidence; it
does not prove physical power-loss behavior. Same-host logical peers do not
prove independent machine or regional failure domains.

Classical development tests do not establish PQ security or unconditional
majority-attack immunity. Preserve mandatory SEC0/SEC1/SEC3 and T-M/T-P coverage;
deferred bridges/external programs remain outside current core acceptance.

## Share evidence without raw artifacts

Publish compact reviewed summaries in [execution status](../execution/README.md):
tested SHA/source identity, commands, counts, outcome, CI links and residual gaps.
Label local-only report references explicitly. Keep raw stdout/stderr, development
keys, databases, generated builds/dependencies and machine configuration ignored.
Never force-add them. Only `README.md` Markdown files may be tracked.

A bulk is complete only after implementation, required positive/negative tests,
integration, documentation and relevant gates pass at the reviewed source. A
passing local run and a hosted run are separate facts; publish both accurately.
