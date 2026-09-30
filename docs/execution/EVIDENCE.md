# Evidence index

## Current evidence

No runtime code, build, test, deployment, fault injection or benchmark has been executed as part of this planning package. There are no passing runtime evidence records yet.

The repository documentation is an implementation contract. Its existence is not evidence that consensus, EVM compatibility, storage recovery, tokenomics or 1M TPS works.

## Record format for future runs

Each actual run record must include:

- Run ID, bulk/task IDs and covered requirements/test IDs.
- Tested commit and dirty-diff digest if applicable.
- Genesis/config/dependency/fixture identity.
- Environment and exact topology; distinguish local emulation from real hosts.
- Exact commands, exit codes, selected/executed test counts and pass/fail/not-run results.
- Expected invariant and observed outcome.
- Sanitized artifact paths/URLs, checksums and reproduction command.
- Known limitations, failed cases and next repair action.

Large artifacts stay outside Git with a verifiable manifest/reference. Local-only artifacts must be identified as local-only. Do not invent CI links, log files, independent audit results or downloaded benchmark evidence.

A new source/config change invalidates evidence for affected behavior until the relevant gate is rerun. The integrator verifies this before a bulk becomes DONE.
