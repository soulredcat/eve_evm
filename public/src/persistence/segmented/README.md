<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Version 2 segmented part persistence

Public owns this bounded lease/worker orchestration. It uses public recovery-store
codecs and node-policy contracts without changing v1 handoff, applied state, master
authority or canonical execution/finality semantics. This primitive alone completes
no authenticated recovery, Source readiness, power-loss or full B4 gate.

`SegmentedRecoveryPolicy` explicitly counts retained parts. Each DataPart has one or
two encoded segment Vecs; MarkerPart has one nonempty encoded Vec. The unused array
entry has zero capacity. Four parts, 8 MiB per part and 32 MiB total keep their new
version 2 meanings; they do not rename version 1 batches or permit a multi-record DB
transaction. The actual repository profile must have maximum_batch_records=1.

Planning uses bounded fixed arrays. `plan_segmented_layout` binds identity/length without a target state binding. `reserve_segmented_layout` acquires all capacity before candidate preparation; `bind_segmented_reservation` introduces the actual verified local target later. Its rejected result returns the original unbound reservation. No provisional target binding is serialized. Existing `reserve_segmented_batch` delegates to the same reservation operation and atomically acquires
ALL part slots, exact encoded capacities and a distinct count/type-derived metadata
lease before any output allocation. Allocation happens outside the accounting lock;
unexpected Vec spare capacity rejects. Canonical writers fill the reserved buffers
directly, without another complete output Vec. Sealing verifies the whole body SHA identity before filling any buffer; a same-length substituted body refuses. Cancellation releases unfinished
parts and metadata; immutable sealed clones share those same buffers and leases.
Caller logical input allocation remains separately charged.

The largest declared 21,025,569-byte logical record requires six data segments, three
DataParts and a MarkerPart, retaining 21,027,288 encoded payload bytes. Segment header
plus hash overhead is 209 bytes; two 4,194,304-byte encoded segments fit one 8 MiB
part. Opaque wrapping adds 88 bytes per separately appended record, giving a maximum
physical segment record of 4,194,392 bytes. A six-reference marker is 465 payload
bytes, or 553 with opaque framing.

The pool is bound to a namespace and actual repository budget. Worker startup
acquires a pool-owned single-worker lifetime lease before any startup read or spawn.
WorkerState retains that lease across the thread, handle and tickets. A second
worker using the same pool is refused even with another database path. Finishing
a handle does not release this fence while a ticket still owns WorkerState;
startup failure or the final state-owner drop releases it. This keeps one scratch
envelope inside the declared auxiliary allowance. It is a local resource fence,
not distributed database/key fencing or finality authority. Startup checks
authoritative namespace/budget, actual physical head and separately supplied logical
parent. Height zero must match the authoritative bootstrap cursor; a nonzero parent
must match a real retained marker row/hash/height/local state binding. Startup checks
all referenced segment rows and their physical hashes, ordered indices/counts,
canonical offsets and shared identity. The marker follows its final reference
immediately; future or mismatched references cannot establish a local prefix.
Pool policy and codec encoded-payload ceilings must agree before allocation.
Admission conservatively bounds occupied unvalidated logical slots by the declared
logical lag limit, using no physical sequence as a height.
These checks establish local storage membership, without authenticating the
caller's state/finality. Known
orphan segments may precede new data without changing the logical parent.

Admission checks pool ownership, exact planned physical/logical parent, duplicate
batch ID, available bounded slot and exact Duration queue age immediately before
channel admission. Rejection returns the original charged batch and leaves admitted
positions unchanged. The sole worker appends every segment in one real WAL/sync
transaction and verifies the actual exact acknowledgement. Only after all data
acknowledgements match does it append the presealed marker. Logical success exposes
logical target height separately from marker physical cursor, exact references and
database sequence. It grants neither execution validation nor consensus finality.

Handle-owned WorkerState retains bounded full-batch Arc slots through I/O, cancelled
receivers and thread panic. A marker success stays charged until logical ticket
validation removes the admitted slot. Shutdown returns owned remaining tails,
including completed but unvalidated markers, without silently discarding the only
RAM tail. Existing tickets may still validate after join; returned tail owners must
be explicitly reconciled/released. Failure latches admission, returns no success
marker, and reports only the last acknowledged physical cursor. Ambiguous storage
or panic tails require reopen/reconciliation; they cannot advance a logical marker.

Encoded parts charge the existing 32 MiB queue envelope. Distinct metadata and
scratch provision fit the unchanged unused base allowance. Metadata sizing uses
actual Rust descriptor/array/reference sizes plus explicit logical channel/control
allowances. Fixed four part slots, six references and two admitted batch slots avoid
unbounded maps and queues. These estimates are neither allocator nor RSS measurements.

Before each repository operation, a real scratch lease reserves:

```text
2 * payload_length + 88 + repository.maximum_batch_bytes
  + 2 * repository.maximum_read_bytes + 4096
```

For a maximum segment under the actual 16 MiB batch and 4,198,400-byte read ceilings,
this is 33,566,808 bytes. The declared scratch limit is 40 MiB, alongside metadata
inside the existing 60 MiB auxiliary allowance. The v1 8 MiB payload/batch envelope
is not presented as a worker peak. Scratch RAII releases on normal return/unwind;
allocator/database/OS-cache, CPU, physical I/O and shared-host interference still
require independent measurements.

Tests author actual one-record sync/reopen, six segments before marker, paused two
logical admissions, orphan/marker position checks, atomic reservation/refusal,
captured leases, cancelled tickets, actual retention-capacity refusal and deliberate
unit-only thread panic. They are structural persistence fixtures, without EVM or
hardware fsync/power-loss claims. Applied import integration, authenticated recovery,
retention/pruning, snapshots, independent peers, readiness and T-N09/T-N10 remain
required subsequent work. Verification outcomes belong to root's recorded gates.
