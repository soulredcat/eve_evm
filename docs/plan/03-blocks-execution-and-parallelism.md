# 03 — Blocks, Execution and Parallelism

## Block production

Blocks are produced by the chain protocol; they are not mined unless a PoW design is explicitly selected.

Initial conceptual flow:

```text
mempool
→ proposer/sequencer
→ deterministic ordering
→ EVM execution
→ receipts/logs
→ state root
→ candidate block
→ validator verification
→ finality
→ durable commit
```

## Minimum block header fields

To be finalized during protocol design:

- chain ID / protocol version;
- block height;
- parent block hash;
- timestamp/slot;
- proposer identity;
- transaction commitment/root;
- receipt commitment/root;
- state root;
- optional regional/batch commitments;
- finality certificate reference.

## Parallel execution

Transactions may execute concurrently only when correctness is equivalent to the canonical deterministic ordering.

Potential techniques to evaluate:

- access-list/read-write-set scheduling;
- optimistic parallel execution plus conflict detection/re-execution;
- deterministic state partitioning;
- actor/object-style ownership for native optimized paths;
- dependency graph scheduling.

## Pool identity

Pools/contracts may have deterministic unique state identities. A unique identity helps partition state but **does not by itself make transactions independent**.

A transaction touching:

```text
Pool A → Pool B → Pool C
```

creates a dependency across all three state domains and must remain atomic according to EVM semantics.

## High-contention requirement

Benchmarks must include many transactions writing the same contract/pool. Aggregate parallel throughput from independent accounts is not sufficient evidence for DEX throughput.

## Batch vs block

A regional execution batch and a canonical/global block may be different objects. The protocol must specify:

- what is final;
- what may be reorganized;
- what a wallet can call confirmed;
- what a validator signs;
- what public nodes expose through Ethereum-compatible RPC.
