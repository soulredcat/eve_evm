# 10 — Master and Public Node Operating Model

## Purpose

This document records the current agreed direction for the EVE EVM node model.

The design intentionally separates the protected canonical infrastructure from permissionless public infrastructure.

---

## 1. Master Node

The Master Node is developer-controlled infrastructure.

### Access model

- Master Node source/runtime is controlled by the EVE developers.
- Master Node is not intended to be permissionless.
- Master Node must not expose public administrative access.
- Master Node must not expose its canonical database directly to Public Nodes.
- Master Node should not have a public-facing management surface.

### Responsibilities

The Master Node is responsible for:

- canonical chain state;
- deterministic EVM execution in the initial implementation;
- block construction;
- durable state persistence;
- WAL / crash recovery;
- snapshots;
- canonical block history;
- publishing block/state updates;
- maintaining the authoritative state root;
- coordinating state synchronization;
- serving the private versioned protocol used by Public Nodes.

### Storage model

The Master Node uses:

```text
RAM
= hot/current execution state

NVMe Gen5
= durable canonical state
= WAL
= block data
= snapshots
= recovery data
```

RAM is an acceleration layer and must never be the only canonical copy.

---

## 2. Master Operating Modes

The implementation should support staged deployment.

### Mode A — MASTER_ONLY

Used during early development.

```text
Master Node
├─ EVM execution
├─ canonical state
├─ block builder
├─ durable storage
├─ local/private RPC
└─ recovery
```

The chain must be able to operate and be tested without any Public Node.

This is the first implementation target.

### Mode B — MASTER_WITH_PUBLIC

After the Master is stable:

```text
Master
  ↓
signed/versioned state + block protocol
  ↓
Public Nodes
```

The Master remains the canonical state source while Public Nodes synchronize from it.

### Mode C — PRIMARY + HOT STANDBY

Later availability mode:

```text
Primary Master
      │
      └── replication
              ↓
        Hot Standby
```

Only one logical canonical writer may exist at a time.

Split-brain must be prevented using explicit fencing/lease rules.

---

## 3. Public Nodes

Public Nodes are permissionless.

### Access model

There is no protocol-level fixed maximum number of Public Nodes.

```text
Public Node #1
Public Node #2
Public Node #3
...
Public Node #N
```

Anyone may run a Public Node if they meet the protocol and resource requirements.

### Responsibilities

A Public Node may provide:

- public JSON-RPC;
- WebSocket subscriptions;
- transaction ingress;
- current-state RAM replica;
- block verification;
- state-root verification;
- validator duties;
- peer-to-peer distribution;
- state-delta propagation;
- snapshot distribution;
- health/status service.

Public Nodes do not become canonical simply because they are publicly reachable.

---

## 4. Source Tree Separation

Master and Public Node source trees must be separate.

Proposed layout:

```text
eve_evm/
├─ master/
│  └─ src/
│
├─ public/
│  └─ src/
│
├─ crates/
│  ├─ protocol/
│  ├─ primitives/
│  ├─ evm/
│  ├─ crypto/
│  ├─ networking/
│  └─ shared/
│
└─ docs/
```

The design must avoid copy-pasting consensus-critical logic between Master and Public Node.

Shared types and protocol rules belong in reusable crates.

Example:

```text
Master
   ┐
   ├── shared protocol / primitives
   │
Public
   ┘
```

This allows a dedicated Public Node distribution to be built later without duplicating Master implementation code.

---

## 5. State Synchronization

Public Nodes synchronize from the canonical chain.

### Bootstrap

A new Public Node should:

```text
start
↓
discover network / trusted bootstrap source
↓
download verified snapshot
↓
verify snapshot commitment
↓
apply ordered state deltas / blocks
↓
reach current canonical root
↓
enter live synchronization
```

### Live sync

The intended model is:

```text
Master canonical state
        ↓
block + state delta
        ↓
Public Nodes
        ↓
independent verification
        ↓
RAM/current-state replica
```

Public Nodes must not mount or directly query the Master database.

Synchronization must happen through a versioned protocol.

---

## 6. Bulk State Synchronization

Cross-node synchronization should use bulk state transitions instead of individual database operations.

Example:

```text
Base Root A

100,000 transactions execute

State Delta Batch #500
↓
New Root B
```

The logical synchronization unit is the batch even if transport uses many network chunks.

A Public Node must be able to verify:

