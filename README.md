# EVE EVM

EVE EVM is a research and engineering project for a high-throughput EVM-compatible blockchain with separated public access, validator consensus, execution, persistence, and regional synchronization layers.

> Status: **Planning / pre-implementation**. No production implementation exists yet. Performance targets are engineering goals until demonstrated by reproducible end-to-end benchmarks.

## Core goals

- Preserve a familiar EVM developer experience: Solidity, standard transaction semantics, Ethereum-style JSON-RPC, and common tooling.
- Keep Internet-facing infrastructure separate from protected durable storage.
- Make validators responsible for consensus decisions and finality.
- Keep the Master focused on synchronized finalized state, persistence, recovery, snapshots, and archive duties.
- Keep hot/current state in memory where useful while maintaining durable recovery state on fast NVMe storage.
- Support permissionless Public Nodes and separately managed Validator Nodes.
- Support staking, measurable uptime/work accounting, and protocol-level fee distribution.
- Support regional execution and bulk state synchronization without placing WAN latency on every user transaction.
- Design for horizontal scaling and parallel execution rather than assuming a single machine can reach the long-term throughput target.
- Treat the **1,000,000 TPS** objective as an aggregate engineering target that must be proven under explicitly defined workloads.

## Architecture direction

```text
Users / dApps
      |
Edge / Load Balancer
      |
Public Nodes (RPC + P2P + RAM state)
      |
Validator Nodes (execution + proposal + vote + finality)
      |
FINALIZED blocks / state transitions
      |
Master Layer (sync + durable state + snapshots + archive)
      |
RAM hot state + durable NVMe storage
```

The Master does not decide production finality. Validators decide; the Master persists and redistributes finalized state.

During early development, a `MASTER_ONLY` prototype may temporarily include local execution/block production so the storage and EVM core can be built and tested before distributed consensus exists. That prototype authority is not the intended production trust model.

## Repository direction

```text
master/      # protected durable sync/storage runtime
public/      # permissionless RPC/P2P/state replica runtime
validator/   # consensus/execution/finality runtime
crates/      # shared protocol, primitives, EVM, crypto, networking
docs/        # architecture and implementation plans
```

## Economic direction

Initial fee-allocation proposal:

- 40% burn
- 30% node reward pool
- 30% validator reward pool

Reward eligibility must depend on protocol-verifiable work and availability rather than self-reported activity. Final economics are not frozen.

## Design principles

1. **Correctness before throughput.**
2. **Validators decide; Master remembers.**
3. **Canonical durable state must survive loss of Public Nodes.**
4. **RAM is an acceleration layer, not the only durable copy.**
5. **Public/Validator nodes must not have direct database/filesystem access to Master storage.**
6. **Consensus/finality, execution, state replication, persistence, and public RPC are separate responsibilities.**
7. **Regional latency should affect finality/synchronization, not every local transaction where avoidable.**
8. **No TPS claim is accepted without a published workload, hardware profile, sustained duration, state-growth measurement, and correctness checks.**
9. **EVM compatibility is a contract with developers and must be versioned/tested explicitly.**

## Planning documents

Implementation work is organized under [docs/plan](docs/plan/README.md).

## Current phase

The repository is intentionally documentation-first. The next step is to lock protocol invariants, failure semantics, state format, block format, validator finality rules, and benchmark methodology before writing the production runtime.
