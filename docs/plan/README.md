# Planning index

Status: **specification ready for implementation; runtime work not started**. These documents describe requirements, not evidence of execution.

Start with [goal.md](../../goal.md), [AGENTS.md](../../AGENTS.md), [current decisions](24-decision-register.md), [bulk backlog](23-task-backlog-and-execution.md), and [actual status](../execution/STATUS.md).

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
| [23](23-task-backlog-and-execution.md) | B0–B11 executable dependency-aware bulks |
| [24](24-decision-register.md) | Accepted direction, dev defaults and owner gates |

## Execution support

- [Specialist responsibilities](../agents/README.md).
- [Status](../execution/STATUS.md), [evidence index](../execution/EVIDENCE.md), [handoff](../execution/HANDOFF.md).
- [Primary references](../references.md) and [documentation change record](../CHANGELOG.md).

Specs 12–24 resolve earlier exploratory suggestions. A changed decision requires updating affected specs, tests and this index in the same bulk. No file may silently restore master authority, mandatory master round trips, RAM-only validator signing, or claimed linear TPS scaling from additional replicas.
