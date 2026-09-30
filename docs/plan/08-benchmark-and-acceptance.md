# 08 — Benchmark and Acceptance

## Principle

A TPS number without workload, duration, hardware, correctness, state growth, and latency distribution is not an engineering result.

## Mandatory benchmark metadata

Every published result must record:

- commit SHA;
- protocol/config version;
- CPU model/count;
- RAM;
- storage model/filesystem;
- network topology;
- transaction type;
- account/contract cardinality;
- contention level;
- block/batch interval;
- run duration;
- initial and final state size.

## Metrics

At minimum:

- admitted TPS;
- successfully executed TPS;
- finalized TPS;
- p50/p95/p99 submission latency;
- p50/p95/p99 finality latency;
- CPU utilization;
- RAM use;
- storage read/write bandwidth;
- write amplification if measurable;
- network ingress/egress;
- state growth bytes/transaction;
- WAL growth;
- state-sync backlog;
- validator lag;
- failed/re-executed/conflicted transactions.

## Workload classes

### B0 — Simple transfer
Baseline only. Never use as the sole 1M TPS claim.

### B1 — Token transfer
ERC-20-like state access.

### B2 — Independent contract writes
Measures parallelism under low contention.

### B3 — Single hot contract/pool
Measures serial/conflict bottleneck.

### B4 — Multi-pool atomic transaction
DEX/arbitrage-style multi-state access.

### B5 — Mixed workload
Realistic distribution of reads/writes/contracts.

### B6 — Adversarial workload
Malformed transactions, nonce collisions, repeated conflicts, oversized inputs within protocol limits.

## Long-term 1M TPS acceptance

The long-term target is not met until **finalized** throughput reaches the target for a sustained test while:

- state remains correct;
- backlog is bounded;
- replicas keep up;
- restart recovery works;
- finality does not continuously degrade;
- storage does not grow unexpectedly;
- benchmark uses a declared non-trivial workload.

Aggregate cross-shard throughput must be labeled as aggregate, not single-shard throughput.
