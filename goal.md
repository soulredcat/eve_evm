<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# EVE EVM — implementation goal

## Objective

Turn this documentation-only repository into a working, tested EVM-compatible network with separate `master/`, `public/` and `validator/` runtimes, then prove **1,000,000 aggregate finalized TPS as a mainnet release requirement**. Preserve a versioned, measured development path toward future 100M and potentially 1B TPS as technology advances; these future targets are not current capability claims. Do not stop after producing another plan, scaffold, mocked RPC response, in-memory demo, or compiling workspace.

User decisions are authoritative: validators make consensus decisions; developer-operated masters synchronize and persist finalized results; public nodes are permissionless; runtime sources are separate; fees split 40% burn, 30% node rewards and 30% validator rewards; useful work and availability determine reward eligibility. Organize implementation in meaningful recursively nested folders, with one primary function per behavioral file and the 200/400/600 physical-line policy in plan 25.

Follow plan 32 for the regional topology and public persistence: start with one master, one public node and the existing validator devnet baseline; prepare independently verified two-master replication and a planned ten-master regional layout. Public nodes use a preferred nearby eligible sync endpoint without private master inventory, retain fallback P2P sources, and continue without master acknowledgements. Default public working state is RAM-first with finalized blocks/recoverable checkpoints persisted by a separate bounded worker; resource interference must be measured, not declared zero. Zone IDs are routing metadata, not sharding or finality authority.

The owner additionally requires majority-attack resilience and post-quantum security for EVE itself. Implement the explicit fault models, authenticated security profiles and core acceptance in plans 26/27/29. Do not claim unlimited 51% immunity or end-to-end quantum security from the classical baseline.

The latest owner decision D40 defers bridge and external-chain programs until EVE reaches testnet. Future separate programs adapt to EVE's versioned public interfaces; the core does not adapt to remote consensus, asset rules or speeds. External polling, confirmations, relays and bridge queues cannot enter EVE's transaction, voting, execution or durable-acknowledgement path. Preserve generic own-chain headers, receipts, finality and proof capabilities without premature external route implementations.

## Current scope boundary

Follow plan 31 and D40. Liquidity, production DEX/reserve trading, stabilization, bridges and external-chain adapters are separate deferred work. Core acceptance and its 1M target do not wait for those programs. Keep Solidity/AMM fixtures, Ethereum-style RPC and Shanghai reference vectors as local EVM correctness/performance tests; they introduce no external-chain runtime dependency. The reserve/multi-admin proposal does not authorize trading, issuance or withdrawal.

## Authority and reading order

1. Current explicit user instructions and repository safety boundaries.
2. `docs/plan/24-decision-register.md`, `docs/plan/31-mainnet-target-and-module-boundaries.md`, and `docs/plan/32-regional-masters-and-public-persistence.md`: current architectural decisions, release target, scope, regional topology/public persistence, and superseded assumptions.
3. Specifications 12–22, policies 25–28, and plan 30, with their acceptance tests. Plans 26–28 refine the older classical development baseline; they do not retroactively make it secure.
4. `docs/plan/23-task-backlog-and-execution.md` and core plan 29: mandatory B0–B11 and SEC0/SEC1/SEC3. Plans 28/30 and SEC2/INT0–INT3 are deferred program references under D40.
5. Overviews 00–11, which summarize rather than override those specifications.
6. `docs/execution/STATUS.md`, `EVIDENCE.md`, and `HANDOFF.md`: actual progress.

For an inconsistency, preserve safety and the latest architectural decision, add a decision record, repair affected documents/tests together, then continue. Do not silently broaden scope, reintroduce master-controlled finality or add the deferred liquidity module.

## Execution contract

Work autonomously through every ready B0–B11 and SEC0/SEC1/SEC3 bulk using `AGENTS.md`. SEC0 belongs to B0. SEC2/INT0–INT3 are DEFERRED_UNTIL_EVE_TESTNET, outside the mandatory core registry; historical executed INT0 results stay recorded. Plan by dependencies and implement complete vertical slices. Verify unfamiliar APIs with primary references and bounded spikes, then implement rather than repeatedly rewriting plans.

Use five specialist responsibilities when supported: lead/orchestrator; protocol/EVM; state/network/performance; correctness/security; reviewer/integrator. Assign file ownership before parallel work. Tests and review belong to every bulk. Do not create external-chain adapter programs or custody scaffolds before the deferred testnet phase is separately scoped.

