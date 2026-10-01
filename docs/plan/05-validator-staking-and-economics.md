<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# 05 — Economic invariants

The user-selected split is 40% burn, 30% node reward pool and 30% validator reward pool. Implement deterministic integer accounting over collected gas fees after applicable gas refunds, including fees from valid reverting transactions. Do not distribute transaction value as fees.

EVE's 40/30/30 distribution is a chain-specific economic rule. Standard EIP-1559 transaction encoding/pricing does not mean Ethereum's base-fee burn distribution is preserved. Prevent accidental double burn or extra proposer credit in the EVM adapter.

Node count does not produce voting power or reward entitlement. Public operation is permissionless; earning a node reward requires registered, assigned, verifiable work and the specified availability score. Validator participation is authenticated by consensus records rather than a master's report. Staking eligibility, voting power and reward allocation are related but distinct rules.

Track supply, bonded/unbonding escrow, fee escrow, node/validator reward liabilities, claims, burns and penalties. Replay every block/epoch without paying twice. No reward may exceed its funded pool; undistributed amounts remain an explicit liability, not a hidden payment to the operator.

The first development network uses fake tokens and parameters from genesis. Mainnet supply, economic tuning and governance remain owner gates, not values an agent may invent for production. Rewards are fee-funded by default; no guaranteed yield or profitability is promised.

[13](13-transaction-and-gas-spec.md) specifies gas/fee collection. [17](17-validator-lifecycle-and-rewards.md) specifies registration, delegation, deterministic participation/work, distribution and penalties. [16](16-genesis-upgrade-and-recovery.md) specifies genesis and upgrades.
