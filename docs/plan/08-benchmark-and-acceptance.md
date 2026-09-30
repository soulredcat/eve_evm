# 08 — Benchmark and acceptance policy

## What counts

Count unique, valid, successfully executed and finalized user transactions in the measurement window. Also report included-but-reverted, rejected, retransmitted and retried transactions separately. Internal calls, votes, simulation attempts, duplicate submissions and copies replicated to multiple nodes are not additional TPS.

Every result names commit, protocol/config digest, dependency versions, CPU, RAM, storage, filesystem, network topology, node count, validator voting power, workload, account/contract cardinality, contention, run duration and initial/final state size. Distinguish local multi-process tests from physically distributed deployments.

## Workloads

W0 native transfers; W1 ERC-20; W2 independent contract writes; W3 one hot pool; W4 atomic multi-pool transactions; W5 declared mixed-contract workload; W6 adversarial/overload. Publish each separately. A transfer benchmark cannot stand in for arbitrary smart-contract capacity.

Measure admitted/executed/finalized TPS, p50/p95/p99 latency, CPU/RAM, disk writes and amplification, state growth, network traffic, retries, re-execution, validator lag, master lag, snapshot lag and queue slopes. Report achieved offered load, not just a configured generator rate.

## Sustained gates

B10 requires a repeatable sustained run with warm-up, at least 60 minutes of measured steady load and a post-run recovery/replay check. A 24-hour soak is required before the strongest sustained 1M target claim. Short runs may guide optimization but cannot pass that target. Gates may be resource-blocked; they may not be weakened to obtain green status.

A complete 1M result requires bounded execution, consensus, availability, persistence and sync backlogs. A constantly falling-behind master is not a solved storage layer merely because it is off the hot path. State growth is expected; unexplained growth and unavailable retention capacity must be reported.

Additional validator replicas normally repeat verification and do not automatically multiply execution capacity. Measure scaling rather than summing nominal worker capacities. [20](20-test-vectors-and-acceptance.md) and [21](21-capacity-and-regional-scaling.md) define evidence and deployment details.
