<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Planning index

Status: **historical B0 including SEC0/INT0 passed its complete local foundation gate**. Current D40 scope is EVE ownchain: B0–B11 and SEC0/SEC1/SEC3 remain mandatory; SEC2 and INT0–INT3 are `DEFERRED_UNTIL_EVE_TESTNET` for separately authorized programs. Complete runtimes, standalone packages, core security and capacity acceptance remain unfinished. Specifications and historical foundation tests do not prove route availability, security certification or target throughput.

Start with [goal.md](../../goal.md), [AGENTS.md](../../AGENTS.md), [current decisions](24-decision-register.md), [mainnet target and module boundaries](31-mainnet-target-and-module-boundaries.md), [regional masters and public persistence](32-regional-masters-and-public-persistence.md), [folder/function policy](25-folder-hierarchy-and-file-function-policy.md), [core backlog](23-task-backlog-and-execution.md), [core security backlog](29-security-implementation-and-acceptance.md), and [actual status](../execution/STATUS.md). Plans 28/30 are retained deferred program references, not active execution queues.

## Design overviews

| Plan | Subject |
|---|---|
| [00](00-vision-and-non-goals.md) | Vision, objectives, non-goals |
| [01](01-system-architecture.md) | Runtime roles and authority |
| [02](02-state-and-storage.md) | Persistence and RAM |
| [03](03-blocks-execution-and-parallelism.md) | Execution and block lifecycle |
| [04](04-master-public-node-security.md) | Trust boundaries |
| [05](05-validator-staking-and-economics.md) | Economic invariants |
| [06](06-regional-sync-and-finality.md) | Regions, replication and finality |
| [07](07-developer-compatibility.md) | EVM developer surface |
| [08](08-benchmark-and-acceptance.md) | Measurement policy |
| [09](09-delivery-roadmap.md) | Delivery order |
| [10](10-master-public-node-model.md) | Master/public/validator modes |
| [11](11-main-goals-and-planning.md) | Requirements and success levels |

## Implementation specifications

| Plan | Subject |
|---|---|
| [12](12-consensus-spec.md) | Consensus, quorum, locking, commitment binding |
| [13](13-transaction-and-gas-spec.md) | Transactions, EVM fork, gas and fees |
| [14](14-block-and-state-commitment-spec.md) | Headers, roots, serialization and persistence |
| [15](15-network-and-sync-protocol.md) | P2P, snapshots, delta sync and availability |
| [16](16-genesis-upgrade-and-recovery.md) | Genesis, activation, recovery and upgrades |
| [17](17-validator-lifecycle-and-rewards.md) | Registration, staking, work, rewards and penalties |
| [18](18-rpc-mempool-and-developer-experience.md) | RPC contracts, mempool and developer tests |
| [19](19-security-and-release-engineering.md) | Threat model, keys, packaging and release safety |
| [20](20-test-vectors-and-acceptance.md) | Test matrix, gates and evidence |
| [21](21-capacity-and-regional-scaling.md) | Capacity, WAN, parallelism and sharding gates |
| [22](22-code-layout-and-dependency-policy.md) | Modules, interfaces and dependency choices |
| [23](23-task-backlog-and-execution.md) | B0–B11 core dependency-aware bulks |
| [24](24-decision-register.md) | Accepted direction, dev defaults and owner gates |
| [25](25-folder-hierarchy-and-file-function-policy.md) | Recursive folders, one-function files, 200/400/600 limits and structure gates |
| [26](26-consensus-adversary-and-majority-resilience.md) | Majority adversaries, actual BFT limits, safety/availability and incident tests |
| [27](27-post-quantum-cryptography-and-migration.md) | Crypto inventory, hybrid/PQ authentication, accounts, commitments and migration |
| [28](28-bridge-security-and-finality.md) | Deferred separate-program bridge safety, proofs, backing and T-BR acceptance |
| [29](29-security-implementation-and-acceptance.md) | Mandatory SEC0/SEC1/SEC3 core queue; SEC2 deferral and security completion levels |
| [30](30-cross-chain-interoperability.md) | Deferred adapter programs, external candidates, wallet/SDK, INT0–INT3 and T-I references |
| [31](31-mainnet-target-and-module-boundaries.md) | EVE-ownchain/testnet adapter boundary, proven 1M mainnet target, future 100M/1B path and deferred application/reserve proposals |
| [32](32-regional-masters-and-public-persistence.md) | Regional 1→2→10 master topology, nearest eligible sync endpoints, RAM-first public state with isolated durable persistence, recovery budgets and T-N09–T-N12 |

## Execution support

- [Specialist responsibilities](../agents/README.md).
- [Status](../execution/STATUS.md), [evidence index](../execution/EVIDENCE.md), [handoff](../execution/HANDOFF.md).
- [Primary references](../references.md) and [documentation change record](../CHANGELOG.md). Security and chain-adapter primary links are included directly in plans 26–28 and 30.

Specs 12–32 resolve earlier exploratory suggestions. Plans 26–27 and SEC0/SEC1/SEC3 add mandatory core security requirements; the classical initial engine/accounts remain only a development baseline, without retroactive PQ or majority-attack immunity. D40 supersedes earlier mandatory bridge/interop scope. Plans 28/30 retain future safety requirements and historical candidates, including two-EVE bridge tests, outside current core acceptance. No live route or new adapter development is authorized.

Plan 31 records the latest scope: EVE ownchain first, external development only after EVE testnet plus separate owner scope, and proven secured 1M finalized TPS for mainnet release. External adapters conform to versioned public EVE proof/transaction interfaces without remote latency, block-production, voting, finality or durable-acknowledgement dependencies. Future 100M/1B goals, liquidity/DEX/stabilization and reserve/admin proposals remain separate. Shanghai/compiler/client/AMM correctness fixtures stay core; no funding or trading is authorized.

Plan 32 records the owner-approved regional topology and public persistence model. Master followers remain off the transaction path; public working state stays in RAM while bounded storage workers retain durable recovery data. Nearby-source selection includes authentication, freshness, fallback and resource limits. Storage interference must be measured; zone IDs do not create sharding or finality authority. T-N09–T-N12 extend B4/B6/B8 acceptance and remain unimplemented.

A changed decision requires updating affected specs, tests and this index in the same bulk. No file may silently restore master authority, mandatory master round trips, RAM-only validator signing, linear TPS claims from extra replicas, monolithic files, classical-only security-profile bypasses, unverified bridge minting or automatic chain support from an RPC URL. Real bridge deployment remains an owner gate.
