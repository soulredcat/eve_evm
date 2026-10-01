<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# 18 — RPC, mempool and developer experience

## A01 — Required RPC surface

Implement real responses for:

`web3_clientVersion`, `net_version`, `net_listening`, `eth_chainId`, `eth_syncing`, `eth_blockNumber`, `eth_getBalance`, `eth_getCode`, `eth_getStorageAt`, `eth_getTransactionCount`, `eth_sendRawTransaction`, `eth_getTransactionByHash`, `eth_getTransactionReceipt`, `eth_getBlockByNumber`, `eth_getBlockByHash`, `eth_call`, `eth_estimateGas`, `eth_gasPrice`, `eth_maxPriorityFeePerGas`, `eth_feeHistory`, `eth_getLogs`, and `eth_getProof`.

Add WebSocket `eth_subscribe`/`eth_unsubscribe` for new heads and logs after the corresponding historical queries are correct. Unsupported methods return an explicit JSON-RPC error, never a success-shaped placeholder. Document any unsupported optional method before declaring client compatibility.

Use standard hex quantity/data encodings and null for genuinely unknown transactions/receipts. Distinguish unknown, not-yet-applied, pruned and malformed requests. A pruned log range must not look like an empty range containing no events. `net_version` is decimal chain ID; `eth_chainId` is its standard hex form.

## A02 — Read and finality semantics

`latest` means the highest locally verified, consensus-committed and applied execution state, not the highest received unverified header. `safe` and `finalized` map to that node's verified BFT-finalized state under plan 12. Fast-sync followers only expose a height after its state commitment is properly authenticated. A reexecuting full node may derive a state from a committed block and authenticated parent without waiting for a remote master's storage.

`pending` is a local simulation/admission view and carries no global finality guarantee. `eth_sendRawTransaction` returns the standard transaction hash after bounded validation/admission/relay; it does not mean inclusion or finality. Persisting a receipt on one master is not the definition of network finality.

Account/storage proofs verify against the returned EVM state root. They do not automatically prove membership in Ethereum mainnet or a beacon-chain header. Add `eve_getFinalityProof` to expose the EVE execution-hash/application-commitment/consensus-header binding and `eve_getNodeStatus` for finalized/applied/durable/authenticated heights, peer count, lag and verification mode.

Each multi-key read/simulation captures a single state view. Ready status is false during unverified bootstrap or unacceptable lag. Process health may remain true while readiness is false.

## A03 — Mempool policy

Identify duplicates by transaction hash and replacements by `(sender, nonce)`. Keep executable nonce chains and bounded future-nonce queues. Revalidate after each committed block and evict transactions that can no longer be valid. Mempool policy is local admission policy, not an extra consensus validity rule.

Initial local limits: 10,000 transactions, 64 MiB total raw bytes, 64 queued items per sender, 300-second local TTL. Enforce both count and byte limits. The TTL uses local time only for eviction, never for whether a committed transaction is valid.

Replacement policy requires at least a 10% integer-rounded-up increase in the relevant fee caps; for type 2 increase both max fee and max priority fee, with a minimum one-base-unit increase. Reject replacement that violates fee/nonce/balance rules. Pin exact edge-case vectors, including zero tips and base-fee changes.

The baseline proposer prioritizes eligible sender-head transactions by effective tip, then transaction hash, preserving sender nonce order and block limits. Other valid proposed orders remain acceptable: peers need not share mempool arrival order. No fairness or MEV-resistance claim is made. A deterministic tiebreaker does not prevent censorship or priority extraction; such policies are separate research.

## A04 — Query budgets

Default development HTTP endpoint is loopback port 8545 and WebSocket port 8546; external exposure is opt-in. Bound JSON body size, batch count, response bytes, subscriptions, simulation concurrency, simulation gas and log ranges. Starting limits: 1 MiB request body, 100 JSON-RPC batch items, eight concurrent simulations and 1,000-block log scan range. Return pagination/range-limit errors when applicable.

A wall-clock timeout may abort an RPC simulation with a timeout error; it cannot change consensus execution results. Validate gas estimation with actual replay, propagate revert data and never assume a fixed successful gas value for every contract. Fee suggestions use documented recent-chain data and dev defaults, not outside price APIs.

Public RPC shares no signing/admin credentials with validator or master. A node may relay transactions through several validator peers without opening direct master access.

## A05 — Developer fixture

Create a pinned TypeScript test client using a maintained Ethereum client library, plus pinned Solidity compiler/tooling targeting Shanghai. From a clean devnet it must:

1. Read chain identity and funded development accounts.
2. Send a signed native transfer and check exact fee/value/nonce changes.
3. Deploy and transfer an ERC-20, verify code/ABI/events/receipt.
4. Deploy minimal two-pool fixtures and execute one atomic successful path.
5. Force the second leg to revert and verify no partial token movement survives, while gas/nonce behave correctly.
6. Test eth_call, estimation, logs, proofs, subscriptions and historical read errors.
7. Restart public and master processes and repeat verification through another RPC node.

Never use the user's real wallets or tokens. Examples must explain the exact EVM fork, custom fee distribution, finality semantics, system interface and unsupported features. Packaging must provide reproducible commands rather than a claim that every Ethereum tool works automatically.

## Acceptance

T-A01: RPC encoding/error conformance and hash/header/receipt consistency.
T-A02: real TypeScript/Solidity end-to-end fixture succeeds without mocked responses.
T-A03: pending/unknown/pruned/finalized semantics are distinct and tested.
T-A04: nonce gaps, duplicates, replacements, mempool eviction and post-block revalidation.
T-A05: expensive simulation/log/subscription floods stay bounded and do not starve consensus.
T-A06: read isolation, event ordering, multi-pool atomic revert and proof verification through independent RPC peers.
