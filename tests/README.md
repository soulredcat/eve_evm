# Test implementation area

This directory currently contains planning guidance only. No tests have passed merely because this file exists.

Implement the traceability matrix and golden/property/integration/fault cases in [plan 20](../docs/plan/20-test-vectors-and-acceptance.md). B0 pins upstream fixture revisions, creates the executable gate manifest and rejects empty/missing test selections. Runtime modules may own unit tests; shared regression fixtures and cross-runtime tests belong here or in the documented integration workspace.

Save minimized counterexamples for real defects. Never make expected roots/signatures equal to unchecked outputs of the function under test. Keep production secrets and live database contents out of fixtures.

This is the shared, versioned test area. Required reproducible tests and sanitized deterministic fixtures must be available to every collaborator. Use English for test descriptions and documentation.

Put exploratory tests, debugging scripts, raw logs, disposable databases, and other local-only output in root `local-tests/`, which is ignored and must never be staged, committed, force-added, or pushed. Keep raw run artifacts and coverage reports in ignored output directories. Publish only compact reviewed evidence summaries and reproduction instructions under `docs/execution/`.

See [CONTRIBUTING.md](../CONTRIBUTING.md) for the absolute publication rules. Local experiments cannot replace a mandatory shared test or acceptance gate.
