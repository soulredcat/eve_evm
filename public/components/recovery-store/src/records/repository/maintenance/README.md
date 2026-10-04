<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Opaque KEEP_ALL maintenance

Canonical owner: public recovery storage. The mutable repository handle owns one
exclusive RocksDB namespace. Compaction requests bind its actual immutable
identity and acknowledged physical head, declare a finite retained-record ceiling,
and require caller-leased read capacity before maintenance I/O begins.

The operation reuses the canonical whole-namespace validator before and after
RocksDB WAL synchronization, a waiting memtable flush, and maintained full-range
compaction. Every retained payload, parent/hash, schema/identity key and actual
disk head must remain intact and equal to the previous acknowledged prefix. Any
validation/sync failure fences the handle. Admission rejection does not mutate it.

No keys are deleted, no history is pruned, and no replacement WAL is introduced.
At retained capacity, append refuses further growth while existing recovery rows
remain available. Keeping data locally is the initial recovery policy; a small
development fixture capacity demonstrates refusal, not a production retention
window or independently operated source availability.

The pinned safe RocksDB `compact_range` call has no returned status. The operation
reports only successful flush calls and unchanged validated retained data after
the call; it does not assert bytes reclaimed, successful hardware power-loss
recovery, or a new durable/finalized height. Caller scheduling must keep this
maintenance outside RAM execution/query locks. Numeric reservation is admission,
not a real lease, allocator/RSS bound or CPU/disk interference measurement.
