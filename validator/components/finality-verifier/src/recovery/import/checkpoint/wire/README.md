<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Canonical checkpoint witness wire

Canonical owner: validator finality verification. This transport carries one
untrusted streaming checkpoint witness and reuses the maintained state and native
frame codecs. It implements no new signature, EVM execution or consensus parser.

The V1 envelope is the literal ASCII domain `EVE_CHECKPOINT_WITNESS_V1`, one
kind byte (1 execution, 2 lookahead), and a big-endian u32 length-prefixed canonical
native data frame. Execution adds one length-prefixed canonical StateVersion and
one length-prefixed canonical BlockPayload. Lookahead ends after the native data
frame. Unknown kinds, trailing bytes, malformed framing and oversized components
are rejected. The canonical uncompressed bytes identify the record independently
of any outer chunking. The maximum wire envelope is 13 MiB; operators may choose
a smaller maximum witness size. An envelope limit is local resource admission,
not a consensus block-validity change.

Borrowed preflight seals the exact immutable bytes, copied StateBudget and
CheckpointLimits, component slices, byte sizes and transaction/receipt/signature
counts. Native fields use the existing allocation-free pinned-protobuf scanner;
execution uses the existing bounded RLP scanner. Version field grammar, widths,
UTF-8 and network byte count use the canonical state-owned borrowed version scan.
Version metadata is decoded by the state-owned decoder after the reservation check.
Semantic header, certificate, identity, parent, roots, H/H+1 and target checks
remain the authenticated checkpoint verifier's responsibility.

Owned decode accepts only the sealed preflight and a checked numeric reservation.
It uses the same native and state decoders, then reencodes the complete envelope
for exact canonical equality. Decoding produces an untrusted CheckpointWitness,
never an AuthenticatedCheckpoint, ImportedState, durability or freshness claim.

The decode estimate is checked arithmetic over actual scanned sizes and counts.
It charges 16 complete encoded envelopes, four raw payload envelopes, four Bytes
scaffolds per native/execution transaction or receipt, 2048 bytes per native
signature, four version byte and network String envelopes and 256 KiB bounded metadata/scratch.
The caller separately leases the ingress bytes before preflight, owns the actual
capacity lease before decode, and retains it until decoded witnesses and any
copies have dropped. Encoding similarly requires charges for existing typed
input, bounded version scratch, native/block component buffers and output.
Numeric estimates provide no allocation/RSS, physical-memory or lease guarantee.

This source capability has no network I/O, proof storage, checkpoint-base schema,
atomic public activation or complete B4 acceptance. Tests and publication gates
must be executed by the integrator before reporting verification.
