<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Checkpoint-base local record

Canonical owner: public recovery storage. This versioned payload records a local
checkpoint base in an actual synced opaque repository row. It does not verify
finality, execution, artifact availability, retention policy or source freshness.
The caller authenticates the referenced immutable snapshot and proof artifacts
before admitting the base or activating a recovered state.

The fixed header is 364 bytes: domain `EVE_CHECKPOINT_BASE_V1` (22), schema u16 (1),
verification mode u8 (authenticated import = 1), canonical target security profile
u8, target-version byte count u16, physical parent cursor (40), logical parent
anchor (80), target height u64 and state binding (32), snapshot manifest and body
hashes (32 each), proof manifest ID and root (32 each), and six u64 heights.
All integers are big-endian. The six heights are proof genesis, first/last
execution, H+1 lookahead, and first/last declared retained height. Exact canonical
target-version bytes follow, then SHA-256 over the complete header and target.
The target ceiling is 4096 bytes; the complete payload ceiling is 4492 bytes.
Unknown schema/mode, empty/zero hashes, gaps in the declared execution range,
invalid parent order, non-newer targets, overflow and trailing bytes reject.

The verification mode is distinct from the canonical target's security profile.
Genesis/network/configuration/profile/key epoch and all state-version metadata
remain in the exact canonical target encoding. A sealed prepared target uses the
existing eve-state encoder; borrowed preflight compares those exact bytes and
performs no target string/map allocation or duplicated state parser. The local
1024-byte network-name precheck does not change eve-state's stricter identity rule.
Prepared target metadata, artifact IDs and checksum equality confer no authority.

Membership reads the actual opaque row by a nonzero sequence and full hash. It
checks the real row's parent, canonical payload and every caller-expected base
field. The actual physical and logical parent cursors must belong to the opened
namespace; sequence zero is accepted only as that namespace's real bootstrap
parent. Height H with cursor zero cannot pass. The helper retains the same owned
row and exposes borrowed views; no detached count can authorize another buffer.

Reservation helpers admit canonical codec scratch, the retained target/payload,
actual repository read ceiling and fixed control envelopes. The caller acquires
and retains real working leases before preparation/I/O and while owned values
survive. Numeric reservation values are not leases or allocator/RSS measurements.
Storage integrity alone does not permit scanner activation or pruning. The
worker/scanner and authenticated recovery orchestration own those separate steps.
