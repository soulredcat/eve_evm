<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Segmented authenticated application and restart recovery

Public owns this explicit persistence profile. The same AppliedOwner, immutable
reader, imported-state capability, genesis initializer and ordered durability
pipeline serve compact and segmented backends. Compact constructors and v1 wire
limits retain their behavior; this profile accepts only EVE_IMPORT_V2 and uses
the canonical pure importer for actual H/H+1 authentication, without REVM or a
second fee update.

`SegmentedAppliedConfig` combines the existing node settings with explicit v2
part policy and codec limits. Its logical limit comes from the codec, bounded by
the canonical 21,025,569-byte V2 ceiling. The application's compact payload limit
keeps its old meaning and is not relabelled as logical-body capacity. The actual
repository transaction limit must be one record, and worker scratch settings must
match the part profile. Existing working/process targets are unchanged.

This profile derives a distinct namespace domain using SHA-256 over
`EVE_PUBLIC_SEGMENTED_IMPORT_DOMAIN_V2 || configured_domain`. It verifies the full
stored prefix before starting its sole worker. Changing profiles cannot turn a
compact namespace into segmented history or reset existing contents.

Admission borrowed-preflights the V2 body, calculates its canonical logical SHA
identity, and atomically reserves every DataPart/MarkerPart and metadata before
candidate preparation. The unbound reservation introduces no target binding.
After real sizing/decode/native/candidate leases, canonical import preparation
produces the private authenticated state. Only its actual local binding finalizes
the reservation and is serialized into the marker. Sealing copies from the same
immutable caller input; that input needs its caller's separate transport charge.

Local state binding is SHA-256 over `EVE_PUBLIC_SEGMENTED_STATE_BINDING_V1` and
the canonical encoded StateVersion. It includes auxiliary local representation
identity and does not certify those auxiliary bytes or replace EVE_APP_V1/native
finality verification. Startup, producer and acknowledgement checks share this
one operation.

The existing actual-count import estimate supplies canonical candidate, decoded
operations, native copies and retained generation charges. V2 additionally
reserves two full logical byte envelopes, two journal encoding envelopes and
bounded operation-Vec storage for complete canonical reencoding. Sizing scratch
is held before these estimates; retained captures keep the immutable generation
lease. `reserve_applied_working` returns a sealed actual service-pool reservation
for upstream transport/assembly lifetimes, without authentication authority.
These are logical estimates, not allocator/RSS measurements or a capacity proof.

RAM publication uses a short guard only for immediate worker admission and the
complete pointer/cursor update. Failed proof, allocation, parts, queue or age
admission leaves the accepted publication unchanged. CPU, hashing, decoding,
state validation and storage waits happen outside RAM publication locks.

Pending payloads retain the full sealed batch through the application-level
ordered ACK check. A physical marker ACK must match the exact next logical
parent/height, body identity, target binding, marker and references. Counts are
bounded before slicing public ACK fields. Only this check advances logical
durability. Physical segment sequence is never used as a logical block height.

Shutdown preserves full pending parts and bounded worker tail descriptors,
including actual last acknowledged physical cursor and incomplete/complete marker
status. Partial physical progress does not advance logical durability or claim
the current physical head after ambiguous failure. Already validated complete
tails can release; failed tails remain owned for explicit reconciliation.

Restart attaches a real working lease to the scanner, assembled-body results,
verification and returned reusable buffer. Storage membership/full-body SHA
integrity creates no state authority. Every Complete bundle must pass canonical
V2 decode, exact local parent and H/H+1 verification, and local binding equality
before scanner acceptance advances the logical anchor. Malformed records fail
without reset, fallback decoder or deletion.

Unmarked data leaves the last complete verified state active as historical data,
preserves physical orphan positions, and exposes `missing_from`. Authenticated
replacement data can append after those orphans; recovery does not invent a
marker or duplicate effects. Unknown independent head freshness remains NOT_READY
even for a complete locally verified prefix. Runtime head corroboration, peer-tail
retrieval and snapshot orchestration remain subsequent integration work.

Tests author a greater-than-compact signed import with hash-checked unused code
auxiliary data, paused writes/RAM reads, exact restart and fee/receipt/state checks.
Orphan/panic/capacity fixtures distinguish real local sync from hardware power-loss
claims. Invalid complete bodies, malformed public ACKs, proof/resource/age refusal
and retained progress are checked. This component alone does not establish Source
readiness, measured T-N09/T-N10, PQ protection, full B4 or throughput acceptance.
