<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# 30 — Deferred external-chain adapter programs

Status: retained future separate-program specification. D40 supersedes the earlier mandatory external implementation queue: INT0–INT3 and T-I01–T-I12 are `DEFERRED_UNTIL_EVE_TESTNET`. Subsequent owner scope is required; no adapter, route, provider partnership, token backing or bridge acceptance is claimed complete.

## I01 — Owner requirement and scope

EVE's own chain is the current goal. External bridges, remote-chain clients, custody and adapters must not be developed now or remain prerequisites for core acceptance. External work is eligible only after EVE reaches testnet, in subsequently authorized separate programs whose adapters conform to EVE. Ethereum/Solana two-way targets, the two-EVE bridge fixture and additional-chain candidates are retained future requirements, not active assignments.

EVE remains an EVM execution environment with its declared Shanghai/RPC semantics and reference corpus. EVM standards, Ethereum libraries and local compiler/client/correctness fixtures do not require a remote-chain connection and remain core. Future Solana interoperability would use its actual runtime/account/token/signing semantics outside EVE, with explicit adapters/verifiers/custody and wallet support. Compatibility, route availability, fault tolerance and quantum protection remain separate claims.

Do not create a replacement program prototype, adapter package or unused core hook now. Historical INT0 source/tool research and metadata tests remain factual evidence at their recorded revision. All requirements below apply only to a later separate owner scope. EVE testnet does not automatically authorize development; live custody, real-token issuance, paid providers, external trust downgrade and production deployment remain unapproved.

## I02 — Stable architecture and responsibilities

```text
Ethereum contracts <-> Ethereum adapter --\
                                          > Separate bridge program <-> EVE public APIs
Solana programs    <-> Solana adapter   --/
Other chain        <-> Separate adapter --/

Wallet / SDK -> source-chain transaction -> authenticated source evidence
            -> untrusted relayer -> destination verifier -> atomic claim
```

Keep five boundaries explicit: chain transport/observation; chain-specific finality and event verification; shared message/replay/backing accounting; destination custody execution; developer/wallet presentation. An RPC observation is not a verified bridge message. Never upgrade an untrusted observation into a verified type merely because several RPCs agree.

Define typed route/chain/asset/observation/verification/quote interfaces within the separately authorized program when its INT0 scope begins. Historical type names do not justify retaining those interfaces as core dependencies. Verification capability constructors are restricted to real checked paths. EVE provides versioned own-chain headers, receipts, finality and proofs through public APIs; master has no bridge approval, custody key or mandatory per-transfer round trip.

Transport retries, polling, subscriptions and proof fetching occur outside deterministic EVM execution. Future adapter submission uses ordinary authenticated EVE transactions and destination effects follow that chain's atomicity rules. EVE block production, voting, finality, transaction latency and durable acknowledgement cannot depend on a remote chain's availability, confirmation speed or acknowledgement. Bound ordinary RPC/mempool admission so adapter overload cannot starve core workloads; more relayers are not more independent consensus security.

## I03 — Route identity and supported-capability registry

Each direction has an authenticated, versioned manifest binding source/destination namespace and immutable genesis identity; local numeric IDs where applicable; custody contract/program IDs and implementation identity; verifier type/version; source finality policy; anchor freshness; asset mapping; encoding; fee/amount/resource limits; upgrade authorities; pause policy; crypto coverage; and evidence references.

Ethereum chain ID and an EVE chain ID are not Solana cluster identity. Use full chain-scoped raw address bytes with canonical validated display encodings. EVM account addresses and Solana account addresses have different widths and validation rules; never truncate a Solana address to fit an EVM address. Bind the intended destination recipient, including any separate token-account identity or derivation rule.

Runtime APIs expose route states such as `NOT_IMPLEMENTED`, `LOCAL_FIXTURE_ONLY`, `VERIFIER_INCOMPLETE`, `REVIEW_REQUIRED`, `ENABLED_FOR_NAMED_NETWORK` and `PAUSED`. Fixture scope, integration readiness, owner deployment approval and crypto profile are separate fields. Unknown routes/assets fail closed; discovery or adding an RPC URL cannot activate minting.

Adding another EVM chain may reuse transaction/ABI codecs, but not Ethereum's finality rules without verification. Rollups, PoW chains and other consensus systems require their own settlement/reorg/trust policies. Route IDs and configurations are not evidence of support.

## I04 — Ethereum integration

