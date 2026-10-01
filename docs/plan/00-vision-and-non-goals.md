<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# 00 — Vision and non-goals

## User objectives

Build an EVM-compatible network with separate public, validator and master runtimes. Public nodes may be operated by anyone; master infrastructure is developer-operated and protected. Validators decide consensus; masters synchronize and retain finalized data. Working state should be RAM-friendly, durable data compact and recoverable, and regional traffic handled in batches where semantics allow.

The current goal is EVE's own chain. External-chain bridges and adapters are `DEFERRED_UNTIL_EVE_TESTNET`, then require a separately authorized program whose adapters conform to EVE. They are outside core acceptance and must not control EVE block production, voting, finality, transaction latency or durable acknowledgement. EVM compatibility remains an execution/RPC contract, independent of remote Ethereum or other networks.

Retain the long-term objective of 1,000,000 aggregate finalized TPS. Success also requires deterministic execution, bounded queues, retained data, correct gas, atomic contracts, verifiable state and realistic operating costs. More public nodes, higher NVMe bandwidth or smaller commitment messages alone do not establish capacity.

## First accepted network

A four-validator devnet must independently execute Solidity transactions, finalize blocks, survive one validator failure, preserve data while master is unavailable, restore master/public nodes, and stop finality when quorum is lost. It must expose usable Ethereum-style RPC and demonstrate correct staking/fee accounting.

Master-only development is allowed through explicit co-located test roles. It is not the production finality model.

## Not authorized by this plan

No mainnet launch, live token allocation, paid infrastructure, real key ceremony, external-chain bridge development before EVE testnet, production bridge or settlement deployment, or claim of inherited L1 security. These require separate approval and evidence. Reaching testnet does not automatically authorize adapter development. No change to standard EVM synchronous atomic calls merely to improve a benchmark. No promise that one hot pool can process the same rate as many independent accounts.

Read [goal.md](../../goal.md) for completion levels and [24](24-decision-register.md) for fixed decisions and bounded development defaults.
