# Planning index

Status: **specifications prepared; B0/SEC0 foundation implementation in progress**. Role-owned component source and an initial structure checker exist; complete runtimes, standalone packages and full acceptance are unfinished. Specifications remain requirements, not proof of route availability, security certification or target throughput.

Start with [goal.md](../../goal.md), [AGENTS.md](../../AGENTS.md), [current decisions](24-decision-register.md), [mainnet target and module boundaries](31-mainnet-target-and-module-boundaries.md), [regional masters and public persistence](32-regional-masters-and-public-persistence.md), [folder/function policy](25-folder-hierarchy-and-file-function-policy.md), [core backlog](23-task-backlog-and-execution.md), [security backlog](29-security-implementation-and-acceptance.md), [interoperability queue](30-cross-chain-interoperability.md), and [actual status](../execution/STATUS.md).

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
| [28](28-bridge-security-and-finality.md) | Route trust, source proofs, custody/backing, limits and local bridge acceptance |
| [29](29-security-implementation-and-acceptance.md) | Mandatory SEC0–SEC3 queue, core dependencies and security completion levels |
| [30](30-cross-chain-interoperability.md) | Ethereum/Solana two-way targets, extensible adapters, wallet/SDK, assets, INT0–INT3 and T-I gates |
| [31](31-mainnet-target-and-module-boundaries.md) | Proven 1M mainnet target, future 100M/1B path, liquidity deferral and separate reserve/admin proposal |
| [32](32-regional-masters-and-public-persistence.md) | Regional 1→2→10 master topology, nearest eligible sync endpoints, RAM-first public state with isolated durable persistence, recovery budgets and T-N09–T-N12 |

## Execution support

- [Specialist responsibilities](../agents/README.md).
- [Status](../execution/STATUS.md), [evidence index](../execution/EVIDENCE.md), [handoff](../execution/HANDOFF.md).
- [Primary references](../references.md) and [documentation change record](../CHANGELOG.md). Security and chain-adapter primary links are included directly in plans 26–28 and 30.

Specs 12–32 resolve earlier exploratory suggestions. Plans 26–29 add mandatory security requirements; the classical initial engine/accounts in older plans remain only a development baseline. They are not retroactively quantum secure or majority-attack immune. Plan 30 adds Ethereum and Solana as named implementation targets, not approved live routes; the two-EVE fixture alone cannot complete them.

Plan 31 records the latest scope: proven 1M finalized TPS is a mainnet release requirement; 100M/1B remain future evidence-driven goals. Production liquidity/DEX and price stabilization are a separate future module, not current core work. AMM/swap test fixtures remain mandatory where already specified, and the recorded reserve/multi-admin proposal does not authorize trading or funding.

Plan 32 records the owner-approved regional topology and public persistence model. Master followers remain off the transaction path; public working state stays in RAM while bounded storage workers retain durable recovery data. Nearby-source selection includes authentication, freshness, fallback and resource limits. Storage interference must be measured; zone IDs do not create sharding or finality authority. T-N09–T-N12 extend B4/B6/B8 acceptance and remain unimplemented.

A changed decision requires updating affected specs, tests and this index in the same bulk. No file may silently restore master authority, mandatory master round trips, RAM-only validator signing, linear TPS claims from extra replicas, monolithic files, classical-only security-profile bypasses, unverified bridge minting or automatic chain support from an RPC URL. Real bridge deployment remains an owner gate.
