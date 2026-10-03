<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Performance contract and measurement

The target is **1,000,000 aggregate unique finalized user transactions per second**.
It remains `NOT_ACHIEVED`. No EVE capacity or latency benchmark is published yet.
Test execution times are verification durations, not transaction throughput.
The [benchmark area](../../benchmarks/README.md) retains implementation scope;
[execution status](../execution/README.md) records actual verified progress.

## Required workloads

| Profile | Workload |
|---|---|
| W0 | Native transfers |
| W1 | ERC-20 transfers |
| W2 | Independent contract storage updates |
| W3 | One hot AMM pool with real write contention |
| W4 | Atomic multi-pool execution |
| W5 | Mixed contracts and transfers |
| W6 | Adversarial input and overload |

Default W5 is 20% native transfers, 30% ERC-20 transfers, 20% independent writes,
20% AMM swaps and 10% atomic two-pool operations. Declare at least 100,000 funded
sender identities and 1,024 pools for the low-contention mixed profile, with exact
seed, gas distribution, token-sharing graph and hot-key skew. W3 separately tests
one pool. Changing contention/cardinality produces a different workload result.
AMM fixtures test EVM behavior; no production liquidity or external-chain program
is required by this measurement contract.

Count each valid user transaction once at actual finality. Disclose reverts,
failures, retries, duplicate submissions, votes and replica copies separately.
Ingress rates, executor loops, roots and sums across full replicas cannot prove
the target. Transfer-only evidence cannot establish arbitrary mixed-EVM capacity.

## Measurements that must agree

Fix the workload, topology, active security profile, resource budgets, latency SLO
and error budget in the run manifest before execution. The development default
is p99 admission-to-finality latency <= 5 seconds outside deliberately injected
faults. Declare the SLO and fault profile before measurement. This is a development
default, not a measured result, and does not alter R12/mainnet acceptance.

[R12](../../config/gates/requirements/core.toml)/1M `SCALE_TARGET_VERIFIED`
acceptance requires a passing mixed-EVM W5 run of at least 60 measured sustained
minutes after warm-up **and a 24-hour soak**. Both are mandatory, alongside the
active secure profile, bounded backlog and post-run deterministic verification
and recovery evidence. Retain W0–W6 results, including the separate W3 test of one
hot pool, with the predeclared workload cardinality and contention.

Report:

- Finalized TPS and p50/p95/p99 admission-to-finality latency.
- Execution-applied, recovery-durable, authenticated-state and checkpoint rates
  and heights, without substituting one for another.
- Queue bytes/count/age and slope, durable lag, backpressure and error rates.
- RAM footprint, CPU, network traffic, useful RTT/throughput and catch-up load.
- Disk bytes, fsync latency, WAL/compaction, write amplification, retention and
  snapshot/catch-up headroom for every full replica stream.
- Interference with execution/RPC under real persistence, snapshot, indexing and
  failure work; no claim of zero storage overhead from thread separation.

Increasing public ingress nodes, validators or master replicas does not multiply
unique chain TPS. Zone IDs and nearby sources are routing metadata. Masters stay
off the mandatory transaction path but still need storage and catch-up capacity.

For average wire bytes `b`, durable bytes `d`, gas `g` and target rate `R`:

```text
one-copy wire rate = R * b bytes/second
one-copy durable rate = R * d bytes/second
daily raw wire bytes = R * b * 86400
required execution gas rate = R * g
```

Measure the actual workload inputs; these formulas are capacity planning, not
benchmark evidence. Replication, indexes, protocol overhead and compression are
additional measured quantities.

## Secure and recoverable acceptance

Correctness, serial/parallel equivalence, durable signing, authenticated data,
fee conservation and resource gates remain enabled. Classical-only or security-
disabled runs are diagnostic and cannot close the secured target. SEC1/SEC3
acceptance is mandatory for that claim. Master-offline recovery and bounded
storage/retention obligations remain required; a fast RAM-only run is insufficient.

B10/B11 measurement tooling and completed target acceptance remain unfinished.
Do not advertise a future `xtask bench` command as an implemented successful run.
Insufficient capacity is `TARGET_UNMET`; unavailable required hardware is
`BLOCKED_INFRA`, with reproducible deployment/workload instructions retained.

Publish reviewed compact workload/config manifests, source/tool identities,
commands, hardware/topology, summary metrics, outcomes and limitations in allowed
READMEs. Store raw metrics/traces/databases under ignored local storage with
explicit local-only references. Never publish keys or personal configuration.
