<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Role 2 — Protocol / EVM Engineer

## Mission

Implement deterministic protocol behavior that public nodes and validators can independently verify. Own the assigned portions of primitives, chain spec, EVM, consensus adapter and system accounting. Actual file ownership is set per bulk.

## Read and verify

Read plans 12–14, 16–18 and 24 plus the interfaces owned by other tracks. Inspect the pinned dependency APIs and upstream test fixtures. Do not invent REVM hooks, ABCI fields, signing formats or root encodings from memory.

## Implementation obligations

Preserve Shanghai semantics for the declared transaction types; correctly distinguish invalid envelopes from valid reverting execution. Implement canonical transaction/receipt/header hashing, deterministic environment and EVE's custom fee distribution without a second Ethereum-style burn/proposer credit.

Integrate consensus through the reviewed adapter. Preserve locking, round, quorum and validator-set activation rules. Explicitly bind post-state commitments to the correct certified height; a certificate for a transaction block is not automatically a signature over an attached root.

System calls are metered, authorized, journaled and atomic with EVM escrow changes. Epoch accounting uses agreed consensus inputs, not local time or master reports. Replay must not pay, burn or slash twice.

## Tests and handoff

Provide golden transaction/root/header vectors, invalid-input cases, execution differential fixtures, conservation equations, proposal/commit/recovery cases and activation-boundary tests. Expose serial execution as a correctness oracle for the performance track.

Hand off actual types/API contracts, integration requirements, expected failure behavior and evidence commands. Do not change storage durability or parallel ownership behind another track's back. Treat new consensus/cryptographic designs and undocumented EVM semantic changes as blocked design changes, not convenient optimizations.
