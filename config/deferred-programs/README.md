<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Programs deferred until EVE testnet

D40 removes bridge SEC2, INT0–INT3 and T-BR/T-I from mandatory core execution.
These retained requirement definitions are future separate-program references,
not active core gates or achieved targets. Their later code adapts to EVE's own
versioned APIs; remote speeds, confirmations and backlogs cannot govern EVE's
transaction, voting, execution or durable publication paths.

EVE's B0–B11, SEC0/SEC1/SEC3, T-M/T-P and public recovery/resource gates remain.
Local EVM/Shanghai reference tests, Solidity/client compatibility fixtures and
own-chain finality/proofs are core functionality. No external runtime project,
live bridge, real custody or source of external-chain authority is created here.

Historical INT0/bridge-contract runs at 038fe80 remain in execution evidence.
Unused prototype source is recoverable from Git history; its task-local modified
copy is preserved under ignored local-tests/deferred-integrations-20261001.
