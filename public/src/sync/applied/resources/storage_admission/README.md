<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Public storage operation admission

This owner-bound controller enforces the configured simultaneous storage job count
and raw snapshot staging bytes before allocating or starting checkpoint IO.
The development budget supplies two jobs and 16 MiB staging. Its control structure
also holds a real working-pool lease before allocation.

Reader clones and checkpoint transfers share this exact controller. Captured
artifact manifests retain staging leases through completion, authentication,
pending activation, ambiguous acknowledgement and shutdown tails. Disk job slots
surround their synchronous IO/materialization and release with that job. A raw
buffer remains separately staging-charged until it drops.

Checkpoint body and witness reading acquire raw buffer/scratch staging and working
capacity before IO. Canonical decode additionally reserves six actual encoded-byte
raw-copy envelopes before decoder/reencoder materialization; decoded state maps
and verification sessions retain their independent working estimates. Body/read
scope ends before proof-stream jobs, avoiding nested slot admission.

The fixed development checkpoint transport uses at most 256 KiB request/JSON
response. Each real client callback acquires a working lease and 1900544 staging
bytes: five 256 KiB raw/RLP/reencoding envelopes, two request/framing envelopes,
and 64 KiB control. The 64 MiB conservative JSON working allowance is not charged
to the 16 MiB raw staging pool. Reference vectors and generated proof manifests
reserve their actual count-derived metadata envelope separately. Spent source
response ownership drops before the second proof-transfer pass.

Capacity refusal is local backpressure; valid larger inputs may refuse without
changing their consensus validity. These leases do not establish allocator/RSS,
OS hard CPU/memory quotas, finality, freshness, or a complete B4 acceptance result.
Future transport-profile changes must revise this raw-copy accounting before
integration; the fixed private profile cannot silently acquire larger raw inputs.
