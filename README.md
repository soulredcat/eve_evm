<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# EVE EVM

An engineering project for an EVM-compatible network with separate public access, validator execution/consensus, and developer-operated durable synchronization infrastructure.

**Status: the published checkpoint `c2f9b2a` passes the complete local B3 gate
with 531 tests and zero failed/ignored/pending cases. Hosted B3 acceptance remains
unresolved: one failed run reports `ENGINE_PROCESS_INODE_MISMATCH` during native
engine startup. Repair this prerequisite before B4. Standalone distributions,
activated PQ, security-profile acceptance and measured 1M TPS remain unachieved.**

Start with the [shared documentation](docs/README.md),
[contribution contract](docs/development/README.md) and
[verified execution status](docs/execution/README.md). These are available to
GitHub-only collaborators; private local plans are not needed to read this context.

## Main objective

Build a correct, recoverable, independently verifiable network and pursue **1,000,000 aggregate finalized transactions per second**, using explicitly declared workloads and hardware. This target is not achieved by receiving requests, adding replicas, or publishing state hashes alone.

## Production authority

> Validators decide. Master remembers.

```text
Users / dApps
       |
Public nodes: RPC, transaction ingress, P2P, verified state
       |
Validator network: execute, propose, validate, vote, finalize
       |
Finalized blocks + authenticated state commitments
       +----------------------+
       |                      |
Public / validator peers      Master replicas
recent durable data           sync, durable state, snapshots, recovery
```

Master acknowledgements are not required for ordinary transaction finality. Public nodes are permissionless, with no fixed protocol-wide count; active validator membership and voting power follow staking rules. Master infrastructure is operated by developers, but its signatures do not create consensus authority.

A validator needs durable signing-safety and recovery records even when its working state is in RAM. Losing validator quorum stops new finality; there is no automatic master takeover.

Default public nodes keep active verified state in RAM and finalized recovery blocks/checkpoints durable through a separate bounded storage worker. Execution/current-state reads do not hold state locks while waiting for disk; applied and durable heights remain distinct. Storage still consumes resources, so latency interference and lag must be measured. Public restart recovery must work from retained verified data while masters are offline.

One regional master can serve several public nodes through a preferred nearby eligible sync endpoint with fallback peers. Public does not require private master topology or database access. The planned evolution is two masters synchronizing independently verified finalized history, then ten regional masters; this is not launch authorization or a throughput claim. `zone_id` affects routing/placement only. See the [public architecture contract](docs/architecture/README.md).

## Source layout

- `master/`: protected synchronization and storage runtime.
- `public/`: independently distributable public RPC/P2P runtime.
- `validator/`: validator runtime; it may be co-located with a public node.
- `public/components/recovery-store/`: bounded durable recovery record storage owned by the public role.
- `public/components/node-policy/`: validated public resource, watermark, readiness and source-selection policy.
- `validator/components/execution/`: deterministic EVM execution owned by the validator role.
- `validator/components/authentication/`: reviewed authentication integration owned by the validator role.
- `validator/components/consensus-comet/`: pinned engine API, framing, height binding and unsupported hybrid guard.
- `validator/components/protocol-config/`: development genesis, native ABI/gas and canonical record/header contracts.
- `validator/components/state/`: complete logical state, canonical roots, journals and structural recovery commits.
- `xtask/`: structure, ownership, verification, evidence and pinned local tools.
- `docs/`: shared public architecture, development, roadmap and execution READMEs.
- `docs/plan/`: public roadmap and local-only detailed specifications.
- `docs/agents/`: role-specific execution responsibilities.
- `docs/execution/`: public verified status and local-only detailed evidence/handoff.
- `tests/`: reviewed reproducible test source and sanitized regression fixtures.
- `tests/acceptance/state-recovery/`: independent B1 root, execution, durable, corruption, snapshot and process-recovery acceptance.
- `tests/acceptance/serial-rpc/`: independent B2 EVM corpus, RPC/contract/client, limits and process-restart acceptance.
- `local-tests/`: ignored machine-local experiments and test output; never published.

