# 11 — Main Goals and Project Planning

## Main Goal

Build EVE EVM as a high-throughput, EVM-compatible blockchain architecture that keeps the developer experience familiar while using a custom execution, storage, synchronization, validation, and regional infrastructure model.

The project must prioritize:

1. correctness;
2. deterministic state;
3. recoverability;
4. security isolation;
5. horizontal scalability;
6. permissionless Public Nodes;
7. developer compatibility;
8. measurable performance.

The long-term performance objective is up to **1,000,000 aggregate finalized transactions per second**, but this remains an engineering target until proven by reproducible end-to-end benchmarks.

---

## Core Architectural Goal

The chain should separate responsibilities instead of forcing every node to do everything.

```text
Developer / User
      ↓
Public Node Layer
      ↓
Validator / Verification Layer
      ↓
Protected Master Layer
      ↓
EVM Execution
      ↓
Canonical State
      ↓
RAM + Durable NVMe Storage
```

The first implementation starts simple with a single Master and grows incrementally.

---

## Goal 1 — Master Node as Protected Canonical Core

The Master is the protected source of canonical state.

Primary responsibilities:

- deterministic EVM execution;
- canonical state;
- block production;
- state-root generation;
- durable persistence;
- WAL and recovery;
- snapshots;
- state-delta generation;
- internal protocol for Public Nodes.

Initial mode:

```text
MASTER_ONLY
```

Later modes:

```text
MASTER_WITH_PUBLIC
PRIMARY + HOT_STANDBY
MULTI_REGION
```

The Master must not depend on Public Nodes to preserve canonical state.

---

## Goal 2 — Permissionless Public Nodes

Public Nodes are intended to be unlimited in count at the protocol level.

Public Nodes should:

- provide RPC;
- provide WebSocket;
- accept transactions;
- run validator duties;
- independently verify blocks/state roots;
- maintain current-state replicas;
- participate in P2P propagation;
- distribute snapshots/deltas.

Public Nodes are replaceable and should be recoverable from verified snapshots plus ordered deltas.

The number of Public Nodes must not scale Master bandwidth linearly.

---

## Goal 3 — Separate Master and Public Source Trees

The repository should keep Master and Public Node implementations separated.

```text
master/
public/
crates/
docs/
```

Shared consensus-critical types and protocol rules belong in shared crates.

The design should make it possible to distribute a Public Node source/binary independently from the Master implementation.

---

## Goal 4 — EVM Developer Compatibility

Application developers should not need to understand EVE's internal architecture.

Target developer experience:

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

The project should preserve standard EVM semantics unless a difference is explicitly versioned and documented.

---

## Goal 5 — High-Performance State

The storage model should minimize latency and unnecessary disk amplification.

Target model:

```text
RAM
= hot/current state

NVMe
= durable current state
= WAL
= snapshots
= immutable block/history segments
```

The project should prefer compact binary representations internally where appropriate.

History should avoid millions of tiny files.

---

## Goal 6 — Deterministic Parallel Execution

The long-term throughput target requires parallel execution.

Parallelism must never change canonical results.

Every parallel execution result must be equivalent to the deterministic reference ordering.

Research and implementation areas:

- transaction dependency detection;
- read/write sets;
- conflict detection;
- optimistic execution;
- deterministic retry;
- state partitioning;
- pool/contract identity;
- execution domains.

A unique pool ID helps with ownership and scheduling but does not automatically make cross-pool transactions independent.

---

## Goal 7 — Fast Regional Transaction Ingress

Users should normally connect to nearby Public/Regional infrastructure.

WAN latency should not be inserted into every local transaction unnecessarily.

Regional execution may produce batches:

```text
Region A → Batch A
Region B → Batch B
Region C → Batch C
```

The global/canonical layer then synchronizes and finalizes according to the final protocol.

State synchronization should be bulk/delta based.

---

## Goal 8 — Bulk State Synchronization

State replication should operate on state transitions, not remote database queries.

Target:

```text
Base Root A
+ Delta Batch
= New Root B
```

A node must verify that the resulting root is correct before accepting the update.

Snapshots are used for bootstrap; deltas are used for continuous synchronization.

---

## Goal 9 — Security Isolation

The Master must be structurally isolated from Internet-facing infrastructure.

Required principles:

- no public Master admin API;
- no public Master database;
- no shared unrestricted credentials;
- strict authenticated internal protocol;
- mTLS or equivalent node authentication;
- independent management network;
- strict message size and decoding limits;
- fuzzing of protocol decoders;
- separate validator, release, Master, and admin keys.

Assume a Public Node can be fully compromised without granting direct canonical-storage access.

---

## Goal 10 — Validator and Staking Model

Validators run on Public Nodes.

Initial economic proposal:

```text
100% transaction fees
├─ 40% burn
├─ 30% node reward pool
└─ 30% validator reward pool
```

Reward distribution should depend on measurable protocol work and availability.

Validator scoring may include:

- stake;
- uptime;
- valid participation;
- timely voting;
- correct proposals;
- slash/jail history.

Node scoring must use verifiable work rather than self-reported usage.

---

## Goal 11 — Chain Survival and Recovery

The design must assume failures.

