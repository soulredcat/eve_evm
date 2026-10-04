<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Streaming authenticated checkpoint capability

Validator finality verification owns this pure certified-outcome operation. It
starts from an actual private ImportedState K, never a peer-selected validator
set, root, genesis or profile. It retains only the full target and one preceding
execution/version/header plus bounded native verifier state, without Vec0..H.

The target must preserve every actual parent history entry through K, contain no
unchecked replacement in that prefix and include every ordered K+1..H entry.
Canonical complete-state/commit validation checks account/storage/code/system
material, current roots, hash window, execution metadata and receipts. Each new
entry is matched against an ordered execution witness authenticated through the
actual next native certificate. Native data, time, proposer, mixHash, header
parent, base fee and gas context use the same canonical operations as import.

An actual parent K>0 already carries certified K+1 lookahead. The first witness
must reuse those exact native/data bytes, without rebinding them. Subsequent
witnesses verify real successor certificates under the inherited fixed genesis
policy. The final H+1 frame authenticates target H through EVE_APP_V1. Missing,
reordered, invalid, wrong-profile or unsupported-transition proof cannot finish.

AuthenticatedCheckpoint fields are private. Its sole authority conversion is to
ImportedState, preserving certified-import semantics; it cannot become independent
EVM replay. EVE_APP_V1 certifies roots and execution hash. Unused code, locally
computed content digest and historical versions' auxiliary representation remain
checked local consistency rather than separately certified exact values. The
classical quorum/adversary assumptions and profile limits are unchanged.

Callers own decoded target/witness input leases and reserve bounded canonical
sizing scratch before asking for the conservative reservation. They hold real
capacity throughout complete-state/root/encoding work, native copies, session and
returned capability lifetime. The numeric reservation is only a checked parameter;
it supplies no lease, allocator/RSS guarantee, durability or proof authority.

This first source step implements no wire-state preflight, chunk transport,
resumable durable staging, checkpoint-base schema, atomic activation, snapshots
in the public runtime, Source freshness, PQ acceptance or complete B4. Those
requirements remain explicit subsequent work; no heightH/cursor0 bypass exists.
