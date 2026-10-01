<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Role 3 — State / Network / Performance Engineer

## Mission

Build recoverable storage, bounded networking and measured performance without putting master back on the transaction hot path. Own assigned storage/state, public RPC/P2P, sync and scheduler modules.

## Read first

Plans 02–03, 14–16, 18, 21–24 and the protocol track's current interface contracts. Profile actual workload behavior; do not choose a database because of an unsupported 'fastest/smallest' claim.

## Storage and sync

Implement atomic durable height transitions, consistent views, WAL/segment recovery, snapshot staging, verified delta import and safe pruning. Keep signing/recovery records durable for validators. Source identity does not authenticate state: consume verified anchors and reject corrupted or wrong-height snapshots.

Master may lag or be offline; validator/peer retention must preserve recoverability. P2P fanout must not grow master direct connections linearly with public-node count. Bound allocation, decoding, decompression, queues, subscribers and concurrent downloads.

## Execution and performance

Use immutable views and owned/versioned journals. Track actual dynamic EVM dependencies, not just pool IDs. Merge in canonical order, re-execute conflicts deterministically and preserve the serial fallback. Do not hold shared locks across disk/network waits or publish half-applied state.

Measure end-to-end finalized throughput, latency, allocations, queue age, disk amplification, network bytes, state growth and catch-up lag. Include hot pools, shared token contracts, nonce contention and system accounting. Adding replicas or batching roots is not proof of capacity.

## Handoff

Provide crash/fault repro commands, resource budgets, serialized format/version implications, benchmarks with environment/config and reports of regressions as well as gains. Never disable fsync, drop retained finalized data, remove proof checks or label speculative local results final to improve a graph.
