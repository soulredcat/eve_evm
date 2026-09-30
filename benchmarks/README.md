# Benchmark implementation area

Planning only. No EVE TPS or latency result exists yet.

Implement W0–W6 profiles, generators, manifests and reports from plans [08](../docs/plan/08-benchmark-and-acceptance.md), [20](../docs/plan/20-test-vectors-and-acceptance.md) and [21](../docs/plan/21-capacity-and-regional-scaling.md).

Count unique valid finalized user transactions. Report reverts, failures, retries, replication and local emulation separately. Preserve workload seeds/configs, hardware/topology, raw measurements, state/IO/network growth, queue slopes and post-run recovery evidence. A smaller or easier changed workload is a different result, not an improved score on the original test.

Do not commit huge traces or databases; store checksummed manifests/references. Hardware limitations are explicit blockers, not permission to fabricate a 1M-TPS result.
