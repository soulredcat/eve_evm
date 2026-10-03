<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Public applied RAM state: bounded empty-block development slice

Canonical owner: public sync application and persistence orchestration. This
module uses validator-owned canonical execution and authenticated recovery; it
does not implement consensus, grant master authority, or depend on private master
code. It remains an incomplete first B4 slice.

The service currently accepts only records with empty execution and empty
lookahead transaction lists. Borrowed canonical preflight rejects nonempty lists
before decoded-envelope allocation. The canonical recovery verifier and executor
retain their existing full transaction capability. This service limitation is a
local implementation boundary, not a consensus rule or complete B4 acceptance.

`open_applied_state_service` owns namespace identity selection and opening. It
reconstructs local canonical genesis and replays every complete retained record
before starting the sole record writer. Foreign genesis, unverifiable opaque
history and incomplete replay refuse startup without resetting the namespace.
Opening and replay reserve a separate logical read estimate using the actual
repository read ceiling. No public RAM lock spans repository I/O or execution.

`try_apply_recovery_bytes` consumes borrowed ingress bytes; the ingress owner must
separately account its existing buffer. The service reserves an exact handoff
buffer, copies the bytes once, then canonically decodes and verifies that same
buffer under a working lease. Proof, execution, resource and queue failure leave
the accepted publication and admitted cursor unchanged. Queue age is checked
again immediately before final admission. The short publication guard permits
only bounded immediate admission/bookkeeping; the worker's reply channel and ID
entry may allocate there, but no disk/network wait or replay occurs there.

Readers capture one immutable publication containing state, H/H+1 authentication,
and distinct applied/authenticated/finalized/durable markers and exact cursors.
State charges remain attached to the private shared generation until its final
captured owner drops. Reader APIs expose borrowed state/anchor access, without a
naked uncharged recovery-state Arc. These markers do not establish independently
known head freshness, remote tail availability, or current-head readiness.

Every pending record retains its original charged payload through successful
ordered acknowledgement or failure. A prospective cursor is admission metadata;
only the actual ticket's exact next synced acknowledgement advances the durable
prefix. A failed acknowledgement fences new application and preserves the tail.
Shutdown joins the writer outside RAM locks, reconciles available tickets and
returns an owned `RetainedAppliedTail`, including its queue metadata charge. It
never equates worker termination with authenticated durability.

## Estimated resource model

Checked arithmetic uses StateBudget maxima A/S/K/C/N/Y/H/Jn/Jb/W/Bc for account,
slot, code count/bytes, system count/bytes, hash count, journal count/bytes, state
bytes and commit bytes, and local service payload cap L:

```text
R = 2 MiB + 512 A + 256 S + 256 K + C + 512 N + 2 Y + 128 H + 2 L
D = 4 MiB + 16 L
P = 4 W + 2 Bc + 256 (A + S + K + N + H)
J = 2 Jb + 512 Jn + 256 (A + S)
replay = actual canonical parent clone estimate + 2 R + D + P + J
repository read = 2 maximum_read_bytes + 4096
queue metadata(count) = 4096 + count (sizeof(PendingRecord) + 8192)
```

One R transfers to the retained generation. Transient replay/read charges remain
alive through their owned data lifetimes. Queue-count metadata reserves before
owner allocation and remains with an unacknowledged shutdown tail. Polling reserves
its additional retirement queue before allocation, then drops retired payloads
outside the publication guard. Configuration requires the retained parent, worst
replay/read and both queue metadata envelopes to fit the public working pool.
Actual replay calls the canonical executor estimator; its configuration ceiling
also belongs to the canonical executor, avoiding copied cost coefficients.

The unchanged development StateBudget ceiling exceeds the unchanged 256 MiB
public working pool and refuses this service configuration. Tests use an explicit
smaller local StateBudget and payload cap. Resource refusal is local backpressure;
it never changes a block's consensus validity or silently raises public limits.

These are conservative logical estimates. They do not prove allocator/RSS/CPU,
OS-cache, physical I/O, whole-process enforcement, or zero storage interference.
Nonempty execution growth is intentionally outside this service slice. Full
transaction application, fragmented large-record storage, authenticated peer-tail
recovery, fresh-head readiness, snapshot/retention orchestration and measured
T-N09/T-N10 acceptance remain unfinished.

Tests exercise real signed empty certificates, canonical execution, paused
repository append with two admissions and independent RAM capture, ordered sync
and exact reopen, failed decode/proof/execution/admission, retained-reader resource
leases, final-admission age refusal and unverifiable-prefix rejection. The failed
tail case deliberately corrupts a private control cursor in a unit fixture to
trigger a real repository refusal; it is not a hardware fsync or power-loss test.
No verification result is claimed by this contract document alone.
