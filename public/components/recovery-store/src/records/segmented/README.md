<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Segmented local recovery record codec

Canonical owner: public recovery storage. This codec performs no I/O, voting,
certificate verification, state execution, lease acquisition or logical-height
publication. It preserves existing opaque repository APIs and PublicBudget v1.
The explicit versioned part policy and orchestrator own their separate contracts.

Every payload begins with `EVE_SEGMENTED_RECOVERY_V1`, kind u8, schema u16 big-endian
1 and mode u8 1 (authenticated import). Unknown kind/version/mode rejects. Integers
have fixed widths; no alternative integer encodings or trailing bytes are accepted.
Logical identity has a full-body logical ID, opaque parent anchor and next target
height. The parent anchor is height u64, physical opaque cursor (40 bytes), and a
caller-selected local state binding (32 bytes). Its 80 bytes prove no finality.
Only height-zero/sequence-zero genesis bootstrap is supported by this initial codec.

A segment has a 177-byte header, exact data bytes and 32-byte SHA-256 footer. Header
fields are logical ID, parent anchor, target height, index/count u32, offset/total
length u64 and chunk length u32. Total overhead is 209 bytes. The fixed target
encoded payload ceiling is 4,194,304 bytes, below the existing opaque payload cap;
maximum chunk data is 4,194,095. Canonical chunks use the policy's maximum data size,
count ceil(total/chunk), offset index*chunk and exact final remainder. Six chunks
fit three aggregate 8 MiB data parts; the separate marker uses a fourth part.

A marker has a 193-byte header, ordered 40-byte opaque cursor references and a
32-byte SHA-256 footer: 225+40N, at most 465 bytes for six references. Header fields
are logical ID, parent anchor, target height, local target-state binding, total
length and reference count. References must be consecutive, nonzero and start
after the parent physical cursor; known incomplete ranges may precede them. The
orchestrator must verify actual segment membership, hashes, complete logical bytes,
state/proof provenance and successful sync before advancing any durable height.

Hashes use the pinned standard SHA-256 over the exact canonical header and data or
ordered references. Supplied hashes are checked, never normalized. A rehashed
attacker-chosen record remains untrusted. Logical full-body ID validation, namespace
binding, orphan/resume policy and actual recovery availability belong to the caller.

Borrowed preflight checks size, width, range, count, reference ordering and hashes
before decoded Vec allocation. It seals the same immutable bytes and copied limits.
Views/counts confer local integrity only. Direct writers require empty exact-capacity
caller-leased Vecs and use fixed header scratch, without intermediate payload Vecs.
Convenience encoders/decoders allocate and require separate capacity charges. This
is not an allocator/RSS or power-loss proof, complete segmented runtime, or B4 pass.
