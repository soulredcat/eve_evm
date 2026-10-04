<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Borrowed full-commit materialization admission

The validator-owned canonical state codec owns this preflight. It scans one
immutable EVE_STATE_COMMIT_V1 input without materializing state, code, network
Strings, headers or receipts. StateCommitPreflight has private fields and freezes
the exact input borrow and StateBudget copy. Detached statistics and a changed
caller budget cannot decode a substitute buffer.

The scan validates bounded canonical RLP framing, schema/optional topology,
primitive widths/UTF-8 and strict map ordering. Account, storage, code, system and
history keys must increase, rejecting duplicates without allocating lookup maps.
Code/system aggregate limits count the actual borrowed bytes. System record leaf
scanning reuses the existing bounded helper; header scanning bounds byte-field
topology and total encoded size without constructing alloy Header.

Actual statistics report account/slot/code/history/system counts, total/largest
code bytes, system encoded/payload/leaf sizes, each network payload, header leaf
sizes, raw transaction/receipt counts and lengths, and conservative Vec scaffolding.
These facts authenticate neither state, execution, roots nor finality. System
schema/key semantics, header/receipt decoding, code hashes and complete-state/root
validation stay in the maintained canonical decoder.

decode_preflight_state_commit accepts only the sealed preflight and delegates the
existing decode_state_commit, including canonical full reencoding. Required decode
reservation uses actual counts/map/blob allowances plus six complete encoded-byte
envelopes and bounded scratch for roots, nested serialization and output copies.
The caller must reserve real capacity before decoding, separately retaining its
input/assembly allocation and old captured state. No numeric estimate grants a
lease, trusted checkpoint, allocator/RSS guarantee or safety bypass.

## Source allocation audit

Borrowed preflight paths use stack Option keys/scalars, fixed-width Decodable
values, StateBudget/Copy statistics, checked integer arithmetic, borrowed RLP
payloads and the existing fixed three-slot system-leaf stack. They contain no
Vec/BTreeMap/String construction, clone of owned payload, reencoding, Header or
receipt materialization. encode/decode work begins only in the explicit sealed
decoder wrapper after caller admission. Heap allocation-counter measurement is
NOT_RUN; source inspection is not presented as measured zero allocations.

Tests compare maintained-codec parity, exact counts and same input pointers,
frozen budget behavior, exact/one-lower limits, every map's duplicate/order cases,
truncation/schema/width/UTF-8, raw execution limits and bounded header/system trees.
Structural malformed payloads are not execution/finality fixtures. Snapshot wire
staging, authenticated proof capabilities, durable base/activation and full B4
acceptance remain separate requirements.