Implement EVE <-> Ethereum custody/claim contracts and the chain adapter with pinned Solidity ABI/events, transaction building, gas estimates, fee caps, nonce/replacement handling and receipt decoding. Compile EVE-side contracts for EVE's declared supported fork; independently parse the actual Ethereum source fork and header/proof format. EVE's initial Shanghai execution choice is not permission to reject all later Ethereum source headers or pretend they share one format.

For a direct light-client route, authenticate the appropriate Ethereum finalized beacon history from a valid trust anchor, sync-committee transitions and fork-specific execution-payload binding, then verify successful receipt/event or custody-state inclusion. A JSON-RPC `finalized` string, elapsed delay or transaction receipt alone is not the destination proof. Do not turn an optimistic light-client update into permission to release finality-protected funds.

The reverse direction needs a destination verifier of EVE consensus history, validator/profile transitions and the execution/application commitment binding in plans 12/14. Deploying a Solidity escrow alone does not verify EVE. Measure verifier/precompile support, proof bytes and gas for each direction; EVM compatibility does not guarantee EVE's consensus or PQ verifier runs cheaply on Ethereum.

Native ETH and ERC-20 assets are separate asset kinds. Use explicit lock/mint and burn/unlock with actual received balances and maintained safe token interactions. Reject rebasing, transfer-tax and unsupported callback semantics until their own tests exist. A local execution simulator proves contract behavior only; Ethereum finality requires actual consensus fixtures/client evidence, not a renamed local mining API.