Each bulk must leave a usable repository state. Run its gates, repair failures, save evidence, update status, create a coherent local commit, and proceed. For a genuine external blocker, record the failed command, cause, required resource and resume action, and work on other ready tasks. Lack of certainty is a reason for a test or source check, not for inventing a guarantee.

Follow the absolute publication policy in `CONTRIBUTING.md` and `AGENTS.md`: shared first-party prose and GitHub collaboration use English; root `local-tests/` and raw local artifacts must never be committed or pushed. Required reproducible tests and sanitized fixtures remain versioned. Inspect the index and every outgoing commit before an authorized push, and leave the task-owned checkout/worktree clean after each coherent commit while preserving unrelated user edits.

## Mandatory code organization

Use `runtime-or-crate/src/domain/capability/sub-capability/operation/.../function_name.rs`. Nest as deeply as the actual responsibility requires; neither stop at three levels nor create empty directories merely for depth. One production behavioral file owns one primary function. Extract other operations/helpers into correctly named files; keep types, facades and thin delegation adapters in the explicit categories of plan 25.

Target at most 200 formatted physical lines per file; 201–400 requires decomposition review and a retained-size rationale; 401–600 requires a reviewed exact-path temporary exception with an expiring split task; more than 600 is a hard failure for handwritten files. Line counts include imports, comments, blank lines and tests. No minification, blanket exclusions or lowered acceptance criteria.

B0 implements `cargo xtask check-structure` and T-L01–T-L06; every bulk also checks Redcat ownership notices with `cargo xtask check-ownership`. Preserve deterministic behavior, English publication, per-file permission-only notices and package independence. Fixture Solidity/TypeScript receives explicit structure coverage plus actual pinned compiler validation; future external programs have their own gates.

## Required software outcomes

The implementation must provide:

- A pinned, reproducible Rust workspace and executable development workflow.
- Deterministic REVM execution, signed transaction validation, real gas accounting, receipts, logs and authenticated state roots.
- Persistent state, crash recovery and restart-safe validator signing records.
- RAM-first public state with durable finalized blocks/checkpoints, isolated storage workers, bounded resource/lag budgets, distinct applied/durable/authenticated heights, and verified restart recovery with masters offline.
- Public Ethereum-style JSON-RPC and P2P ingestion with bounded queues, rate limits and documented compatibility differences.
- Four-validator development consensus, proposer rotation, independently verified proposals, quorum finality, validator-set transitions and no master override.
- A protected master follower that can stop, return and catch up without deciding finality or being acknowledged on the transaction hot path.
- Verified snapshot/delta bootstrap from multiple sources, corruption rejection, restart recovery, pruning safety and retained data during master outages.
- Staking, delegation, node work accounting, validator participation, the 40/30/30 fee ledger, slashing evidence, jailing and unbonding with supply-conservation tests.
- Separate public/validator packages that build without master code; signed release verification and operator-controlled rolling updates.
- Deterministic parallel execution proven equivalent to the serial oracle, including hot-pool contention and atomic multi-contract transactions.
- Master replication/failover, partition tests, multi-region single-chain deployment and an explicit experimental path for state partitioning.
- Preferred eligible regional sync endpoints with bounded discovery/failover, independent authenticated master-to-master replication, and zone IDs that do not change network identity or consensus authority.
- Reproducible workload generators, telemetry, fault injection, capacity reports and measured release-candidate evidence for the 1M target.
- Function-focused, recursively nested implementation modules, passing structural checks and no expired size exceptions.
- Majority-adversary tests and honest fault-assumption reporting, without automatic quorum reduction or master takeover.
- Actual mandatory hybrid/PQ authentication through consensus, protected accounts, clients, control/recovery and release paths; crypto/commitment inventory and tested migrations.

A master-only early prototype composes the same shared execution/storage components with a local test producer. It must not become a second production consensus implementation. A production-mode master with all validators offline preserves data and reports stalled finality; it does not invent blocks.

## Completion levels

### DEVNET_ACCEPTED

All B0–B9 mandatory acceptance checks pass. A clean checkout can build and start a local network, deploy a Solidity fixture, send native/ERC-20 transfers, execute successful and reverting swaps, read accurate receipts/logs, restart every role, lose one validator, lose the master, recover a public replica, settle rewards, and verify unchanged roots and balances. Packaging, structure, security, upgrade and chaos tests must also pass. Disclose whether this checkpoint is CLASSICAL_DEV or a stronger profile. It is not completion of security, external interoperability or throughput targets, and is not permission to launch mainnet. Test-fixture swaps do not require implementing the deferred production liquidity module.

