<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Serial execution and public RPC acceptance

Canonical owner: B2 independent correctness/security acceptance. Production EVM,
logical state and protocol rules remain validator-owned; public owns RPC,
admission and persistence. This package owns test consumers and disposable
Solidity/TypeScript fixtures. It contains no production SDK, private master
implementation, validator signing authority or custody path.

Every result must come from real execution. Missing pinned tools, binaries,
fixtures, selected tests or final assertions fail; successful-looking mocked RPC
responses and empty test discovery are not acceptance. Current source is a test
implementation, not a declaration that the integrated B2 gate has passed.

## Responsibilities and paths

- `tests/corpus/` consumes the entire pinned Shanghai state workload and its
  transaction-deserialization vector. Per-family metadata lives in
  `fixtures/corpus/shanghai/`; upstream payloads remain in ignored local storage.
- `tests/cases/` tests actual execution, unsigned simulation, nonce admission,
  replacement, affordability, byte/count/sender limits and monotonic local TTL.
- `contracts/` contains twelve separately named disposable contracts targeting
  Shanghai. The no-fee acceptance pools use integer constant-product arithmetic;
  they are not reviewed production DEX contracts. First-party fixtures use the
  `LicenseRef-Redcat-Permission-Only` SPDX identifier; use requires prior written
  permission from Redcat. Upstream fixture rights remain separately preserved.
- `client/src/` contains strictly typed test operations using pinned viem and an
  independently maintained EthereumJS proof/RLP implementation. Every included
  Solidity source is compiled by the pinned compiler for the actual run.

Generated compiler artifacts, JavaScript, dependency trees, keys, databases,
debugging scripts and raw logs belong only in ignored root `local-tests/`.
Development signing keys are generated in memory. Only public genesis fields
are written for the task-owned public processes.

## Executable acceptance matrix

| ID | Assertions and bounded scope |
|---|---|
| T-E01 | Signed legacy/type1/type2 corpus transactions, exact canonical bytes, wrong-chain/signature/intrinsic/balance/nonce/initcode rejection, all 88 unprotected policy rejections and nine-byte nonce overflow |
| T-E02 | Full Shanghai reference workload, exact gas/receipts/logs/poststate, real native/ERC20/CREATE/CREATE2/delegatecall/Shanghai destruction/reentry/refund/out-of-gas, two-pool success and transaction-wide second-leg rollback |
| T-E03 | Identical signed transactions, receipts, execution headers and roots through two separate public processes and repositories, selected-header environment/BLOCKHASH and repeat verification after restart; parallel-worker schedules belong to B7 |
| T-E04 | Literal basefee rise/fall/target/floor/overflow, caps/refunds from upstream vectors, exact 40/30/30/odd-wei conservation, no additional Ethereum beneficiary credit or invalid-envelope payout |
| T-E05 | Sender nonce ordering, future gaps, duplicate hashes, competing replacements, cumulative upfront reservation and actual predecessor-state affordability with atomic invalid-block rejection |
| T-E06 | Actual state-changing call/estimation/revert leaves complete canonical account/code/storage/system/nonce/fee/header bytes unchanged; inactive native dispatch fails closed, while activated lifecycle atomicity remains B5 work |
| T-A01 | RPC quantities/data/errors/optional params, transaction identity, canonical 17-field header hash, ordered transaction/receipt roots, genuine fee history, standard storage word input and receipts |
| T-A02 | Real pinned Node/TypeScript/viem and compiled Solidity native/deploy/token/two-pool/revert operations against actual loopback RPC, without mocked responses |
| T-A03 | Unknown versus pending versus unavailable/future state, historical retained reads and explicit unauthenticated-finality errors; implemented pruning/finalized-state behavior remains a later requirement |
| T-A04 | Real mempool count/bytes/sender/TTL boundaries, duplicate/gap/replacement competition, inclusion and committed revalidation |
| T-A05 | Exact body/batch/query bounds, actual worker/byte quotas and cancelled blocking-EVM lease retention; independent public unit cases exercise local progress, while BFT consensus-starvation evidence requires B3/B6 |
| T-A06 | Captured-height account/storage/absence proofs verified by independent EthereumJS, corrupted/wrong-root/wrong-key rejection, ordered history/live events, actual subscriptions/unsubscribe and two-process restart |

