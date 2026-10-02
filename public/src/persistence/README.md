<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Public persistence handoff

Canonical owner: public runtime persistence orchestration. The handoff primitive
reserves actual immutable encoded payload capacity and one batch slot before
allocation/application. It grants no finality, execution verification, successful
enqueue, storage acknowledgement or durability. The storage-only worker is
described below; verified follower integration remains separate B4 work.

`create_handoff_pool` validates the existing versioned `PublicBudget`.
`reserve_recovery_payload` atomically reserves count and logical bytes, then
allocates outside the accounting lock. Reservation is immediate: unavailable
capacity returns an error without waiting for disk or queue progress. Call before
RAM publication and outside applied-view locks. One reservation contains one
encoded recovery record/batch; its length must obey both limits.

`write_reserved_payload` appends bounded encoding fragments without buffer growth.
The caller must include the complete serialized envelope, proofs, references and
replay metadata in the reserved length. No caller-supplied accounting estimate or
existing large `StateCommit` is accepted as an implicitly small payload.
`seal_recovery_payload` requires exact completion and freezes the existing buffer
without copying it. Clones share one buffer and lease. Construction, pending
queue, active disk work and retained readers remain charged until the last owner
drops. Cancellation drops an unfinished reservation and releases its exact charge.
Storage code must retain the payload through real I/O completion even when an
awaiting caller times out. `observe_handoff` includes all retained reservations
and their oldest monotonic age; readiness/age enforcement belongs to the runtime.

Covered limits are encoded byte capacity, retained batch count and observed age.
Allocator rounding/metadata, Arc/accounting overhead, upstream encoding scratch,
caller input buffers, state clones, RocksDB buffers/write amplification, OS caches,
worker CPU and whole-process RSS need separate bounds and measurement. An allocator
reporting unexpected logical Vec spare capacity is rejected. This does not claim
an OS-enforced memory cap, zero overhead or T-N09/T-N10 acceptance.

The durable `StateService` and synchronous development producer retain their
existing semantics. Public applied RAM views, authenticated compact replay,
durable markers and master-independent recovery are not implemented by the
handoff resource primitive.

## Dedicated opaque record worker

`worker/` moves an exclusive `OpaqueRecordRepository` into one writer thread.
It binds to the same `Arc<HandoffPool>` as the applied-state handoff; foreign-pool
payloads reject with ownership returned. Channel capacity comes from that validated
pool's batch count. An outstanding reservation ID may be submitted only once;
cloning one buffer cannot create additional uncharged queued batches. Exact replay
after the previous I/O completes remains supported.

`try_submit_record` is immediate and returns the original payload on rejection.
Each ticket receives at most one small opaque synced acknowledgement. A cancelled
ticket does not cancel admitted disk work or release its payload early. Storage
failure latches admission failure; caller-visible errors are fixed typed categories.
`finish_record_worker` consumes the sole sender, drains admitted work and joins the
thread outside applied-state locks. No cloned sender API is exposed. Operators
must explicitly finish a worker; dropping its handle is not a flush acknowledgement.

The worker borrows the original Vec as a record slice, avoiding an additional
caller-level payload copy. Before I/O it reserves a conservative logical staging
envelope using actual repository limits:

```text
2 * payload_length + 88 + repository.maximum_batch_bytes
  + 2 * repository.maximum_read_bytes + 4096
```

This accounts for the current record clone, encoded payload, bounded RocksDB
WriteBatch, possible retained-record decode/read during exact replay and fixed
logical framing allowance. Checked arithmetic rejects overflow. The separately
declared scratch ceiling must fit the unallocated allowance after all existing
PublicBudget v1 pools; configured repository cache/write-buffer/background-job/file
ceilings must also fit that public profile. `observe_record_worker` reports active
staging charge and failures. Allocation rounding, WAL/page caches, database reads,
allocator metadata, physical RSS and CPU still need independent measurement.

The existing default opaque repository has a smaller record ceiling than the
public 8 MiB payload limit and a larger file ceiling than the public profile.
Those configurations cannot silently be called compatible. The worker rejects
an oversized record or incompatible DB knobs without changing global policies.
Tests use an explicitly declared 32-file repository fixture and a 40 MiB staging
ceiling; they also check rejection of the unmodified incompatible file ceiling
and an 8 MiB payload against the actual smaller repository record limit.

Worker tests use real WAL-synced repositories, exact replay and reopen. A test-only
pause before append demonstrates retained ownership after ticket cancellation;
it is not a hardware disk-stall or power-loss experiment. The worker accepts
opaque bytes and returns opaque acknowledgements only. Root's verified recovery
envelope must authorize replay and higher-level watermarks. No worker cursor is
itself finality, authenticated post-state or complete durable-recovery coverage.
