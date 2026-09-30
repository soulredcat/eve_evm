# 21 — Capacity and regional scaling

## P01 — Keep the target and remove shortcuts

The objective remains 1,000,000 aggregate finalized user TPS. Do not replace it with accepted RPC requests, executor loop iterations or the sum of replicated nodes. Adding full validators generally adds repeated verification and communication, not independent transaction capacity. Committee selection reduces how many participate in consensus but does not automatically divide each selected validator's execution workload.

Separate capacity dimensions: signature validation, transaction propagation, proposal construction, serial dependencies, EVM execution, state-root updates, voting, durable writes, history retention, replica catch-up, RPC simulation and load generation. Moving master off the mandatory transaction path removes one coupling; it does not make its incoming bytes or state writes disappear.

## P02 — Capacity calculations before deployment

For measured average wire bytes b, durable bytes d, average gas g, target rate R and replicas k:

```text
one-copy data rate = R * b bytes/second
one-copy durable growth = R * d bytes/second
uncompressed daily bytes = R * b * 86400
replication egress depends on topology, fanout and compression
required execution gas rate = R * g
```

Illustration only: at 200 bytes per transaction, 1M TPS produces 200 MB/s, 1.6 Gbit/s and 17.28 TB/day for one raw data stream before replication/indices/protocol overhead. Actual bytes must come from the workload. Aggregating a root does not erase this data.

At an illustrative 21,000 gas per simple transfer, 1M such transfers requires 21 billion gas/second. The local 30-million-gas block profile is a correctness baseline, not a configuration that inherently meets that rate. Raising gas/block limits without measuring execution, propagation and recovery is not a performance achievement.

Estimate storage headroom for WAL, compaction, snapshots, retained blocks and catch-up simultaneously. State deltas may collapse repeated writes to the same slot, but replay/availability still needs the committed transaction data and authenticated history under the chosen retention policy.

## P03 — Workload contract

W0 native transfer; W1 ERC-20; W2 independent storage updates; W3 one hot AMM pool; W4 atomic multi-pool execution; W5 mixed contracts; W6 adversarial/overload.

Default W5 mix: 20% native transfers, 30% ERC-20 transfers, 20% independent contract writes, 20% AMM swaps and 10% atomic two-pool operations. Start with at least 100,000 funded sender identities and 1,024 pools for the low-contention mixed profile; declare exact cardinalities, token-sharing graph, seed, gas distribution and hot-key skew. W3 separately concentrates writes on one pool. Increasing account/pool count to reduce contention is a changed workload and must be reported, not hidden.

The strongest 1M target statement names the passing workload. Transfer-only success cannot be presented as arbitrary EVM or single-pool success. For the project's mixed-EVM target, W5 must pass; W0–W4/W6 results remain mandatory disclosures. Cross-shard transactions, if enabled experimentally, must be counted once and included in an explicit profile.

Measure p50/p95/p99 admission-to-finality and execution/apply lag. Fix a latency SLO and error budget in the run manifest before measuring; default development sustained target SLO is p99 finality <= 5 seconds outside deliberately injected faults. Do not change it after seeing results to pass the run.

## P04 — Optimization sequence

1. Establish a correct serial reference and real end-to-end baseline.
2. Profile actual bottlenecks: CPU, allocations, locks, serialization, storage, networking and queue age.
3. Batch signature/IO work where safe; remove unnecessary copying and blocking.
4. Add bounded versioned state overlays and deterministic parallel execution.
5. Optimize commitment construction and storage layout without changing roots or safety.
6. Improve peer fanout, chunking, backpressure, catch-up and snapshot interference.
7. Re-benchmark consensus separately from execution; evaluate an alternate reviewed adapter only with full safety/commitment-binding tests.
8. Attempt state partitioning only after proving where single-domain replication stops scaling.

One shared token contract, account nonce, allowance, fee ledger or pool can create cross-worker dependencies. Track re-execution and serial fallback rather than reporting only ideal independent-task throughput. Fee/epoch accounting and trie root merging are included in the measured path.

## P05 — Regional rollout

First run one logical chain with geographically distributed public ingress and validators. The functional B9 gate can use explicit simulated regions and measured delay/loss on one host. Label this emulation. Real WAN performance requires a separate multi-host deployment and measured network conditions; do not infer it from a local benchmark.

Regional IDs and pool addresses do not create independent authority. Regional workers may speculate or prepare batches, but conflicting transactions still require a deterministic canonical order. Unfinalized local execution must not be exposed as final merely to advertise low latency.

Each master can mirror the finalized chain into its own storage independently. Replica lag, cold bootstrap and post-outage catch-up are benchmark dimensions. If every master retains full state/history, each must handle its configured share of the full committed data stream; 'bulk' does not divide that workload automatically.

## P06 — Gated sharding research

Create an isolated experimental profile and ADR before implementing independently owned state domains. Specify account/contract ownership, token balances across domains, synchronous call semantics, deterministic ordering, coordinator/lock failure behavior, two-phase outcomes if used, rollback/retry, data availability, validator assignment, committee attack assumptions, migration and final global commitment construction.

A global root combining shard roots is only a commitment. It does not prove that conflicting writes or atomic cross-shard calls were resolved correctly. An asynchronous receipt model is a developer-visible semantic change and cannot silently replace ordinary EVM calls. Preserve the standard synchronous path unless a separate owner-approved profile explicitly changes it.

Test cross-domain AMM failure, account nonce collisions, shard-owner loss during commit, coordinator replay, overlapping ownership and committee compromise below/above stated thresholds. Do not assume a small committee has the full global validator-set security. A successful experiment is not automatic authorization to replace the devnet protocol.

If no partitioning is needed to meet the measured target, document that evidence and do not add unnecessary complexity. If it is needed, implement and validate the required interfaces/experiments rather than leaving 'sharding later' as the supposed solution to an unmet 1M target.

## P07 — Required reports

Every target step 10k -> 50k -> 100k -> 250k -> 500k -> 1M records achieved finalized TPS, workload/gas/bytes, failure rate, latency, resource utilization, backlog slopes and correctness/recovery results. Continue profiling and improving failed steps; do not treat a generator reaching the offered rate as success.

Required artifacts: deployment topology/config, generator inputs/seeds, raw metrics, host specs, costs/capacity estimates labelled as estimates, report of limiting components, and exact reproduction commands. Hardware purchases require approval. Insufficient resources produce BLOCKED_INFRA with the minimum required experiment resources, while remaining software work continues.

The target is verified only when sustained/soak, bounded-backlog, availability and post-run recovery gates in plans 08 and 20 pass.
