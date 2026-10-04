<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Development checkpoint query messages

Canonical owner: public recovery storage's uncompressed checkpoint transport.
This binary V1 codec carries untrusted snapshot content and execution metadata,
using maintained canonical state/version/block codecs. It carries no native
certificate, signature, verifier state, finality, freshness or activation grant.

Requests use `EVE_CHECKPOINT_REQUEST_V1`, version byte 1, compression byte 0,
kind byte, u32-length-prefixed canonical locally known genesis StateVersion and
a positive u64 height no greater than i64::MAX. Manifest kind 1 adds u32 chunk
width. Chunk kind 2 adds width, fixed 32-byte manifest ID and u32 index. Execution
kind 3 ends at height. Width is 1..4 MiB and index is below 4096. A request is
limited to 1 MiB; its genesis is capped at 4096 bytes and all fixed fields are
validated before owned metadata decode. Only exact locally configured genesis
equality is eligible at the server. Genesis bytes are a constraint, not peer trust.

Responses use `EVE_CHECKPOINT_RESPONSE_V1` and the same version/compression/kind
bytes, followed by length-prefixed target StateVersion. Manifest responses add a
canonical durable-tip StateVersion, fixed manifest ID and bounded content manifest.
The ID is SHA-256 of the exact manifest and is verified. The tip is an untrusted
local observation, not independently corroborated freshness. Chunk responses add
manifest ID, whole body SHA, total length, index, width and bounded chunk bytes.
Index and width determine offset; every chunk has the full requested width except
the final tail. Execution responses add only canonical BlockPayload metadata.
The native frame and certificate must be fetched separately and authenticated by
the checkpoint verifier under local policy.

Sealed borrowed response preflight freezes the exact bytes and copied StateBudget
and transport limits. It scans canonical version grammar and network byte counts,
manifest framing/identity, chunk lengths/counts and all envelope boundaries before
allocation. Execution block bytes are bounded before the maintained owned state
decoder checks their detailed grammar and canonical reencoding. The checked decode
charge is 128 times encoded bytes, 2 MiB scratch, and four actual version network
String byte envelopes. The caller holds a real capacity lease before decode and
until all owned responses and copies drop; this numeric charge is not a lease or
allocator/RSS proof. Owned decode uses only the sealed bytes/policy and requires
exact full reencoding.

Transport maxima are 32 MiB body, 4 MiB chunk and 262144-byte manifest, subject
to tighter operator/logical limits and the 4096-chunk bound. Compression is
unsupported. Unknown format/compression, trailing data, malformed canonical
metadata, oversized fields and short non-final chunks refuse. Valid large content
may still refuse under a receiving or serving node's smaller working budget.
These limits do not change consensus validity. No test execution or B4 acceptance
is implied by this source contract.
