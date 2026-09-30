# EVE EVM

A documentation-first engineering project for an EVM-compatible network with separate public access, validator execution/consensus, and developer-operated durable synchronization infrastructure.

**Status: specification and implementation plan only. No runtime, passing test suite, deployed chain, or measured TPS is claimed.**

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

## Source layout

- `master/`: protected synchronization and storage runtime.
- `public/`: independently distributable public RPC/P2P runtime.
- `validator/`: validator runtime; it may be co-located with a public node.
- `crates/`: shared protocol, execution, commitments, storage interfaces and cryptography.
- `docs/plan/`: normative development specifications and dependency-ordered work.
- `docs/agents/`: role-specific execution responsibilities.
- `docs/execution/`: persistent progress, evidence and resumption state.

Directories currently contain planning material, not implemented binaries. Rust + REVM and a CometBFT consensus adapter are the proposed executable devnet baseline; dependency versions must be pinned and verified in bulk B0. They are not a promise of 1M TPS. The baseline preserves a declared Shanghai EVM execution surface; subsequent fork support is an explicit upgrade.

## Economics

Collected transaction fees: **40% burn / 30% node rewards / 30% validator rewards**. Integer rounding, uptime, verified work, staking, escrow and slashing are specified in the plan. No additional inflation or real-value genesis allocation is authorized by these documents. Fee redistribution differs from Ethereum's base-fee burn policy and must be advertised as an EVE difference.

## Start implementation

Read [AGENTS.md](AGENTS.md), [goal.md](goal.md), the [planning index](docs/plan/README.md), and [execution status](docs/execution/STATUS.md).

```text
/goal Implement goal.md end to end. Follow AGENTS.md and docs/plan/23-task-backlog-and-execution.md. Work in dependency-aware bulks, implement and test real functionality, maintain evidence and handoff files, and continue through all unblocked work. Never mark a target passed without its required evidence.
```

The goal file defines exactly what DONE, BLOCKED and TARGET_UNMET mean. Completing the documentation or a local prototype is not completing the whole project.

## Scope boundaries

The first implementation is an independent devnet. Alephium bridging, settlement proofs, production keys, public mainnet launch, token issuance to real users, and paid infrastructure require separate explicit approval. No Alephium-inherited security is claimed. A public repository does not hide master source or secrets; source-distribution licensing and any private repository split are separate owner decisions.
