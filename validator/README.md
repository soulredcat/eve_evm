<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Validator runtime

Planning only; the runtime is not implemented yet.

Role ownership and standalone distribution are absolute requirements. Components own deterministic execution, logical state, authentication, pinned consensus and immutable EVE genesis/native/header contracts. Validator signing/consensus wiring stays here; public readiness/source/persistence stays public-owned. A copy-ready validator must contain all dependencies, locks/toolchain, sanitized configuration and notices and build/run after copying this directory alone. Private master implementation and unresolved outside paths are forbidden. Current components do not yet prove a complete validator or standalone distribution. External programs are deferred until testnet under D40.

Owns transaction execution/replay, proposal validation, consensus participation, finality and validator lifecycle integration. Uses shared EVM/protocol/state modules and a reviewed BFT adapter; no private master database dependency.

Mandatory durable data: consensus WAL, anti-double-sign height/round/step and sign-byte history, key-fencing state, recent finalized block data and recoverable application state. Hot RAM does not replace these records.

A four-validator equal-power devnet needs three votes for a commit under the selected more-than-two-thirds rule. Loss of quorum stops finality; master never takes over. More validators do not automatically increase TPS because replicas verify the same ordered workload.

The initial one-master/one-public topology retains these four distinct validators. A single-validator or all-in-one harness is local-only development, with no distributed-consensus acceptance claim. Later two-master mutual replication and ten-master regional placement do not change validator quorum, signing authority or mandatory data availability.

Carry `zone_id` as operational routing/failure-domain metadata from the start, separately from network/genesis identity and EVM chain ID. Zone membership does not authorize independent state writes or finality. Transaction propagation, execution validation before voting and live finalized-block P2P distribution remain independent of master availability or public bulk-sync endpoint selection.

Public nodes have RAM working state plus durable verified block/checkpoint recovery data by default. Their persistence policy cannot replace validator WAL, anti-double-sign records or recent independently recoverable state/data. An authenticated master transport identity is never a substitute for validator history and finality proofs.

Read plans 12–17 and 20–22, plus 32 (local-only: `../docs/plan/32-regional-masters-and-public-persistence.md`). A co-located public RPC process does not receive consensus keys. Never clone live signing keys into an active standby.
