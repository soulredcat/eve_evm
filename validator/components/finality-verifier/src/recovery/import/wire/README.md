<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Compact authenticated-import wire

Canonical owner: validator finality verifier's import transport codec. The codec
creates neither cryptographic authority nor an imported/replayed state capability.

Canonical bytes are `EVE_IMPORT_V1`, followed by four fields in this order:
ordered canonical state journal, canonical execution block RLP, native finalized
frame, and native lookahead/data frame. Each field has a u32 big-endian byte length.
The existing state/native codecs own those grammars; this wrapper does not replace
them. The complete compact payload ceiling is 4,198,312 bytes, matching the current
opaque recovery payload allowance. This does not fit every maximum valid block
plus delta/proofs; fragmented logical records remain mandatory separate work.

`measure_authenticated_import_wire` checks the exact aggregate before component
Vec construction. Journal measurement uses bounded existing version/system field
scratch; callers must charge it. Encoding preserves exact local parent identity,
including auxiliary digest/time, and carries no peer-selected expected target.

`preflight_authenticated_import_wire` borrows the immutable raw bytes, checks all
length/count/component boundaries before decoded allocation, and retains private
slices plus a frozen StateBudget. Exact journal decode/growth facts and execution
transaction/receipt counts/bytes, native signature counts, and lookahead data counts
are exposed for public admission. Detached slice/stat copies cannot mutate the
sealed preflight or authorize decoding substitute bytes. The borrow prevents raw
mutation while the preflight remains used; public retains its actual buffer lease.

`decode_authenticated_import_wire` accepts only that bound preflight, delegates to
the canonical decoders, and compares complete canonical reencoding to the exact raw
input. Preflight can reject structural/budget faults; full decoding still validates
typed canonical representations. Neither stage authenticates roots, certificates,
freshness, durability, correct execution, or the exact auxiliary representation.
Only the distinct prepared authenticated-import operation grants its stated result.

Public must reserve decoded journal/code/system/container memory, native/execution
copies, complete reencoding scratch, candidate/root work, retained state and queue
capacity before decode/application. Size/count facts are logical admission inputs,
not an allocator/RSS measurement or a heap guarantee. No cap is raised here, and no
full B4, fragmented persistence, snapshot, peer recovery or throughput pass follows.
