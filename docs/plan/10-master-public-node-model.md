# 10 — Master, Public and Validator Node Operating Model

## Purpose

This document records the current agreed node model for EVE EVM.

The design separates three responsibilities:

```text
Public Node
= network-facing RPC/P2P/state replica

Validator Node
= execution + proposal + vote + finality

Master Node
= finalized-state sync + durable persistence + snapshots + archive + recovery
```

The production trust rule is:

> **Validators decide. Master remembers.**

---

## 1. Master Node

The Master Node is developer-controlled protected infrastructure.

### Production responsibilities

- receive finalized blocks/state transitions;
- verify finality certificates;
- persist finalized canonical state;
- maintain WAL / crash recovery;
- produce and store snapshots;
- maintain archive/history;
- publish verified snapshots/state deltas;
- support recovery/bootstrap of network nodes;
- expose only a strict versioned internal synchronization protocol.

### What Master must not do in production

The Master must not be the unilateral source of consensus truth.

It must not:

- finalize blocks by itself;
- override validator quorum;
- expose public admin/database access;
- require every user transaction to pass through it;
- become the hot-path bottleneck for execution.

### Early development exception

The first implementation may run in `MASTER_ONLY` prototype mode to prove:

```text
transaction
→ EVM execution
→ block
→ state root
→ durable commit
→ restart/recovery
```

This is a development mode only. Once Validator Nodes exist, production finality authority moves to validators.

---

## 2. Public Node

Public Nodes are permissionless and unlimited in count at the protocol level.

### Responsibilities

- public JSON-RPC;
- WebSocket;
- transaction ingress;
- P2P propagation;
- current-state RAM replica/cache;
- block/state verification;
- snapshot/delta distribution;
- routing transactions toward Validator Nodes;
- health/status services.

A Public Node has no voting power unless it also runs/attaches to a registered Validator runtime.

Public Nodes are replaceable and may use RAM-heavy current-state storage.

---

## 3. Validator Node

Validator Nodes have a dedicated source/runtime.

### Responsibilities

- receive ordered transaction candidates/mempool data;
- execute/replay EVM state transitions;
- propose blocks when selected;
- verify candidate blocks;
- compute/verify state roots;
- vote/sign;
- participate in finality;
- maintain validator identity and consensus keys;
- report protocol-verifiable uptime/participation;
- participate in staking/slashing rules.

### Consensus authority

A finalized block must be backed by the required validator quorum/certificate.

The Master persists finalized results but cannot manufacture finality.

---

## 4. Source Tree Separation

```text
eve_evm/
├─ master/
│  └─ src/
├─ public/
│  └─ src/
├─ validator/
│  └─ src/
├─ crates/
│  ├─ protocol/
│  ├─ primitives/
│  ├─ evm/
│  ├─ crypto/
│  ├─ networking/
│  ├─ state/
│  └─ shared/
└─ docs/
```

Consensus-critical logic must not be copy-pasted across binaries.

Shared block, transaction, state-root, signature, and protocol types belong in shared crates.

---

## 5. Runtime Relationship

Normal production flow:

```text
User
↓
Public Node
↓
Validator Network
↓
execute / propose / verify / vote
↓
FINALIZED BLOCK
↓
Master sync
↓
durable NVMe state + snapshot + archive
↓
verified distribution back to network
```

The Master is not in the per-transaction consensus hot path.

---

## 6. State Synchronization

A new Public or Validator Node should bootstrap using:

```text
verified snapshot
+
ordered finalized deltas/blocks
=
current finalized state
```

Nodes must verify commitments and finality certificates before accepting canonical state.

No Public/Validator Node may directly mount or query the Master database.

---

## 7. Bulk Synchronization

Synchronization operates on finalized batches/state transitions.

```text
Base Root A
+ Finalized Delta Batch
= New Root B
```

The receiver verifies:

```text
computed_root == announced_root
certificate == valid quorum
```

Only then is the transition accepted as finalized.

---

## 8. Scaling Public Nodes

Unlimited Public Nodes must not create unlimited direct Master connections.

Target distribution:

```text
Master / Snapshot Sources
        ↓
     Seed Nodes
    /    |    \
Public ↔ Public ↔ Public
          |
      Validators
```

P2P distribution carries data outward while cryptographic verification preserves trust.

---

## 9. Failure Semantics

### Public Nodes unavailable

Consensus may continue if Validator Nodes still communicate.

### Master unavailable

Validators may continue consensus/finality if they retain the required state/data.

Finalized blocks queue for later Master persistence.

When Master returns:

```text
read last persisted finalized height
→ sync missing finalized blocks
→ verify certificates
→ persist
→ resume snapshot/delta service
```

### Validator quorum unavailable

Finality stops.

Master may preserve existing state but must not silently replace validator finality.

---

## 10. Security Boundary

```text
Internet
  ↓
Public Layer
  ↓
Validator/Consensus Network
  ↓
Finalized protocol objects
  ↓
Protected Master Network
```

Master must not expose:

- filesystem credentials;
- database credentials;
- unrestricted admin RPC;
- release-signing keys;
- validator private keys.

All inbound data is treated as untrusted until cryptographically and structurally verified.

---

## 11. Key Separation

Use separate key domains for:

```text
Master identity
Validator consensus identity
Validator staking identity where applicable
Software release signing
Administrative access
Backup/recovery
```

---

## 12. Software Update vs State Sync

### State sync

Automatic and protocol-driven:

- finalized blocks;
- finality certificates;
- state deltas;
- snapshots.

### Software updates

Authenticated and separately signed:

```text
signed release manifest
→ download
→ verify hash
→ verify release signature
→ compatibility check
→ rolling update
```

---

## 13. Build Order

```text
1. MASTER_ONLY prototype
2. deterministic EVM + durable state
3. block/state formats
4. shared protocol crates
5. Public Node runtime
6. Validator Node runtime
7. validator quorum/finality
8. Master becomes finalized-state sync/persistence role
9. multi-Public/Validator network
10. staking/economics
11. parallel execution
12. Master HA
13. multi-region
14. scale validation
```

The architectural transition from development to production is explicit:

```text
EARLY:
Master can locally simulate full chain behavior

PRODUCTION:
Validators decide finality
Master synchronizes and persists finalized truth
```
