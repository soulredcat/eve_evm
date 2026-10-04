<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Actual public process-crash and validator-tail retrieval slice

The N10 case uses the existing actual four-validator Cluster, canonical native
certificate/history collection, independent replay and release-public follower
fixtures. It launches no master. A real nonempty transaction's receipt and actual
complete durable logical marker are observed before the owned public process is
terminated with SIGKILL after executable/PID-start identity validation.

Validators then finalize another nonempty nonce-1 transaction while public is
absent. The same public namespace restarts without deleting or rewriting valid
durable files. Its startup observation must restore at least the previously
confirmed prefix and remain below the later transaction's height. Subsequent
actual RPC receipt, complete roots, balance, nonce and durable physical/logical
progress are compared with replay. The first and second canonical transaction
commits must have sender nonce 1 and 2, and the independently replayed prefix
must remain byte-for-byte equal. Unknown freshness must remain not-ready.

The fixture delegates the original bounded public launch, private-output, RPC,
receipt and oracle operations; it copies no signing, authentication or HTTP parser.
EVE_PUBLIC_BINARY must identify the built release product executable; the ordinary
validator/native acceptance environment remains required. Each case uses one
task-owned contained ignored data directory, at most two launches and bounded
startup/receipt/durability waits. Missing binaries or assertions fail the case.

This is a one-host process-crash and later missing-tail download slice. It does
not demonstrate hardware power loss, independent-host availability, controlled
loss of an applied-but-unsynced tail, snapshot activation, retention guarantees,
freshness, throughput or complete B4 acceptance. No execution result is claimed
until the integrator runs the frozen case.

A stronger unsynced-tail-loss case needs an explicit bounded task-owned runtime
hook that pauses the isolated segmented writer immediately before marker sync,
reports the exact admitted/physical position, and resumes or terminates only the
owned public child. The current CLI exposes no such deterministic hook. The unit
fixtures' controlled writer pauses are not reachable by this external product
process case and cannot be relabeled as actual process evidence.
