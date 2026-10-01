<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Validator execution component

Canonical owner: validator execution. This package owns deterministic Shanghai
execution, signed admission, EVE fee allocation, complete candidate construction
and isolated unsigned simulation. Public ingress/mempool/readiness and raw storage
remain public-owned. Private master code is not a dependency. Read plans 13/14/18/22/25
and the state/protocol component READMEs before changing these contracts.

## Canonical candidate execution

`execute_state_block` accepts a validated complete parent `StateCommit`, ordered
signed bytes and `ExecutionBlockInput { timestamp, proposer, previous_consensus_hash }`.
It derives identity, chain, next height, active parent gas limit, f101/f102 fee
escrows, raw transaction bounds and checked next base fee. No host clock/network
input or arbitrary fee override participates. The agreed proposer/consensus-hash
inputs are caller-supplied; accepting them does not authenticate consensus.

`PreparedStateBlock` returns the complete canonical commit, exact replay journal,
transaction outcomes and fee allocation. The journal includes target execution
history after the header is known, so replay reproduces complete target content.
Invalid ordered execution or any bound/commitment failure discards the candidate.
The caller owns publication/durable acknowledgment; no signature/finality is created.
Master and public development composition share this operation rather than copying
header/receipt/root assembly. Pending state may use the same unpublished candidate.

The checked protocol helper derives EIP-1559 pricing from parent gas use with
elasticity 2, denominator 8 and floor 1. It rejects overflow before calling the
maintained update implementation. A 30M gas limit, 128 KiB raw transactions and a raw-user
block sum of 4 MiB are development defaults. Count preflight derives from parent gas
limit divided by Shanghai minimum intrinsic gas of 21,000. Raw sum is not the enclosing
BFT block budget; B3 must validate its complete protocol overhead independently.

`execute_complete_state` and `execute_serial_block` remain lower-level reference
interfaces for agreed/custom fixture environments. Real Shanghai transactions are
protected legacy/type1/type2, canonical signed encodings, checked recovery/low-s,
ordered nonce/balance validity and canonical receipts/roots. Unsupported forks
reject. EVE replaces Ethereum fee redistribution with 40/30/30; the Ethereum
proposer reward is suppressed and fees are split once, including validator dust.

Shanghai commit applies touched-empty clearing at every transaction boundary,
before the next execution lookup. Generic CacheDB commit alone retains those
empty accounts. The canonical operation keeps NotExisting tombstones so projection
and subsequent execution see deletion, while untouched/reverted empty leaves remain.
This follows [EIP-161](https://eips.ethereum.org/EIPS/eip-161); it is not a final-root
filter. [EIP-1052](https://eips.ethereum.org/EIPS/eip-1052) returns EXTCODEHASH zero
for both dead-empty and absent accounts, so deletion regressions also inspect actual
cache presence rather than treating the opcode alone as an absence proof.
## Read-only signed admission

`decode_signed_transaction` returns a sealed `ValidatedTransaction`. Read-only
`hash()`, `sender()`, `transaction_type()` and `evm()` expose its immutable decoded
contents. Callers cannot replace the recovered sender/transaction after decoding.
The inclusion executor independently decodes the original signed bytes.

`check_transaction_admission` consumes that envelope plus one captured sender
account, derived basefee and gaslimit. It uses maintained Shanghai intrinsic gas,
checks EIP-3607, fee caps/basefee, gaslimit and checked maximum fee plus value balance.
It returns `TransactionAdmission` metadata including nonce/state_nonce, caps,
effective price, intrinsic gas and maximum upfront cost. Public owns stale/future
nonce queues, replacement/TTL/order/byte accounting. A future nonce is not rejected
by this metadata check; canonical serial inclusion still requires exact nonce.
These checks do not reserve a mempool entry or establish inclusion/finality.

## Isolated calls and estimation

`SimulationContext` borrows complete state, bound version/header, state budget,
explicit limits and actual reserved clone bytes. `SimulationRequest` supplies an
unsigned caller/target/value/data, gas and optional legacy or dynamic fee fields.
The engine uses the selected header's NUMBER/TIMESTAMP/BASEFEE/COINBASE/PREVRANDAO,
identity CHAINID and complete previous execution-hash window. It does not advance
historical reads into the next block; pending explicitly selects a candidate header.

Simulation-only configuration permits contract callers, skips nonce checking and
fee charges, and bypasses basefee validation for zero simulated gas price/cap.
Explicit positive caps retain maintained checks; an omitted type2 priority fee
means zero. Explicit unsigned types 0/1/2 are supported; access-list presence infers
1 without dynamic fees, dynamic fee fields infer 2. Incompatible legacy/type1/type2
fields and unsupported 3/4 reject rather than silently dropping fields. Maintained
AccessList entries/keys are checked before cloning, with caller limits and hard
256-entry/1024-key caps. Conflicting fee fields and value exceeding captured balance reject.
No balance synthesis flag is enabled. Canonical execution keeps the original checks.
The actual EVM outcome is returned with status/gas/output/revert/halt data; all
private writes, nonce changes, creation, fee and system effects are discarded.
Maintained `ExecutionResult`, `Output`, `HaltReason` and `InvalidTransaction` types
are reexported so RPC consumers can map actual results without a separate REVM import.

`estimate_complete_state_gas` runs the real call at a bounded upper limit, searches
at most the declared 2–64 attempts, and validates its returned gas limit with a final
successful replay. It propagates the upper-limit revert/halt and only treats typed
intrinsic low-gas errors as search failures. Gas-sensitive contracts can be
nonmonotonic; this is a successful tested limit, not a globally minimal-gas proof.

Simulation gas/calldata/interpreter memory are explicit hard limits. Clone
reservation uses the canonical conservative two-oracle estimate and rejects before
construction. Public must also charge interpreter memory, retained original views,
request/result bytes and concurrency to its global admission budget. Logical
charges are not allocator/RSS measurements or a production throughput guarantee.
No state lock spans disk/network I/O; execution itself invokes neither.

## Explicitly inactive native system interface

The maintained Ethereum precompile provider is wrapped with a thin delegation
adapter. Code-address f100 calls, including internal CALL/STATICCALL/DELEGATECALL,
return actual revert bytes `EVE_NATIVE_INTERFACE_INACTIVE_V1` and the frozen 5,000
native dispatch charge; insufficient forwarded gas follows maintained OOG behavior.
Value/custody and system writes revert while included transaction gas/nonce semantics
remain intact. A contract may handle its internal call failure according to EVM
rules. f101/f102 remain distinct fee escrows. Ethereum precompiles are preserved.
The bytes are an explicit development failure payload, not a native ABI read answer.
This boundary does not implement B5 staking/rewards or activate hybrid/PQ security.

## Verification and distribution

Scoped tests cover parent-derived fees, exact journal replay, invalid atomic
candidates, intrinsic/admission failures, native direct/internal rejection,
Ethereum precompile preservation, unsigned mutation isolation, real revert data,
BASEFEE/GASPRICE, gas estimation and memory/clone limits. Existing eight EVM
regressions remain unchanged. Independent pinned Ethereum corpus and TypeScript/
Solidity acceptance belong to tests/acceptance/serial-rpc; their actual runner
records supported cases and explicit EVE/fork divergences separately.

Run scoped strict lint, formatting, tests and release build, then the integrated
bulk gate. Complete standalone role distribution still requires reproducible
canonical source packaging and unrelated-directory build/run; a monorepo component
build is not that acceptance. No secured 1M-TPS, authenticated dev finality, B3
validator quorum/sign durability or native/PQ production claim follows from B2.