The per-method captured view guarantee does not make independent JSON-RPC batch
requests one atomic snapshot. Explicit fixed block selectors are used when
several separate requests must compare the same historical state.

The pinned EthereumJS proof API rejects an empty witness through its missing-node
path even when its expected root is the canonical empty trie root. The test
adapter handles that exact case by independently computing Keccak(RLP empty),
requiring no witness or the exact RLP empty-root node, and a zero value. Nonempty
roots retain strict path
verification; missing nodes and falsely claimed nonzero empty-trie values fail.

`LOCAL_DEV_UNAUTHENTICATED` is explicit development composition. A returned
transaction hash means admission; local receipts and synced storage do not prove
validator finality. `safe`, `finalized` and `eve_getFinalityProof` must not return
invented authenticated results. EVM proofs bind a selected local EVM root; they
do not authenticate an EVE consensus certificate, Ethereum mainnet or a bridge.

## Immutable upstream corpus and policy differences

The source is execution-specs `tests@v20.0.2`, immutable revision
`abbe05777ab83fb94ce18c425daaa7ab79e779c1`, as pinned in
`config/test-corpora.toml`. Its archive is 540,487,005 bytes with SHA-256
`1280540950a4c3470a421416b6f35458a9b635827265c29e5aef1ae839ae1788`.
The twelve reviewed metadata inventories identify 195 original JSON files by
path and SHA-256: 194 state files contain 3495 Shanghai variants, and one separate
transaction fixture contains the nonce-overflow deserialization case.

Every case filled under `for_shanghai` is consumed, including older and newer
family names. The family directory is not the selected execution fork. Other
forks and blockchain/engine formats are separate workloads, not passed cases.

The vanilla reference checks unchanged upstream roots, complete expected state,
exact receipt RLP, log hashes and expected rejection category. It applies
Ethereum EIP-161 touched-empty deletion to the actual returned REVM changes;
untouched explicitly supplied empty leaves are preserved.

The corpus declares 120,000,000 block gas in every state case. This is an explicit
reference environment preserving GASLIMIT, not the live 30,000,000 development
profile. Its 3202 executions include 84 Ethereum-valid unprotected legacy
transactions that EVE deliberately rejects. All 88 unprotected envelopes are
tested as policy divergences, including four Ethereum-invalid cases.

For 3118 protected compatible executions, EVE must retain exact gas, receipt and
non-economic state semantics. Expected economic state is derived only after
the unchanged upstream result is verified: subtract its exact Ethereum priority
credit, apply EVE's collected-fee split and compare every account field/slot/code.
The expected zero-tip/empty-beneficiary rules are not replaced by blanket empty
account filtering. 293 upstream invalid cases remain rejected. These checks do
not claim original Ethereum state-root equivalence under a different fee policy.

The corpus is attributed upstream material under CC0; see
[the immutable source license](https://github.com/ethereum/execution-specs/blob/abbe05777ab83fb94ce18c425daaa7ab79e779c1/LICENSE.md).
The archive may contain explicitly disposable upstream signing seeds. They are
not runtime credentials and are not copied into this repository's tracked inputs.
No upstream filling or setup script is executed.

## Reproduction contract

The integrator's B2 gate provisions verified tools, prepares a bounded safe
extraction, copies exact tracked client inputs into an ignored package, runs
`npm ci --ignore-scripts`, strict TypeScript compilation and all Solidity inputs,
then builds the public/master binaries from the same source before acceptance.

Rust corpus cases require `EVE_SHANGHAI_FIXTURES` to name the extraction root
containing `fixtures/state_tests/for_shanghai/` and the pinned transaction path.
No path fallback or network retrieval occurs inside the test case.

The client entry takes five explicit paths: repository root, new task-owned run
directory under ignored `local-tests/`, verified public binary, verified solc and
verified master composition binary. Simulation rejects unsupported nonce/chain
overrides explicitly. The master process must refuse production, sync-only
producer activation and missing acknowledgement before exposing its listeners.
Each created child is stopped/waited by its creating fixture; interruption is
process evidence, not hardware power-loss testing. Raw local reports are not
published evidence. Use the complete registered B2 gate for integration; an
individual family pass does not close every requirement in this matrix.
