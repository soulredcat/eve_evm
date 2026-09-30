# 01 — System architecture

## Authority map

| Role | Owns | Must not own |
|---|---|---|
| Public | RPC, local mempool policy, peer propagation, verified state queries | Production votes without validator registration; master credentials |
| Validator | Proposal validation, execution, consensus votes, finality, recent durable recovery data | Unilateral finality; master administration |
| Master | Verified finalized-state persistence, snapshots, archive, recovery distribution | Mandatory transaction sequencing, production proposer key, override of quorum |
| Shared crates | Deterministic formats, EVM adapter, roots, verification, storage contracts | Hidden role authority or cross-runtime globals |

One operator may co-locate public and validator processes. Source separation remains mandatory. A master host may run a separate local development validator, but that does not make the master role a voter.

## Required flow

```text
transaction -> public/P2P -> validator proposer -> validator checks and quorum
            -> finalized data -> peers and master followers
```

Validators do not wait for master acknowledgements in ordinary consensus. They must retain sufficient durable blocks/state to recover while masters lag. Masters may use fast authenticated state import or slower replay auditing; those are follower verification choices, not authority.

## Boundaries

Use separate traffic budgets for public RPC, consensus, block availability, bulk snapshot/delta transfer, and management. Sharing a physical host does not justify shared unrestricted credentials or unbounded queues. Consensus and management are not routed through a single mandatory master gateway.

## Baseline and evolution

The first devnet uses one logical ordered EVM state and a reviewed BFT adapter. Region IDs affect routing and operations, not independent write authority. Parallel execution must match serial results. Actual state sharding is a later experimental protocol change with its own atomicity and security gates.

The selected implementation baseline and dependency policy are in [22](22-code-layout-and-dependency-policy.md). Consensus and failure behavior are normative in [12](12-consensus-spec.md). All role APIs use versioned protocol data rather than direct database access.
