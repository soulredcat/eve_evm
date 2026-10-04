<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Canonical state delta transport

Validator owns canonical journal and block-payload encodings. This versioned
transport combines those existing encodings without executing transactions or
certifying a peer-selected state. Public/master consumers must separately check
exact parents, native history and H/H+1 application authentication.

StateDeltaRequest carries the complete local parent version, next target height,
byte offset and maximum chunk width. Its maximum encoded size is 4,136 bytes.
StateDeltaPayload contains an ordered StateJournal and BlockPayload; the ceiling
is 16,792,774 bytes with the original 8 MiB journal and bounded full execution
limits. A sealed borrowed preflight binds bytes and the frozen local StateBudget
before owned decoding. Complete canonical reencoding requires separate caller
capacity; the codec does not acquire node resource leases.

StateDeltaChunk carries parent/target/captured durable-tip versions, whole-body
SHA-256, total length, offset and at most 4 MiB data. Its checksum binds transfer
metadata/data only. The body identity and parent/target stay stable across chunks;
observed durable-tip metadata may advance. Invalid widths, offsets, versions,
lengths, truncation, checksums and noncanonical encodings reject.

All versions and source metadata are untrusted until the canonical verifier checks
locally anchored history and the prepared destination. SHA/checksums, a matching
root and the source's advertised tip establish no execution or freshness proof.