The validator development entry point is implemented. Complete public/follower integration and independent copy/build/run acceptance remain unfinished. Reference tools and component interfaces are pinned; [execution status](docs/execution/README.md) records current results and limitations. Follow the [development instructions](docs/development/README.md) to provision tools and run a registered gate. These foundations are not a promise of 1M TPS. The execution component targets Shanghai; subsequent fork support is an explicit upgrade.

## Economics

Collected transaction fees: **40% burn / 30% node rewards / 30% validator rewards**. Integer rounding, uptime, verified work, staking, escrow and slashing are specified in the plan. No additional inflation or real-value genesis allocation is authorized by these documents. Fee redistribution differs from Ethereum's base-fee burn policy and must be advertised as an EVE difference.

## Current work boundary

The latest owner-authorized checkpoint includes
[PR #3](https://github.com/soulredcat/eve_evm/pull/3), observed merged/closed.
Its complete local gate passes 531 cases; hosted verification remains unresolved.
Signing refusals return a native error without releasing a signature; durability,
fencing and executable authentication remain required.
The owner resumed dependency-ordered core work on 2026-10-02 and now permits
subagents again. Assign file ownership before parallel work and protect frozen gates.
Resolve the hosted B3 failure before advancing to B4; keep the failed evidence visible.

Public contributor context is indexed in [docs](docs/README.md), including the
roadmap, architecture, development rules, verified status and security inventory.
Read each affected component README. Detailed owner instructions, plans and raw
evidence remain local; their absence from a clone does not grant permission to
invent requirements or claim missing acceptance.

To reproduce verification, provision the exact reference tools with
`cargo xtask provision-tools --jobs 2`, then run `cargo xtask verify --bulk B3`
with the pinned Linux environment. A failed or skipped mandatory check is not
acceptance. Tool source/artifact pins and required cases are in `config/`; this
verification must use the registered complete gate and preserve failed evidence.
Use the receipt environment exported in the [CI workflow](.github/workflows/foundation.yml).
Raw output and detailed execution records stay local-only. The exact tested
revision, frozen source identity and local/hosted outcomes are recorded together
in [execution status](docs/execution/README.md). Documentation updates do not
change or extend the recorded runtime test result.

## Collaboration and publication

Read the [public contribution contract](docs/development/README.md) before making changes. Shared source comments, documentation, configuration explanations, test descriptions, commit messages, and GitHub discussions must use English. CONTRIBUTING.md remains a local-only owner document.

Keep GitHub clean: `local-tests/`, raw artifacts, build output, temporary databases, secrets, and personal configuration must never be committed or pushed. Required reproducible tests remain versioned so every collaborator can verify the implementation. Inspect the index and all outgoing commits before an authorized push; never force-add ignored local files.

Only Markdown files named `README.md` may be tracked. `AGENTS.md`, `goal.md`,
`CONTRIBUTING.md` and every other `.md` document are local-only and ignored.
Ignoring does not remove already tracked files; CI rejects forbidden tracked
Markdown. This controls the current tree, not historical commits. Do not force-add
private documents. Required test/gate inventory remains public in allowed READMEs
and existing source/configuration. Complete third-party notices remain intact in
extensionless legal files; no usage permission or upstream right is removed.

## Scope boundaries

EVE develops its own chain first. D40 defers bridges and external-chain adapters until EVE testnet, then separate programs adapt to EVE. Their latency, confirmations and backlog are outside core execution/voting/publication and the secured 1M target. Local EVM standards and correctness fixtures remain. Real keys/funds, mainnet, custody and paid infrastructure need separate approval.

Copyright (c) 2026 Redcat, all rights reserved. Use requires prior written permission under [LICENSE](LICENSE); every first-party file carries a notice or exact [REUSE association](REUSE.toml). Upstream rights and mandatory law/platform terms remain preserved.