Primary references: [Ethereum bridge trust models](https://ethereum.org/en/developers/docs/bridges/) and [Ethereum consensus light-client specifications](https://ethereum.github.io/consensus-specs/specs/altair/light-client/sync-protocol/). The latter is the base protocol; pin the target network's applicable fork extensions and source revisions during INT0.

## I05 — Solana integration

Implement a separate Solana adapter and native on-chain custody/claim program. Use Solana account/program/instruction semantics, not Ethereum logs/MPT proofs by analogy. Bind cluster genesis, deployed program identity, mint, token program, custody account, recipient authority and any associated token-account derivation. Check account ownership, signer/writable permissions, PDA seeds/bump, mint/freeze/upgrade authorities and all CPI targets. Never accept caller-supplied accounts as trusted solely by position.

The first asset scope is native SOL and explicitly allowlisted standard SPL tokens with separately implemented native wrapping behavior. Treat Token-2022 as a capability matrix: inspect the real mint/account extensions, reject unknown or unsupported combinations and test any enabled transfer fee, hook, delegate, freeze or other special behavior. Equal symbols do not identify the same mint or token program.

Source observations use an explicit commitment policy, with finalized data required by the default transfer route. `processed`, `confirmed`, a RPC log, or a transaction signature alone is not proof of finalized successful custody. Implement an authenticated source-verification design for the actual Solana protocol or an explicitly reviewed alternative with its additional trust assumptions. A Solana RPC commitment response is not a self-authenticating light-client proof. Do not invent an Ethereum-style receipt trie or assume a small universal finality certificate exists.

The EVE -> Solana path must enforce EVE finality/profile verification inside the actual authorized destination claim path. Measure compute, transaction size, account size, account creation funding and proof staging on the pinned Solana version. If proofs require staged accounts, bind their chunks/hash/length/author and route, cap resources and expiry, and prevent substitution or premature execution. Do not bypass PQ checks because the proof does not fit.

Client code handles recent-blockhash expiry, transaction versions, re-signing/retry, fees and recipient account creation. A new transaction signature is not a new economic bridge entitlement. Source failure cannot create a bridge deposit, even if diagnostic logs mention the bridge. A single local test-validator run is application integration evidence, not proof of mainnet fault resistance or source-finality verification.

Primary references: [Solana accounts](https://solana.com/docs/core/accounts), [programs](https://solana.com/docs/core/programs), [transactions](https://solana.com/docs/core/transactions), [RPC commitment and methods](https://solana.com/docs/rpc), and [Token Extensions](https://solana.com/docs/tokens/extensions). Pin actual SDK/program/tool versions and active limits rather than copying changing constants into this plan.

## I06 — Assets, amounts and asynchronous transfer semantics

Canonical asset identity is origin network/genesis plus native-asset kind or exact contract/mint and token-program identity. Representations additionally bind the approved route and destination deployment. Preserve origin metadata across wrapping; do not present a bridge-issued token as issuer-native USDC/USDT or another issuer asset without verified issuer support. Begin with fake development assets, not real branded token contracts.

Amounts use checked integers in native base units. Specify source/destination decimals, conversion factor, maximum representable amount and dust handling per mapping. Never assume 18 decimals or cast a 256-bit amount into a smaller token-program amount unchecked. Default to rejecting non-exact conversions and overflow; later dust/refund accounting needs tests. Display rounding cannot change custody amounts.

Use one-origin/one-destination custody partitions first. Multiple routes must not pledge the same backing twice. Cross-route forwarding, third-chain wrapped inputs, liquidity pools and issuer burn/mint are later capabilities requiring explicit conservation rules and authorization. Reuse verified origin identity, not recursive wrapping as a substitute for backing.

Each transfer has a stable economic identity based on source custody and its authenticated sequence, bound to route, origin asset, exact amount and destination. Inclusion locators/proof bytes remain authenticated but retries, new relayers and re-signed transport transactions cannot create a second claim. Destination claim consumption and value effects are atomic and durable.

Expose `SOURCE_SUBMITTED -> SOURCE_FINALIZED -> PROOF_VERIFIED -> DESTINATION_SUBMITTED -> DESTINATION_FINALIZED`, plus explicit waiting/paused/failed states. A cross-chain operation is asynchronous, not one atomic transaction spanning independent chains. No source refund follows merely from a destination timeout; preserve plan 28's mutually exclusive refund/mint rule.

## I07 — Developer and wallet experience

Provide a versioned TypeScript SDK with common discovery/quote/status interfaces and separate EVM and Solana transaction/signing adapters. Suggested operations are `listRoutes`, `getQuote`, `buildDeposit`, `getTransferStatus`, `buildClaim` and `resumeTransfer`; freeze actual signatures with type-checked examples. Do not publish a package or claim an integration before builds/tests exist.

The wallet signs the source chain's actual format using its own keys. EVM and Solana wallets are separate signer capabilities; do not reuse private keys, silently derive matching addresses, force every wallet into EVM signing, or promise legacy wallets can produce EVE PQ authorization. Protected EVE accounts follow plan 27; unavailable wallet/profile support returns a clear error.

A quote binds route/version, asset/amount, intended recipient, source fee, destination fee, relayer fee if any, account-creation funding, minimum receive, expiry and trust/profile disclosures. Quotes are estimates, not proof or guaranteed arrival time. Do not alter the EVE 40/30/30 transaction-fee split to hide an external bridge charge. Source/destination gas assets differ; sponsored claims must have explicit budget and policy rather than silently assuming users already have destination gas.

Provide resumable status by stable transfer ID, transaction references on both endpoints, duplicate-safe retries and a manual claim path where the chosen protocol permits. Relayers may improve delivery, but cannot redirect recipients or override proof checks. General cross-chain arbitrary calls/NFTs/swaps are not bundled into basic token support; add them only with separate capability tests.

Existing interoperability providers may be researched behind an optional transport/verifier boundary. Verify actual EVE integration eligibility, deployed code, trust assumptions, limits and licensing; do not claim a provider already supports EVE. An additional attestation committee is an explicit trust change, not an automatic fallback for a missing light client.

## I08 — Security and performance boundaries

Plans 26–27 and SEC0/SEC1/SEC3 remain mandatory core security; plans 28/30 retain deferred program requirements. Future route crypto coverage includes source consensus/user authority and destination custody/verifier/admin paths. EVE PQ signatures do not upgrade external endpoints by association; classify their residual dependencies as `CLASSICAL` or `MIXED_TRUST`. Compatibility tests may pass without a whole-route PQ claim; they cannot close the future program's independent PQ requirement.

Unknown finality, stale anchors, fork/profile changes, mismatched deployments, unexpected token extensions or unavailable verification pause the affected route. No master-only emergency mint, lower quorum, unauthenticated RPC fallback or silently changed signer policy is allowed.

Report source-finality wait, proof production/verification, destination fees/compute, relayer backlog, end-to-end latency, bytes and completed bridge throughput separately from EVE transaction TPS. Local fast execution cannot make another chain's finality instant. Bridge overload must be bounded and isolated from unrelated EVE consensus/RPC workloads.

## I09 — Separate-program source organization

Choose actual repositories and module paths only when future program scope exists. The separate program owns chain adapters, contracts/programs, route policy, backing/replay accounting, SDK and tests. Core public/validator/master packages own only EVE behavior and cannot import remote-chain clients or an external-chain registry. Use canonical versioned EVE evidence/transactions without copying the EVM engine, consensus or private master implementation.

Keep each chain's verifier, custody, transport and wallet responsibilities distinct; do not create one giant switch over every chain. Follow plan 25 at every depth, including Rust, Solidity, TypeScript and test/generator code with explicit language coverage. Unsupported checker coverage is work, not a silent pass. No empty scaffold is required now.

## I10 — Deferred INT0–INT3 program queue

All IDs below are `DEFERRED_UNTIL_EVE_TESTNET`, excluded from current core selection. These dependencies describe future work after EVE testnet and subsequent owner scope; the old B0+INT0 integration remains historical evidence only.

| Bulk | Dependencies | Implementation and exit evidence |
|---|---|---|
| INT0 | EVE testnet + subsequent owner program scope | Review historical inventory; pin actual target-chain tools/references, typed interfaces and direction-specific proof feasibility for authorized targets; register program gates. No dependency back into core. |
| INT1 | Program INT0 + applicable separately scoped SEC2 interfaces | Implement program SDK/registry and authorized contracts/programs/adapters with fake-asset local fixtures. Independent chain tracks may run in parallel. Label simulator-only and classical evidence. |
| INT2 | INT1 + SEC2 applicable proof/accounting gates | Integrate actual source authentication and destination verification for each named direction; test real client/consensus evidence, negative cases and costs. Program acceptance under a verified EVE profile requires SEC1 evidence. A placeholder verifier or RPC-only observation cannot pass. |
| INT3 | INT2 + B8/B9 relevant recovery/release gates | Run fresh-checkout Ethereum/EVE and Solana/EVE transfers and returns, SDK/retry/incident tests, route matrix and independent review artifacts. No live route is enabled by local success. |

After authorization, chain tracks may proceed independently. If a verifier needs unavailable infrastructure or unapproved extra trust, record the program blocker and continue its other authorized work. Do not substitute two EVE networks for a named external integration. Core completion remains independent of future program completion, while route status remains explicit.

A future program must implement its own nonzero verification gates with ownership/structure checks, source/verifier/profile identities and real test counts. Current root runner requests for INT0–INT3 report the deferral without a passing empty gate. Historical `cargo xtask verify --interop INT0` evidence applies only at its recorded revision. Mainnet economics, funds, deployment and extra trust approvals remain owner gates.

## I11 — Retained future program tests

T-I01–T-I12 below are deferred program requirements, not current core failures or achievements.

| Test | Required evidence |
|---|---|
| T-I01 | Chain/genesis/address namespace and width checks reject cross-network collisions, truncation and wrong recipient/token accounts. |
| T-I02 | Route registry rejects unknown/incomplete/paused routes; a new mock adapter proves isolation without being advertised as an integrated chain. |
| T-I03 | Ethereum fake ETH/ERC-20 lock/mint and burn/unlock use verified source evidence and real destination contracts in both directions. |
| T-I04 | Solana fake SOL/SPL round trips validate program/account/PDA/mint/token-program/authority and exact custody effects. |
| T-I05 | Nonfinal/stale/forked/wrong-root/failed-event evidence cannot move value; RPC labels are insufficient in both adapters. |
| T-I06 | Decimals, zero/dust, overflow, native wrapping and multi-route backing invariants pass; unsupported token behavior fails closed. |
| T-I07 | Solana blockhash retries, duplicate relayers, destination revert/crash and delayed claims produce exactly-once economic effects without timeout refunds. |
| T-I08 | Typed SDK examples, source wallet signing, recipient verification, fee disclosure, gas/account-funding and resume/manual-claim paths work. |
| T-I09 | Active EVE profile and historical key sets are enforced on each destination; external classical dependencies are visible, not renamed PQ. |
| T-I10 | Proof/gas/compute/size/staging limits, invalid-input floods and relayer backlog stay bounded with real measured costs. |
| T-I11 | Contract/program/verifier upgrades, provider changes, token extensions and circuit breakers cannot bypass route approval or backing. |
| T-I12 | Reproducible packages run both named integrations independently; local simulation, authenticated-verifier evidence and deployment approval are reported separately. |

## I12 — Completion and reporting

Current `ETHEREUM_INTEROP_DEV`, `SOLANA_INTEROP_DEV`, `INTEROP_DEV_ACCEPTED` and INT0–INT3 scope are `DEFERRED_UNTIL_EVE_TESTNET`; live routes remain DISABLED_NOT_APPROVED. Historical metadata tests are neither route acceptance nor a current core failure. A future authorized program records verifier status, trust mode, quantum coverage and approval per direction; no generic SUPPORTED flag hides an incomplete route.

Future `INTEROP_DEV_ACCEPTED` requires the separately authorized program's INT0–INT3 and T-I01–T-I12 on actual integrated code under its declared local/test profile. Simulator-only contracts or stubs are partial progress, not acceptance. A two-way Ethereum result does not complete Solana or arbitrary other chains. Core security acceptance remains independent; whole-route PQ, stronger majority tolerance and deployment retain their own evidence/approval gates.