### SECURITY_PROFILE_ACCEPTED

SEC0/SEC1/SEC3 and T-M/T-P acceptance pass for the declared EVE fault model and protected authentication scope. Publish core crypto coverage, residual classical dependencies and migration/incident evidence. Classical-only consensus, post-hoc PQ stamps or unprotected account recovery cannot satisfy this level. Deferred bridge security cannot substitute for core verification or block the core gate.

CometBFT's baseline does not guarantee safety/liveness under 51% Byzantine power. Any stronger continuity requirement remains `UNSATISFIED_BY_BASELINE` until a reviewed alternative with explicit assumptions meets it. Passing an attack test does not prove universal immunity. An external bridge route remains disabled until its own adapter and authorization gates pass.

### INTEROP_DEV_ACCEPTED

DEFERRED_UNTIL_EVE_TESTNET. SEC2/INT0–INT3 and T-BR/T-I belong to future separate programs under plans 28/30. They are neither achieved nor mandatory core prerequisites. Any later scope must independently define route trust, verification, accounting, client/recovery and incident acceptance; a simulator cannot supply source finality.

This deferred checkpoint approves no live route or external-chain security claim. Future adapter backlog, latency and failure remain isolated from EVE's consensus and finalized throughput.

### SCALE_TARGET_VERIFIED

B10–B11 evidence proves sustained 1M EVE finalized TPS under plans 08/20/21/27/29/31/32. Include the active security profile, workload, latency, cross-node/storage behavior, bounded backlog, recovery and independent replay. Transfer, mixed-contract, hot-pool and atomic cross-domain workloads stay mandatory. Worker sums, roots, ingress, classical-only benchmarks and bursts do not prove the secured target. Future external programs have separate measurements and cannot cap EVE's core acceptance.

For mainnet readiness, evidence must cover the release candidate and production-representative configuration/topology. Do not describe a prelaunch capacity benchmark as an already-observed live-mainnet load. Future 100M/1B goals require separate measured evidence; preserve extensibility without claiming those capacities or expanding the first-launch threshold beyond 1M.

If software runs but measurements fall short, status is TARGET_UNMET and the next optimization remains work. If suitable hardware is unavailable, status is BLOCKED_INFRA for the affected experiment. Neither state may be renamed DONE. Continue all implementable work and produce the exact deploy/run package needed to resume.

### MAINNET_READY

Requires SCALE_TARGET_VERIFIED at 1M, owner-approved genesis/economics/governance, independent core security review, operations/key ceremony, permitted distribution and production infrastructure. This goal authorizes no deployment or spending. Code completion does not certify economic/PQ security, majority immunity or audit status. Deferred external, bridge and liquidity programs are not core mainnet prerequisites.

## Definition of done per requirement

A requirement is DONE only with real implementation, positive and negative tests, integration with adjacent modules, updated operator/developer documentation, passing relevant gates including structure checks, and recorded evidence at the reviewed commit. Stubs, TODO bodies, unconditional success, fake signatures, hard-coded roots, ignored recovery errors and disabled assertions do not qualify.

`docs/execution/STATUS.md` tracks core/security bulks, deferred scope and unmet stronger targets. `EVIDENCE.md` links actual reports; raw logs stay ignored. `HANDOFF.md` records the exact next action without requiring this conversation.

## Permission and resource boundaries

Operate on this repository and task-owned development resources with fake tokens. Preserve unrelated wallets/keys/data/bots. No paid infrastructure, visibility change, additional license grant beyond the authorized Redcat permission-only policy, real signing/funds, mainnet/live custody or reserve spending is authorized. External programs require a later scoped instruction after testnet.

Session/tool limits must produce a resumable checkpoint, not a promise of unattended future work. A goal file guides an available Codex session; it does not remove runtime, quota, permission or hardware limits.

## First actions

The owner's latest 2026-10-01 instruction pauses development during incomplete B3 because the PC runs other programs. Publish the current-state checkpoint on its branch and create a draft PR to main without merging, then stop. Do not resume development, heavy checks, B4 or another security bulk until new owner input. Preserve the failed complete gate and all remaining requirements for later work. This takes precedence over automatic advancement; it does not mark B3, testnet, security or capacity accepted.
