<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# 11 — Main goals and planning

## Requirements

| ID | Requirement | Primary plans |
|---|---|---|
| R01 | Validators own finality; no master override | 01, 10, 12 |
| R02 | Separate master/public/validator runtimes and distributable public code | 10, 19, 22 |
| R03 | Deterministic EVM execution and standard developer workflow | 13, 14, 18 |
| R04 | Durable recoverable state and signing safety | 02, 14, 16 |
| R05 | Authenticated snapshot/delta synchronization independent of source | 15, 16 |
| R06 | Permissionless public P2P with bounded abuse exposure | 15, 18, 19 |
| R07 | Staking, verified work/uptime and fee split 40/30/30 | 05, 13, 17 |
| R08 | Parallel execution equivalent to serial execution | 03, 20, 21 |
| R09 | Regional ingress and safe replication/failover | 06, 16, 21 |
| R10 | Verifiable release/update and threat-model coverage | 19, 20 |
| R11 | Real tests, telemetry, fault injection and reproducible evidence | 08, 20, 23 |
| R12 | Measured progression toward 1M aggregate finalized TPS | 08, 21, 23 |

Every Rxx must map to concrete tests/evidence in [20](20-test-vectors-and-acceptance.md); every implementation task belongs to a bulk in [23](23-task-backlog-and-execution.md).

## Delivery policy

Develop the smallest complete path first, but continue past a toy prototype. Keep production trust boundaries present in shared interfaces from the beginning. Make bounded development decisions without repeated approval requests; preserve explicit owner gates for real funds, launch, governance, licensing and infrastructure spending.

Decision D40 prioritizes EVE's own chain. Mandatory current work is B0–B11 with SEC0, SEC1 and SEC3; SEC2 and INT0–INT3 are `DEFERRED_UNTIL_EVE_TESTNET` for separately authorized programs. Their absent routes do not fail core acceptance, and deferral does not claim the historical bridge/interoperability requirements passed. EVE maintains its own versioned verification interfaces without external-chain execution, latency, availability or storage-acknowledgement dependencies.

## Success levels

DEVNET_ACCEPTED means functioning software with all B0–B9 tests and operations evidence. SCALE_TARGET_VERIFIED means the measured B10–B11 throughput/latency/storage/network gates actually pass. MAINNET_READY additionally needs independent review and owner-approved deployment decisions. None implies the next.

A failed benchmark is useful evidence but not completion. Missing hardware is a resource blocker, not evidence that the target is reached or impossible. The agent must leave runnable experiments and a precise capacity gap, and continue work that does not depend on the missing resource.

The complete execution objective is [goal.md](../../goal.md). Actual status belongs in [STATUS.md](../execution/STATUS.md), not in optimistic wording in this overview.
