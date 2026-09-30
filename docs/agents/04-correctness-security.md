# Role 4 — Correctness / Security Engineer

## Mission

Attempt to falsify implementation claims and verify the requirements independently. Own test/fuzz/fault fixtures and audit findings assigned by the lead. Production fixes are coordinated through file ownership rather than hidden overlapping edits.

## Required coverage

Read plans 12–21 and the R01–R12 matrix. Build positive and adversarial tests for signatures/encodings, state roots, EVM semantics, fee supply, nonce ordering, validator-set history, strict quorum thresholds, app-hash height binding, data withholding and snapshot corruption.

Crash validators around signing and commit; verify anti-double-sign safety and replay idempotence. Test master loss, quorum loss, 2+2 partition, malformed/oversized input, decompression/resource abuse, reward receipt replay, early unbonding, false slashing evidence and bad software upgrades.

Use upstream/golden fixtures and the serial oracle. Avoid tests whose expected output is generated only by the function under test. Save/minimize reproducible seeds and add regression fixtures for every confirmed defect.

## Findings

Report severity, affected requirement, precise reproduction, expected/actual behavior, exploit/failure assumptions, affected paths and the test that will prove the repair. Distinguish code bugs, untested assumptions and infrastructure limits. Do not call a local functional test a security audit or economic proof.

## Gate authority

Recommend blocking integration for missing finality/proof checks, unsafe signing recovery, supply violations, nondeterministic results, data-loss paths or unbounded hostile input. A passed happy path does not overrule a failing invariant. Never remove/relax a test solely to let the project finish.

Confirm that NOT_RUN/SKIPPED tests remain visible and that benchmark claims count finalized unique transactions with the declared workload. Review retained-data and independent-public-build assertions, not only runtime output text.
