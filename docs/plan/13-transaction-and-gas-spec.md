<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# 13 — Transactions, EVM execution and gas

Status: normative devnet choices, explicitly distinct from a promise of full latest-Ethereum protocol equivalence.

## E01 — Baseline execution

Use pinned REVM with Shanghai execution semantics. Reuse a maintained Ethereum transaction/receipt/RLP implementation for types and cryptography. Pin matching crate versions and Solidity fixtures in B0. [REVM's maintained source](https://github.com/bluealloy/revm) and [API documentation](https://docs.rs/revm) are primary references.

Accepted user transactions: EIP-155-protected legacy, EIP-2930 type 1 and EIP-1559 type 2. Reject unprotected legacy signatures, unknown types, blob transactions and newer authorization-list transactions until a documented fork enables them. Verify canonical encoding, signature range/low-s rules, sender recovery, chain ID, nonce bounds, intrinsic gas, balance and declared size before inclusion. Do not silently normalize malformed signed bytes and change their hash.

`tx_hash` is the standard Keccak hash of the signed transaction encoding. Transaction identity and mempool replacement identity `(sender, nonce)` are different. Chain ID is one logical chain-wide value, never a region ID. The full network identity additionally binds the immutable genesis/config hash.

## E02 — Deterministic environment

NUMBER is execution height. CHAINID is the configured EVM chain ID. BLOCKHASH uses the previous EVE execution-header hashes within the supported historical window. TIMESTAMP is whole seconds derived from the agreed consensus timestamp, never the machine clock. EVE allows nondecreasing seconds when multiple consensus blocks fall in one second; document this consensus-environment difference.

COINBASE is the EVM reward address mapped to the agreed proposer identity; its fee income follows EVE accounting, not an implicit second payment. BASEFEE is the block's derived base fee. PREVRANDAO is a documented deterministic value derived from the preceding consensus block hash in the devnet, not unbiased randomness or a secure lottery seed. Difficulty/nonce/uncle/withdrawal fields follow plan 14's declared EVE header mapping. Shanghai excludes transient storage and newer fork-specific execution behavior; report unsupported features accurately.

No execution-time public RPC, pricing API, master query, local randomness or floating-point arithmetic. Any external observation that affects protocol state must first become authenticated consensus input under a specified rule.

## E03 — State transition and errors

Check each transaction against state produced by its predecessors, not independently against the original block state. Validate nonce progression and upfront affordability; apply the pinned EVM rules for gas reservation, refunds, reverts, contract creation and code size.

Distinguish INVALID (cannot appear in an accepted block) from EXECUTED_REVERT/OUT_OF_GAS (valid included execution with receipt status 0). Reverts restore transaction effects according to the fork rules while nonce and gas charges behave correctly. Failed internal calls and partial gas refund behavior must match the reference EVM.

Receipts and logs use canonical transaction order. Accumulate actual charged gas after applicable refunds. Record exact effective gas price and contract address where applicable. `eth_call` and estimation operate on isolated snapshots; they never consume canonical nonce/balance or produce durable receipts.

## E04 — Fee pricing versus redistribution

Use EIP-1559 type-2 pricing with `effective_price = min(max_fee, base_fee + max_priority_fee)` and reject cap inconsistencies. The baseline base fee evolves from parent gas use with target `block_gas_limit / 2`, denominator 8, integer rounding and a genesis minimum of 1 base unit. The local development initial value is 1 gwei; all values are token base units, not a market-price promise. Types 0/1 use their gas price as their cap and must cover base fee.

Ethereum burns the base fee and credits the priority component under its own rules. EVE deliberately replaces that redistribution with 40/30/30 of collected fees. Preserve the advertised transaction/gas semantics while explicitly documenting this economic difference. Reference: [EIP-1559](https://eips.ethereum.org/EIPS/eip-1559).

For a committed block, let F be the sum of charged gas multiplied by effective price over included transactions. Compute:

```text
burn = floor(F * 4000 / 10000)
node_pool_credit = floor(F * 3000 / 10000)
validator_pool_credit = F - burn - node_pool_credit
```

The rounding remainder goes to the validator pool and is tested. These integer-rounded amounts conserve F exactly. Use checked wide intermediates; no floating-point percentage math. Burn removes value from spendable supply and increments an auditable counter; sending to an arbitrary dead-looking account is not the burn implementation.

The EVM adapter must not both run Ethereum's default burn/proposer-credit path and apply the EVE split. Implement a reviewed fee-accounting hook for the pinned engine or a rigorously tested equivalent journal transformation. Do not compensate by minting coins. Fee escrows/reward liabilities and supply are covered by plan 17.

## E05 — Resource limits and admission

Genesis/config binds block gas limit, maximum serialized block bytes, per-transaction bytes and relevant execution limits. Initial local profile: 30,000,000 block gas, 4 MiB complete consensus block budget, 128 KiB maximum raw user transaction. These are development defaults, not 1M-TPS settings. Validate enclosing protocol overhead as well as raw transaction sums.

Mempool policy may be more restrictive than consensus validity but must not make validators reject a valid proposal merely because their local limits differ. Recheck affordability and nonce at inclusion. Oversized calldata, signature storms and simulation requests are bounded separately. State growth remains metered through the declared gas schedule; cheaper custom state operations require an explicit upgrade and resource model.

## E06 — System operations

Staking and work/claim operations enter through ordinary signed EVM transactions to the reserved system interface in plan 17. Native handlers are deterministic, metered and journaled together with EVM state. System state and any EVM escrow transfers revert atomically on failure. Epoch bookkeeping runs once in the deterministic block transition and is included in state commitments; it is not an unrecorded off-chain database update.

## Acceptance

T-E01: canonical signed fixtures for all accepted types and rejection vectors for wrong chain, malformed RLP, high-s, unsupported types and insufficient gas/balance.
T-E02: serial reference results for transfer, ERC-20, CREATE/CREATE2, delegatecall, selfdestruct under Shanghai rules, reentrancy, revert, out-of-gas and storage refund.
T-E03: fixed block environment produces identical roots/receipts on repeated runs and different worker schedules.
T-E04: fee caps, base-fee rise/fall/floor, refund limits and odd-wei rounding; no double fee, accidental mint, extra proposer credit or payout on invalid envelopes.
T-E05: duplicate nonce, competing replacements and multi-transaction affordability checks.
T-E06: simulation leaves canonical state unchanged; system-call reverts restore both state domains.