Required recovery scenarios:

- Public Node crash;
- all Public Nodes unavailable;
- Master process crash;
- partial WAL write;
- corrupted state tail;
- network partition;
- validator lag;
- software upgrade failure;
- regional outage.

Canonical state must always have a deterministic recovery path.

---

## Goal 12 — Software Update Model

State synchronization and software updates are separate.

State sync may be automatic.

Software updates must use:

- signed release metadata;
- cryptographic hashes;
- protocol compatibility checks;
- rolling upgrades;
- rollback plan.

Public Nodes must never blindly execute binaries received from a Master process.

---

# Project Planning

## Stage 0 — Planning and Specification

No production code should start before the basic invariants are written.

Deliverables:

- architecture documents;
- block format;
- transaction format;
- state model;
- storage model;
- Master/Public protocol;
- threat model;
- validator model;
- fee accounting;
- benchmark methodology;
- failure/recovery rules.

### Exit criteria

All core responsibilities and trust boundaries are explicit.

---

## Stage 1 — MASTER_ONLY Prototype

Build the smallest complete chain.

Target:

```text
transaction
↓
signature validation
↓
EVM execute
↓
state update
↓
block
↓
state root
↓
persist
↓
restart
↓
same state
```

Deliverables:

- Master binary;
- EVM integration;
- state database;
- WAL;
- block builder;
- receipts/logs;
- state root;
- minimal RPC;
- restart recovery.

### Exit criteria

A deterministic replay test always produces the same canonical state root.

---

## Stage 2 — Public Node Coupling

Build one Public Node that synchronizes from the Master.

Deliverables:

- snapshot bootstrap;
- block stream;
- delta stream;
- root verification;
- Public Node RAM state;
- RPC separation;
- transaction forwarding.

### Exit criteria

Delete the Public Node state, restart it, and rebuild it entirely from verified Master/network data.

---

## Stage 3 — Validator Layer

Add validator behavior to Public Nodes.

Deliverables:

- validator identity;
- signing;
- verification;
- validator registry;
- participation accounting;
- finality rules;
- catch-up behavior.

### Exit criteria

A malformed or invalid candidate block is rejected consistently by independent validators.

---

## Stage 4 — Staking and Economics

Implement:

- staking;
- delegation;
- epochs;
- uptime accounting;
- work accounting;
- fee split;
- burn;
- node rewards;
- validator rewards;
- slashing/jailing.

### Exit criteria

Epoch reward results are deterministic and independently reproducible.

---

## Stage 5 — Multi-Public Node Network

Scale to many Public Nodes.

Deliverables:

- P2P discovery;
- seed nodes;
- peer sync;
- snapshot distribution;
- delta propagation;
- peer verification;
- anti-abuse controls.

### Exit criteria

Adding Public Nodes does not require a proportional increase in Master direct connections.

---

## Stage 6 — Parallel EVM Execution

Implement parallel scheduling.

Start with a serial engine as the reference implementation.

Every optimized engine result must match the serial reference.

Deliverables:

- dependency model;
- parallel scheduler;
- conflict detection;
- deterministic re-execution;
- high-contention tests;
- pool/contract partitioning experiments.

### Exit criteria

Parallel and serial executions produce identical:

- balances;
- nonces;
- logs;
- receipts;
- state root;
- block hash inputs.

---

## Stage 7 — Master High Availability

Introduce:

```text
Primary Master
+
Hot Standby
```

Deliverables:

- replication;
- fencing;
- failover;
- split-brain prevention;
- recovery tests.

### Exit criteria

Forced Primary loss promotes exactly one valid replacement without canonical-state divergence.

---

## Stage 8 — Multi-Region Architecture

Add geographically distributed execution and state synchronization.

Deliverables:

- regional IDs;
- regional ingress;
- bulk state delta sync;
- state ownership;
- regional failover;
- global finality model;
- cross-region transaction semantics.

### Exit criteria

A regional outage does not corrupt canonical state and does not create two valid owners for the same state domain.

---

## Stage 9 — Performance Scaling

Performance targets must be reached incrementally.

```text
10k TPS
↓
50k TPS
↓
100k TPS
↓
250k TPS
↓
500k TPS
↓
1M aggregate finalized TPS
```

At every step measure:

- finalized TPS;
- latency p50/p95/p99;
- CPU;
- RAM;
- NVMe bandwidth;
- state growth;
- WAL growth;
- network bandwidth;
- validator lag;
- sync backlog;
- conflict/retry rate.

Do not proceed solely because RPC ingress reaches the target.

---

# Definition of Success

EVE EVM is successful only if it can simultaneously provide:

- deterministic and recoverable canonical state;
- EVM-compatible developer experience;
- protected Master infrastructure;
- unlimited permissionless Public Nodes;
- independently verifiable block/state data;
- validator and staking economics;
- safe software upgrades;
- scalable state synchronization;
- parallel execution without correctness loss;
- multi-region operation;
- reproducible high-throughput benchmarks.

The project's primary objective is not merely to produce a high TPS number.

The objective is to build a chain that remains **correct, recoverable, verifiable, secure, scalable, and usable by normal EVM developers while throughput increases**.
