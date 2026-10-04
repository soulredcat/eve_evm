<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Retained validator delta serving

ABCI Query `/eve/recovery/v1/delta` accepts canonical `StateDeltaRequest` bytes.
The request identifies the exact parent version/network, next height, byte offset
and maximum chunk length. Native query height is zero or that requested height;
proof requests are unsupported. Success returns canonical `StateDeltaChunk` bytes.
The body is `StateDeltaPayload`: ordered canonical journal and execution payload.

The source is untrusted. SHA-256/checksums detect transfer inconsistency and grant
no finality or execution proof. Public independently obtains certified native H
and H+1/lookahead data, binds transactions/header context and uses canonical import.
Parent auxiliary representation is exact; this endpoint never rebinds a journal.

Every request reads current RAM first. A miss/stale cache admits the configured
maximum decode cost before repository reload/cache refresh. Default capacity may
refuse that conservative cold-cache bound. Historical commits come from one actual
StateSnapshot; its version must equal the captured RAM version. The returned tip
is that durable capture. Missing/future heights return GAP; unavailable or corrupt
storage returns NOT_READY. No database handles or master implementation are exposed.

Journal/target/body identity and chunks at one height are stable across later heads.
The separately observed durable tip and enclosing checksum may advance. Chunk length
is exactly min(request.maximum_chunk_bytes, total_length-offset). Four MiB chunks
plus bounded version metadata fit the unchanged 4 MiB+64 KiB application frame cap.

Configured CLASSICAL_DEV query admission has an independent 64 MiB working allowance,
not the proposal clone reservation. The exclusive actor permits one active query.
Before capture/historical decoding, actual cached head and pinned historical encoded
lengths receive a conservative 128x decode/map/root/projection charge plus fixed
control allowance. Large component/output/chunk copies are charged before encoding.
Insufficient capacity returns RESOURCE_LIMIT; large valid states may therefore be
unavailable under the default allowance. These are conservative operator estimates,
not allocator/RSS bounds or measured CPU/IO isolation. Query work shares this actor
and still requires interference/load evidence; no zero-overhead claim is made.

Stable failures are WRONG_NETWORK, UNSUPPORTED_VERSION, GAP, RESOURCE_LIMIT,
NOT_READY and MALFORMED_REQUEST, in codespace EVE_RECOVERY. Other query paths keep
the existing EVE_QUERY_UNSUPPORTED response. Queries write no consensus/state data,
execute no transactions and change no vote, signing, commit or fee semantics.
