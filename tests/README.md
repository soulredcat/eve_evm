# Test implementation area

This directory currently contains planning guidance only. No tests have passed merely because this file exists.

Implement the traceability matrix and golden/property/integration/fault cases in [plan 20](../docs/plan/20-test-vectors-and-acceptance.md). B0 pins upstream fixture revisions, creates the executable gate manifest and rejects empty/missing test selections. Runtime modules may own unit tests; shared regression fixtures and cross-runtime tests belong here or in the documented integration workspace.

Save minimized counterexamples for real defects. Never make expected roots/signatures equal to unchecked outputs of the function under test. Keep production secrets and live database contents out of fixtures.

Implement the public-persistence and regional-master cases from plans [15](../docs/plan/15-network-and-sync-protocol.md) and [32](../docs/plan/32-regional-masters-and-public-persistence.md) as shared reproducible tests:

- T-N09 (B4): storage-worker/compaction/fsync stalls, bounded queues/resources/lag, no global RAM lock across IO, truthful durable markers and predeclared readiness transitions.
- T-N10 (B4): all masters offline, continued verified public persistence, public power-loss/unsynced-tail recovery from complete local records and authenticated durable peers, exact roots/receipts, and NOT_READY when required copies are missing.
- T-N11 (B6): healthy authenticated freshness before service RTT/throughput, explicit unknown freshness, rejection of invalid/wrong-network/height-profile-mismatched data, valid older replay history distinguished from a fresh head, and bounded probing/hysteresis/fallback without internal master inventory.
- T-N12 (B8): two-master partition/outage/catch-up produces identical authenticated history/roots at one height, verified public failover and no master consensus authority.

B0 registers these fixtures and their budget/measurement contracts; B9 reruns them after integration. Compare recovery with the serial oracle, record real retained-copy availability, distinguish simulated unsynced-write loss from hardware power-loss evidence, and retain every existing required test. These IDs currently have no executed evidence.

This is the shared, versioned test area. Required reproducible tests and sanitized deterministic fixtures must be available to every collaborator. Use English for test descriptions and documentation.

Put exploratory tests, debugging scripts, raw logs, disposable databases, and other local-only output in root `local-tests/`, which is ignored and must never be staged, committed, force-added, or pushed. Keep raw run artifacts and coverage reports in ignored output directories. Publish only compact reviewed evidence summaries and reproduction instructions under `docs/execution/`.

See [CONTRIBUTING.md](../CONTRIBUTING.md) for the absolute publication rules. Local experiments cannot replace a mandatory shared test or acceptance gate.
