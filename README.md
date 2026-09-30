# EVE EVM

EVE EVM is a research and engineering project for a high-throughput EVM-compatible blockchain with a separated public, validation, execution, persistence, and regional synchronization architecture.

> Status: **Planning / pre-implementation**. No production implementation exists yet. Performance targets are design goals until demonstrated by reproducible end-to-end benchmarks.

## Core goals

- Preserve a familiar EVM developer experience: Solidity, standard transaction semantics, Ethereum-style JSON-RPC, and common tooling.
- Separate public-facing infrastructure from protected canonical-state infrastructure.
- Keep hot/current state in memory where useful while maintaining durable recovery state on fast NVMe storage.
- Support validator participation, staking, measurable uptime/work accounting, and protocol-level fee distribution.
- Support regional execution and bulk state synchronization without placing WAN latency on every user transaction.
- Design for horizontal scaling and parallel execution rather than assuming a single machine can reach the long-term throughput target.
- Treat the **1,000,000 TPS** objective as an aggregate engineering target that must be proven under explicitly defined workloads.

## Initial architecture direction

```text
Users / dApps
      |
Edge / Load Balancer
      |
Public Nodes (RPC + Validator + RAM state)
      |
Internal authenticated protocol
      |
Regional / Canonical Master Layer
      |
EVM Execution + Block Building
      |
RAM hot state + durable NVMe state
      |
Snapshots / WAL / block segments
```

The master/canonical layer is not intended to expose public RPC or administration endpoints directly to the Internet. Public nodes form the externally reachable layer and independently verify protocol outputs.

## Economic direction

Initial fee-allocation proposal:

- 40% burn
- 30% node reward pool
- 30% validator reward pool

Reward eligibility must depend on protocol-verifiable work and availability rather than self-reported activity. Final economics are not frozen.

## Design principles

1. **Correctness before throughput.**
2. **Canonical state must survive loss of public nodes.**
3. **RAM is an acceleration layer, not the only durable copy.**
4. **Public nodes must not have direct database/filesystem access to canonical storage.**
5. **Consensus/finality, execution, state replication, and public RPC are separate responsibilities.**
6. **Regional latency should affect finality/synchronization, not every local transaction where avoidable.**
7. **No TPS claim is accepted without a published workload, hardware profile, sustained duration, state-growth measurement, and correctness checks.**
8. **EVM compatibility is a contract with developers and must be versioned/tested explicitly.**

## Planning documents

Implementation work is organized under [docs/plan](docs/plan/README.md).

## Current phase

The repository is intentionally documentation-first. The next step is to lock protocol invariants, failure semantics, state format, block format, and benchmark methodology before writing the production runtime.
