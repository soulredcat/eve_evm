# 06 — Regional Synchronization and Finality

## Goal

Reduce user ingress latency by processing transactions near users while avoiding a full WAN round trip for every individual transaction where protocol semantics permit.

## Concept

```text
Region ID ─┐
Region JP ─┼→ regional batches → global/canonical finality
Region US ─┘
```

Regional identity and state-shard identity should remain separate concepts. State placement may move between regions.

## Bulk synchronization

Do not synchronize a full database after every block.

A replica should bootstrap from:

```text
verified snapshot
+ ordered state deltas
= current state
```

A logical delta batch may contain many changes while being transported as multiple chunks/packets.

## Synchronization object

Candidate metadata:

- protocol version;
- region/shard ID;
- sequence/batch ID;
- base state root;
- new state root;
- transaction commitment;
- delta commitment;
- compressed/uncompressed size;
- producer signature;
- finality certificate if available.

## Cross-region transactions

Transactions touching state owned by multiple regions/shards require explicit atomicity semantics.

Potential strategies to research:

- deterministic co-location of strongly connected state;
- synchronous cross-shard atomic commit;
- asynchronous receipts/messages;
- execution domains with local atomicity and delayed cross-domain completion.

Standard EVM calls are synchronous, so any deviation visible to contracts must be treated as a compatibility change.

## Finality

The protocol must distinguish:

- accepted;
- executed;
- locally confirmed;
- validator-certified;
- globally finalized.

Wallet/RPC semantics must map these states clearly.

## Failover

Regional failure must define:

- who can take over the affected shard;
- how ownership is fenced to prevent two active owners;
- required finality point before failover;
- how replicas prove they are caught up;
- what happens to in-flight transactions.
