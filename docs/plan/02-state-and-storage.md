# 02 — State and Storage

## Objective

Keep active state fast while ensuring crash-safe deterministic recovery.

## Proposed tiers

### Tier 0 — RAM

Use for:

- hot/current account state;
- active contract storage cache;
- pool state;
- nonce/balance cache;
- execution working sets.

RAM is never the only durable source of canonical state.

### Tier 1 — Durable current-state store

Candidate technologies should be benchmarked rather than selected by preference alone.

Candidates:

- MDBX;
- RocksDB;
- Pebble-equivalent architecture if language choice permits;
- a purpose-built storage layer only after evidence that mature engines are insufficient.

Evaluate:

- random read latency;
- sequential and random writes;
- write amplification;
- compaction stalls;
- snapshot behavior;
- crash recovery;
- storage overhead;
- state-root integration.

### Tier 2 — WAL

Append-only write-ahead log records enough information to recover committed state transitions.

Required properties:

- checksum;
- sequence number;
- block/batch identity;
- deterministic replay;
- truncation/rotation policy;
- corruption detection.

### Tier 3 — Snapshots

Periodic immutable snapshots allow fast bootstrap without replaying chain history from genesis.

### Tier 4 — Block/history segments

Prefer large immutable binary segment files over millions of tiny filesystem files.

Potential layout:

```text
blocks-00000000-00099999.seg
blocks-00100000-00199999.seg
...
```

Compression can be applied to immutable history after profiling.

## State model requirements

Every committed transition must map:

```text
previous_state_root
+ ordered transaction batch
→ deterministic new_state_root
```

State delta synchronization must allow a replica to verify that applying a delta to the expected base root produces the announced new root.

## Recovery acceptance criteria

After abrupt process termination:

1. load latest valid snapshot/current-state database;
2. replay only valid committed WAL records;
3. reject partial/corrupted tail records;
4. reproduce the exact canonical state root;
5. resume without balance, nonce, or contract-storage divergence.
