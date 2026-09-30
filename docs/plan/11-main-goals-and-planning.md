# 11 — Main Goals and Project Planning

## Main Goal

Build EVE EVM as a high-throughput EVM-compatible blockchain whose responsibilities are deliberately separated across Public, Validator, and Master runtimes.

The production architecture should preserve a familiar EVM developer experience while using custom execution, storage, validation, synchronization, and regional scaling mechanisms.

The long-term performance objective is up to **1,000,000 aggregate finalized transactions per second**, but this remains an engineering target until proven by reproducible end-to-end benchmarks.

## Primary Architecture

```text
Users / dApps
      ↓
Public Nodes
RPC + P2P + transaction ingress
      ↓
Validator Nodes
execute + propose + verify + vote
      ↓
FINALITY
      ↓
Master Nodes
sync + durable state + snapshots + archive + recovery
```

Primary rule:

> **Validators decide. Master remembers.**

## Goal 1 — Protected Master Layer

Master is developer-controlled infrastructure, but not production consensus authority.

Master provides:

- finalized-state synchronization;
- durable canonical storage;
- WAL/recovery;
- snapshots;
- archive/history;
- verified distribution to rebuilding nodes;
- protected internal protocols.

Early `MASTER_ONLY` mode exists only so the EVM/storage/block core can be built before distributed validation exists.

## Goal 2 — Permissionless Public Layer

Public Nodes:

- are unlimited in protocol count;
- expose JSON-RPC/WebSocket;
- accept and relay transactions;
- maintain current state replicas;
- verify finalized data;
- participate in P2P distribution;
- remain replaceable.

Public Nodes do not automatically receive voting power.

## Goal 3 — Dedicated Validator Layer

Validator Nodes have a dedicated runtime/source folder.

Validators:

- execute/replay transactions;
- propose blocks;
- verify state transitions;
- vote/sign;
- create finality certificates;
- participate in staking;
- earn rewards based on verifiable participation/work;
- can be jailed/slashed for protocol violations.

The production chain cannot declare new finality without the required validator quorum.

## Goal 4 — Repository Separation

```text
master/
public/
validator/
crates/
docs/
```

Shared consensus-critical logic belongs in reusable crates, not duplicate implementations.

## Goal 5 — EVM Developer Compatibility

Developer surface:

```text
Solidity
↓
ABI
↓
ethers / viem / web3
↓
Ethereum-style JSON-RPC
↓
EVE Public Node
```

Internal storage/synchronization architecture must remain invisible to normal dApp developers.

## Goal 6 — High-Performance State

```text
RAM
= hot/current working state

NVMe
= durable finalized state
= WAL
= snapshots
= history segments
```

Use compact binary formats where appropriate and avoid millions of tiny history files.

## Goal 7 — Deterministic Parallel Execution

Parallel execution is allowed only if it produces the same canonical result as deterministic reference execution.

Research areas:

- read/write sets;
- dependency graphs;
- optimistic execution;
- conflict detection;
- deterministic retry;
- state partitioning;
- pool/contract identities;
- execution domains.

## Goal 8 — Bulk State Synchronization

```text
Finalized Root A
+ verified delta batch
= Finalized Root B
```

Snapshots bootstrap nodes; finalized deltas keep them current.

## Goal 9 — Regional Scaling

Regional infrastructure should reduce ingress latency without forcing WAN round trips into every transaction.

Regional outputs may be batched, but global/final consensus semantics must remain explicit and deterministic.

## Goal 10 — Security Isolation

A compromised Public or Validator Node must not gain direct Master storage/admin access.

Use:

- separate network zones;
- strict protocol boundaries;
- independent credentials;
- bounded decoding;
- node authentication;
- fuzz testing;
- key separation.

## Goal 11 — Economics

Initial fee proposal:

```text
40% burn
30% node reward pool
30% validator reward pool
```

Rewards must depend on protocol-verifiable work and availability.

## Goal 12 — Failure Survival

Required failure cases include:

- Public Node crash;
- Validator crash;
- loss of validator quorum;
- Master crash;
- Master offline while validators continue;
- corrupted WAL tail;
- network partition;
- regional outage;
- failed software upgrade.

No failure mode may silently redefine finality.

# Project Planning

## Stage 0 — Planning and Specification

Define:

- transaction format;
- block format;
- state commitment;
- finality certificate;
- validator identity;
- validator quorum rules;
- storage format;
- Master sync protocol;
- Public/Validator P2P protocol;
- failure semantics;
- economics;
- benchmark methodology.

## Stage 1 — MASTER_ONLY Development Prototype

Build:

```text
tx
→ EVM
→ state
→ block
→ root
→ durable persist
→ restart
→ identical state
```

This stage validates the core engine, not the final production trust model.

## Stage 2 — Shared Protocol/Core

Extract consensus-critical reusable crates:

- primitives;
- transaction types;
- block types;
- state commitments;
- EVM interface;
- crypto;
- networking protocol;
- snapshot/delta format.

## Stage 3 — Public Node

Build:

- RPC;
- WebSocket;
- P2P;
- mempool/relay;
- RAM/current-state replica;
- snapshot/delta bootstrap;
- finalized block verification.

## Stage 4 — Validator Node

Build dedicated validator runtime:

- consensus keys;
- validator registry;
- EVM replay/execution;
- proposer selection;
- block proposal;
- vote/sign;
- finality certificate;
- catch-up/rejoin.

## Stage 5 — Move Finality Authority to Validators

Production flow becomes:

```text
Public
→ Validators
→ FINAL
→ Master sync/persist
```

Acceptance:

- Master cannot unilaterally finalize an invalid block;
- forged Master state without validator certificate is rejected;
- valid validator-finalized blocks are recoverable by Master.

## Stage 6 — Staking and Economics

Implement:

- stake/delegation;
- epochs;
- uptime;
- work accounting;
- 40/30/30 fee allocation;
- burn;
- rewards;
- jail/slashing.

## Stage 7 — Multi-Node P2P Network

Scale Public and Validator nodes without proportional Master fan-out.

## Stage 8 — Parallel EVM Execution

Serial execution remains the correctness oracle.

Parallel result must match serial result for:

- balances;
- nonces;
- storage;
- logs;
- receipts;
- state root.

## Stage 9 — Master High Availability

Add protected Primary/Standby persistence infrastructure without creating consensus authority or split-brain canonical storage.

## Stage 10 — Multi-Region

Add:

- regional ingress;
- regional execution domains;
- bulk synchronization;
- regional failover;
- cross-region semantics;
- global finality integration.

## Stage 11 — Performance Scaling

```text
10k
→ 50k
→ 100k
→ 250k
→ 500k
→ 1M aggregate finalized TPS
```

Measure finalized throughput, not only RPC ingress.

Track:

- p50/p95/p99 latency;
- CPU;
- RAM;
- NVMe;
- state growth;
- network bandwidth;
- validator lag;
- consensus latency;
- sync backlog;
- conflict/retry rate.

# Definition of Success

EVE EVM succeeds only if it remains:

- deterministic;
- recoverable;
- EVM-compatible;
- validator-finalized;
- Master-durable without Master consensus bottleneck;
- permissionless at the Public Node layer;
- secure across trust zones;
- scalable across execution and networking;
- reproducibly benchmarked.

The objective is not merely a high TPS number. The objective is a high-throughput chain whose correctness and finality remain independently verifiable.
