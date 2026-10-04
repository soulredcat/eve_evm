<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Immutable ordered checkpoint witness storage

Canonical owner: public durable recovery storage. This capability stores opaque
execution witness blobs for heights 1 through target H, followed by exactly one
closing lookahead blob at H+1. It does not decode witnesses, check signatures,
execute blocks, authenticate finality or activate state. Validator-owned codecs
and checkpoint verification must establish those properties before runtime use.

The manifest binds the snapshot manifest ID, canonical snapshot body SHA-256,
exact canonical target version, target height, ordered per-witness kind/height/
length/SHA-256 and expected whole-stream SHA-256. The reference input slice is
borrowed and separately charged by the caller. No operation retains all raw
history bodies. A two-pass exporter can read one charged blob at a time to derive
references and the stream checksum, then encode the bounded manifest.

The V1 manifest header is 144 bytes: 24-byte `EVE_CHECKPOINT_PROOF_V1` plus NUL,
two zero format flags, u16 target-version length, u64 H, u32 file count, u64 total
blob bytes, snapshot manifest ID, snapshot body SHA-256 and expected stream
SHA-256. Integers are big-endian. Canonical target bytes follow, then 56-byte
ordered references: u64 height, one kind byte (0 execution or 1 closing lookahead),
seven zero bytes, u64 length and 32-byte SHA-256. The sequence must be exactly
1 through H+1 with the closing kind present only at the final entry.

Manifest identity is SHA-256 of `EVE_CHECKPOINT_PROOF_MANIFEST_V1` followed by
the exact complete manifest bytes. This commits ordered references and target
binding but does not grant authority. The raw stream SHA-256 begins with
`EVE_CHECKPOINT_PROOF_STREAM_V1`, snapshot ID, body SHA-256, u64 H, u16 target
length and target bytes. Each ordered entry then contributes its first 24
reference bytes (height, kind, zero padding, length), followed by its actual raw
blob bytes. The exporter checksum API enforces one ordered blob at a time.

Limits freeze maximum single-witness bytes, summed blob bytes, files, manifest
bytes and logical stored bytes. File count is at most 10,001, supporting H at
most 10,000 for this local retention profile. Target encoding is at most 4,096
bytes and manifest at most 1 MiB. Logical disk accounting includes blob bytes,
both possible manifest entries and both 80-byte completion entries. It does not
claim filesystem block/page-cache/RSS bounds or protect against a hostile local
owner independently writing files. These are local resource limits, not protocol
validity limits; a larger valid checkpoint needs a supported future profile.

Before any owned manifest allocation, the caller supplies the metadata reservation
and retains a real lease for the transfer/completed-store lifetime. IO operations
use one streaming 64 KiB scratch plus bounded metadata. Incoming blob bytes are
separately caller-charged. `required_checkpoint_proof_witness_reservation` must be
backed by a real lease before `read_checkpoint_proof_witness`; that lease outlives
the returned immutable one-blob buffer and its finality-codec processing. Numeric
reservation parameters alone do not acquire or prove actual runtime leases.

Linux directory/file guards reuse the canonical checkpoint storage primitives:
held directory file descriptor, private owner-only namespace, exclusive lock,
no-follow opens, regular single-link files, exclusive creation, no-replace rename
and actual file/directory sync. Generated witness names are `witness-NNNNN.bin`.
Completion streams every file in order, verifies all per-file hashes and the raw
stream hash, then atomically promotes and syncs the exact completion marker.
Native Windows and unsupported filesystem durability operations fail closed.

Identical manifests resume missing/corrupt/verified observations. Valid existing
files are never overwritten. Repairs remove only one rechecked invalid generated
entry in an uncompleted exclusively held namespace. Witness symlinks may be
unlinked without following a target; hardlinked files and valid content are
preserved. Pending metadata repair accepts only an invalid regular single-link
entry, refuses valid foreign metadata, and never touches a published completion.
Foreign manifest framing is checked independently of tightened receiver limits;
possibly valid oversized metadata is preserved without allocating beyond the
caller's current reservation.
Unknown entries, invalid published metadata and completed corruption fail closed.

The `checkpoint_proofs` test source covers real sync/reopen, exact one-blob reads,
ordered execution/closing references, actual no-destructor child exit, partial
staging, explicit invalid-entry repair, missing/corrupt/raw-stream mismatch,
oversized limits and one-byte reservation boundaries, locks, foreign manifests,
unknown entries, symlinks, hardlinks and immutable completed content. Opaque blob
fixtures deliberately make no claim to serialized authenticated witnesses.
Process exit does not prove hardware power-loss durability or complete B4.
