# 09 — Delivery roadmap

The executable backlog and all gates are in [23](23-task-backlog-and-execution.md). This overview must not introduce an alternative stage numbering.

| Bulk | Deliverable |
|---|---|
| B0 | Dependency pins, protocol fixtures, build/task runner, evidence framework |
| B1 | Shared state/storage and master-only recovery harness |
| B2 | Serial EVM, signed transactions, real gas/fees, public RPC |
| B3 | Validator consensus, quorum, durable signing safety |
| B4 | Master follower, snapshot/delta sync, availability and recovery |
| B5 | Staking, work accounting, rewards and penalties |
| B6 | Public P2P, independent packages, bounded resource use |
| B7 | Deterministic parallel executor and differential tests |
| B8 | Master HA, upgrades, releases, security and operations |
| B9 | Multi-region single-chain behavior and accepted devnet |
| B10 | Sustained benchmark/capacity program and optimizations |
| B11 | Scaling experiments, gated partitioning and final target verification |

B0–B9 establish DEVNET_ACCEPTED; they do not establish 1M TPS or mainnet readiness. B10–B11 must keep the user's target explicit and measured. Complete every ready task; record genuine external blockers with exact resume instructions instead of stopping at the first unavailable resource.

The master-only harness builds shared components first, then production validators consume them. Never write a master consensus implementation that must later be copied into validators.

Production launch is a separate owner-approved gate. Implementation, testing, documentation and release artifacts can be prepared without launching a chain with real value.
