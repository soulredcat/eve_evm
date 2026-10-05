<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Public applied RAM state: explicit replay and authenticated import

Canonical owner: public sync application and persistence orchestration. This module
uses validator-owned execution, state and finality contracts, without implementing
consensus, granting master authority, or importing private master behavior. Complete
B4 acceptance remains unfinished.

`AppliedMode::EmptyReplay` independently executes the existing empty-execution and
empty-lookahead service slice. Borrowed canonical preflight rejects nonempty lists
before decoded allocation. Canonical replay/execution retain their full transaction
capability; the service restriction is a local implementation boundary.

`AppliedMode::AuthenticatedImport` applies an ordered journal to a private reserved
candidate and authenticates its EVM/system roots and execution hash through actual
H/H+1 certificates. It accepts nonempty block/receipt payloads without REVM or a
second fee update. Private imported and replayed capabilities remain distinct;
`applied_mode` exposes their authority. Import trusts the declared classical BFT
fault model and cannot independently detect an execution error deliberately
certified outside it. Unused code and local content digest remain checked auxiliary
representation, without a separate certificate of exact auxiliary contents.

`open_applied_state_service` delegates explicitly to legacy `EmptyReplay`.
`open_applied_state_service_with_mode` selects either capability. It constructs local
canonical genesis and verifies every retained record before starting its sole
writer. Foreign identity, unverifiable opaque history and incomplete recovery refuse
startup without namespace reset. Opening/recovery reserves the actual repository
read ceiling; no RAM publication lock spans storage I/O, proof work or execution.

Replay keeps the configured legacy namespace. Import preserves configured genesis
and owner, deriving only its effective storage domain with
`SHA256("EVE_PUBLIC_IMPORT_DOMAIN_V1" || configured_domain[32])`. The base identity
is preserved; repository opening and prospective cursors use the effective identity.
Existing namespace metadata rejects wrong-mode reopening even at genesis. This local
mode binding grants no finality authority. Wire failure never falls back to the
other mode's decoder.

`try_apply_recovery_bytes` takes borrowed ingress bytes whose existing buffer needs
its caller's separate charge. It reserves an exact handoff Vec, copies bytes once,
then canonically decodes and prepares from that same immutable payload under real
working leases. Failed preflight/proof/application/resource/queue admission leaves
the accepted publication and admitted cursor unchanged. Final admission checks exact
Duration queue age again. Its short publication guard permits bounded immediate
worker channel/ID bookkeeping and pointer changes, without disk/network waits or
replay; worker bookkeeping can allocate inside that guard.

Readers capture one immutable coherent state, anchor, mode, applied/authenticated/
finalized/durable markers and exact cursors. The private generation retains its
estimated state lease until its final captured owner drops. Borrowed getters expose
neither a naked unleased state Arc nor a conversion between authority modes. These
markers establish no independently known fresh head or remote recovery availability.

Each pending ticket retains the original charged recovery payload through verified
ordered acknowledgement or failure. A prospective cursor is admission metadata;
only the actual exact next synced acknowledgement advances the durable prefix.
Failure fences new application and preserves its tail. Shutdown joins outside RAM
locks, reconciles tickets, and returns an owned `RetainedAppliedTail`, including its
metadata charge. Worker termination alone is no authenticated durability statement.

## Replay estimates and shared resources

Using StateBudget maxima A/S/K/C/N/Y/H/Jn/Jb/W/Bc for account/slot/code-count/code-bytes/
system-count/system-bytes/hash-count/journal-count/journal-bytes/state-bytes/commit-bytes,
and local payload cap L, checked estimates are:

```text
R = 2 MiB + 512 A + 256 S + 256 K + C + 512 N + 2 Y + 128 H + 2 L
D = 4 MiB + 16 L
P = 4 W + 2 Bc + 256 (A + S + K + N + H)
J = 2 Jb + 512 Jn + 256 (A + S)
replay = actual canonical parent clone estimate + 2 R + D + P + J
repository read = 2 maximum_read_bytes + 4096
queue metadata(count) = 4096 + count (sizeof(PendingRecord) + 8192)
```

One R transfers to the captured generation. Read/transient charges outlive their
owned allocations. Count-derived queue metadata reserves before owner allocation
and transfers with unacknowledged shutdown data. Polling reserves retirement storage
before allocation and drops retired payloads after releasing the publication guard.
Replay configuration requires the retained parent, worst replay/read and both queue
metadata envelopes to fit. Both actual-parent and configuration clone costs belong
to the canonical executor, without copied cost coefficients.

The unchanged default StateBudget replay ceiling exceeds the unchanged 256 MiB
public working pool. Replay tests use an explicit smaller local budget and payload
cap; refusal is local backpressure, without changing consensus validity or raising
public limits. Nonempty independent execution growth remains outside this service
mode.

## Actual-count import estimates

Import reserves `BOUNDED_STATE_CODEC_SCRATCH_BYTES` before actual genesis/parent
sizing. Genesis sizing uses the canonical helper's borrowed accounts/code/system
inputs, without a REVM/default-max charge. Its conservative initialization total
stays charged with the captured genesis generation.

Import wire preflight is sealed to the exact charged immutable bytes and frozen
budget. Decode accepts only that preflight, without substitute input or budget.
Before decode/preparation, C comes from the canonical actual-parent/journal wire-count
candidate estimator, sharing the typed estimator's cost model. Let V be execution
transaction/receipt plus lookahead element count, N the two native frames' encoded
bytes, S their signature count, and X all raw execution/lookahead transaction and
receipt bytes:

```text
R_import = C + X + 2 V sizeof(Bytes) + 8 N + 1024 S + 128 KiB
```

The complete estimate adds another C, exact journal operation Vec element bytes,
copied code/system payloads, leaf and byte-field scaffolding, copied parent network
bytes, canonical codec scratch, execution/native/lookahead wire copies, native
transaction Vec/data copies and signature metadata. All arithmetic is checked;
counts/lengths come from actual preflight. The lease precedes wire decode and
candidate/native copies; only R_import remains with the imported generation.
Sizing scratch stays alive through canonical preparation. Raw handoff, caller
ingress, repository reads and queue metadata retain separate charges.

Import configuration checks static limits. Actual initialization/application must
fit the working pool alongside retained views. Default StateBudget with small
genesis can start import without raising the 256 MiB working or 512 MiB process
target. An oversized actual candidate refuses before decode.

`EVE_IMPORT_V1` is compact-only, capped at 4,198,312 bytes. Some valid maximum-size
protocol records require fragmented storage; snapshot/retention orchestration,
authenticated peer-tail retrieval, fresh-head readiness and measured T-N09/T-N10
remain unfinished. These conservative logical estimates prove no allocator/RSS,
CPU, OS-cache, physical I/O, whole-process cap, zero interference, PQ protection
or full B4 acceptance.

Tests compare actual signed nonempty import against independent canonical execution,
fees, roots and receipts. Paused append, two admissions, RAM capture and reopen
exercise exact prefix and held-view accounting. Failure/age/mode tests preserve
state/cursors and charged tails. Storage rejection fixtures deliberately corrupt
private control cursors; they are not hardware fsync or power-loss experiments.
No verification result is claimed by this contract document alone.