```text
apply(State A, Delta)
= State B

computed_root == announced_root
```

---

## 7. Unlimited Public Nodes Without Overloading Master

Permissionless Public Nodes must not imply that every node maintains a direct high-bandwidth connection to the Master.

The distribution topology should evolve toward:

```text
                 Master
                   ↓
              Seed Nodes
             /     |     \
            ↓      ↓      ↓
         Public  Public  Public
            ↔      ↔      ↔
              P2P network
```

A node may obtain data from peers, but every protocol object must remain independently verifiable.

This prevents Master bandwidth from becoming the scaling limit for Public Node count.

---

## 8. Public Node RAM Model

Public Nodes may use a RAM-heavy current-state model.

Example:

```text
Public Node
├─ current state in RAM
├─ mempool
├─ RPC cache
├─ recent blocks
└─ minimal local persistence
```

A Public Node may be treated as replaceable.

After a crash/restart:

```text
restart
↓
load/download snapshot
↓
verify root
↓
catch up deltas
↓
rejoin network
```

Loss of a Public Node must never destroy canonical chain state.

---

## 9. Validator Placement

Validator functionality is expected to run on Public Nodes.

Conceptually:

```text
Master
= canonical state / block source

Public Node
= RPC + validator + replicated current state
```

The exact finality authority of validators versus the developer-controlled Master is not yet frozen and must be specified explicitly before production.

Important distinction:

- Public Nodes can verify.
- Validators can vote/sign.
- Master remains the initial canonical state authority.
- The protocol must explicitly define what happens when validator quorum is unavailable.

---

## 10. Security Boundary

The security goal is that compromising a Public Node does not provide direct access to the Master.

Required separation:

```text
Internet
  ↓
Public Node Layer
  ↓
Authenticated Internal Gateway / Protocol
  ↓
Protected Master Network
```

Public Nodes must never receive:

- Master filesystem credentials;
- Master database credentials;
- unrestricted Master RPC access;
- Master administrative keys;
- release-signing keys.

The Master must treat every message arriving from a Public Node as untrusted input.

---

## 11. Key Separation

At minimum, use separate keys for:

```text
Master identity/signing
Validator identity/signing
Software release signing
Administrative access
Backup/recovery
```

One compromised key must not compromise every system role.

---

## 12. State Sync vs Software Update

These are separate systems.

### State synchronization

Automatic:

```text
blocks
state deltas
snapshots
validator/finality data
```

### Software update

Must be authenticated.

Recommended flow:

```text
new release
↓
signed manifest
↓
download binary/package
↓
verify cryptographic hash
↓
verify release signature
↓
verify protocol compatibility
↓
rolling restart
```

A Master must never push an unsigned executable that Public Nodes blindly run.

---

## 13. Rolling Public Node Update

With many Public Nodes, upgrades should be rolling.

Example:

```text
Public 1
update → verify healthy

Public 2
update → verify healthy

...

Public N
```

This preserves public RPC availability during software upgrades.

---

## 14. Developer Experience

Even if the Master and Public architectures are custom, dApp developers should see a normal EVM-compatible interface.

```text
Solidity
↓
standard transaction
↓
JSON-RPC
↓
Public Node
↓
EVE protocol
↓
Master / canonical execution
```

Internal storage, WAL, snapshot, delta synchronization, and Master topology must remain invisible to normal application developers.

---

## 15. Initial Build Order

The current agreed build sequence is:

```text
1. MASTER_ONLY
   ↓
2. deterministic EVM execution
   ↓
3. block production
   ↓
4. durable state + restart recovery
   ↓
5. minimal EVM JSON-RPC
   ↓
6. state snapshot/delta protocol
   ↓
7. one Public Node
   ↓
8. validator functionality
   ↓
9. multiple Public Nodes
   ↓
10. permissionless P2P distribution
   ↓
11. Primary + Hot Standby Master
   ↓
12. multi-region architecture
```

Public Node development should depend on a stable Master protocol rather than forcing the initial Master implementation to solve every distributed-system problem immediately.

---

## 16. Current Architectural Invariant

The most important current invariant is:

> The Master is the protected canonical source of truth, while Public Nodes are permissionless, independently verifying, replaceable network-facing replicas/validators that synchronize through a strict versioned protocol.

This decision may evolve as decentralization/finality requirements mature, but any change must be explicitly documented rather than introduced implicitly in code.
