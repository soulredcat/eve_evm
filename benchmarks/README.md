<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Benchmark implementation area

Planning only. No EVE TPS or latency result exists yet.

Use the shared [performance contract](../docs/performance/README.md) and
[testing contract](../docs/testing/README.md) to implement W0–W6 generators,
manifests and reports. Detailed owner plans remain local in their existing paths;
the public contracts expose the required measurement and evidence boundaries.

Count unique valid finalized user transactions. Report reverts, failures, retries, replication and local emulation separately. Preserve workload seeds/configs, hardware/topology, raw measurements, state/IO/network growth, queue slopes and post-run recovery evidence. A smaller or easier changed workload is a different result, not an improved score on the original test.

B0 records a bounded dependency/storage spike and freezes public persistence budgets without requiring the future integrated runtime. B4/B6 establish correctness/resource gates; B10 measures RAM application concurrently with ordered durable storage, checkpoint export, compaction and source failover. Fix queue byte/item/age limits, lag/readiness thresholds, checkpoint memory and CPU/IO budgets before measuring.

Report service RTT and transfer throughput alongside verified-source freshness; finalized/applied/durable/authenticated-state/checkpoint watermarks; durable bytes/s, fsync latency, write amplification, queue slope/age, disk headroom and storage interference with execution/RPC latency. Compare identical workloads/configurations, account for every durable full-replica stream and distinguish one/two real masters from a ten-master projection. Private gateway routing and more replicas are not multiplied chain TPS.

Run master-offline recovery and storage-stall scenarios alongside T-N09–T-N12 in shared tests. Asynchronous persistence still consumes CPU, memory, IO and network resources; never promise zero interference, durable queued RAM or complete power-loss recovery when verified retained copies are missing.

Keep raw benchmark output in ignored `artifacts/` or `local-tests/`; publish only compact reviewed summaries, manifests/checksums and reproduction references under `docs/execution/`. Required benchmark generators and sanitized fixtures stay versioned. Hardware limitations are explicit blockers, not permission to fabricate a 1M-TPS result.
