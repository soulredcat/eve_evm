# 30 — Ethereum, Solana and extensible bridge interoperability

Status: mandatory planning and implementation requirement, added 2026-09-30. No adapter, deployed route, provider partnership, token backing or bridge acceptance is claimed complete.

## I01 — Owner requirement and scope

EVE must be bridge-friendly to Ethereum, Solana and other chains without rewriting its core for each integration. Ethereum and Solana are the first named external integration targets in both directions. The two-EVE fixture in plan 28 remains the foundation, not the final interoperability deliverable. Alephium and additional chains remain extensible route candidates, not silently activated or promised integrations.

EVE remains an EVM execution environment. Solana interoperability does not require embedding Solana's runtime or running Solana programs as Solidity contracts. It requires explicit source/destination adapters, verifiers, custody endpoints, asset mappings and wallet/SDK support. Compatibility, route availability, fault tolerance and quantum protection are separate claims.

Deliver source code, local fake-asset fixtures, documented interfaces and verifier research/implementation under INT0–INT3 below. No live custody, real-token issuance, paid provider enrollment, external trust downgrade or production deployment is authorized. Implement unblocked work; keep any unverified route disabled with its precise blocker.

## I02 — Stable architecture and responsibilities

```text
Ethereum contracts <-> Ethereum adapter --\
                                          > Bridge core <-> EVE endpoint
Solana programs    <-> Solana adapter   --/
Other chain        <-> Separate adapter --/

Wallet / SDK -> source-chain transaction -> authenticated source evidence
            -> untrusted relayer -> destination verifier -> atomic claim
```

Keep five boundaries explicit: chain transport/observation; chain-specific finality and event verification; shared message/replay/backing accounting; destination custody execution; developer/wallet presentation. An RPC observation is not a verified bridge message. Never upgrade an untrusted observation into a verified type merely because several RPCs agree.

Define typed interfaces during INT0: `ChainIdentity`, `ChainAddress`, `AssetOrigin`, `RouteManifest`, `SourceObservation`, `SourceFinalityVerifier`, `VerifiedBridgeMessage`, `DestinationExecutor` and `BridgeQuote`. Verification capability constructors are restricted to real checked paths. Consensus-bound handlers receive authenticated data, not live external RPC results. Master stores finalized data and has no bridge approval, custody key or mandatory per-transfer round trip.

Transport retries, polling, subscriptions and proof fetching occur outside deterministic EVM execution. Destination verification, consumed-message state and mint/unlock effects execute under that chain's own atomicity rules. P2P/RPC distribution may scale independently; more relayers are not more independent consensus security.

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

Plans 26–29 remain mandatory. Route crypto coverage includes both source consensus/user authority and destination custody/verifier/admin paths. EVE PQ signatures do not upgrade Ethereum or Solana by association; classify residual classical/external dependencies as `CLASSICAL` or `MIXED_TRUST` where applicable. Compatibility tests may pass without a whole-route PQ claim; they cannot close the independent PQ end-to-end requirement.

Unknown finality, stale anchors, fork/profile changes, mismatched deployments, unexpected token extensions or unavailable verification pause the affected route. No master-only emergency mint, lower quorum, unauthenticated RPC fallback or silently changed signer policy is allowed.

Report source-finality wait, proof production/verification, destination fees/compute, relayer backlog, end-to-end latency, bytes and completed bridge throughput separately from EVE transaction TPS. Local fast execution cannot make another chain's finality instant. Bridge overload must be bounded and isolated from unrelated EVE consensus/RPC workloads.

## I09 — Recursive source organization

Create real modules when their tasks start; these are planned paths, not existing implementations:

```text
validator/components/bridge/src/
  routes/capabilities/validation/validate_route_capabilities.rs
  assets/origin/mapping/resolve_asset_representation.rs
  assets/amounts/conversion/convert_exact_amount.rs
  messages/claims/replay/consume_transfer_once.rs
  adapters/ethereum/finality/verification/verify_ethereum_anchor.rs
  adapters/ethereum/events/custody/verify_ethereum_deposit.rs
  adapters/solana/accounts/custody/validate_solana_custody.rs
  adapters/solana/finality/verification/verify_solana_evidence.rs
  adapters/eve/finality/verification/verify_eve_application_anchor.rs
contracts/bridge/evm/
programs/bridge/solana/
sdk/bridge/src/chains/ethereum/transactions/build_deposit.ts
sdk/bridge/src/chains/solana/transactions/build_deposit.ts
integration/bridge/ethereum/
integration/bridge/solana/
```

Shared accounting/crypto contracts remain single-source. Do not build one giant switch over every chain or duplicate the EVM engine. Follow plan 25 at every depth, including Rust, Solidity, TypeScript and test/generator code with explicit language coverage; unsupported checker coverage is work, not a silent pass.

## I10 — Mandatory INT0–INT3 execution queue

| Bulk | Dependencies | Implementation and exit evidence |
|---|---|---|
| INT0 | Within B0 + SEC0 | Pin Ethereum/Solana references/toolchains; finalize typed chain/asset/route formats and source-proof feasibility for both directions; register T-I01–T-I12; record real integration blockers. No core/security cycle. |
| INT1 | B2+B3+B4+B5+B6 and SEC2 shared interfaces | Implement shared SDK/route registry plus Ethereum contracts and Solana program/adapters with local fake-asset application fixtures. Independent chain tracks may run in parallel. Label simulator-only and classical evidence. |
| INT2 | INT1 + SEC2 applicable proof/accounting gates | Integrate actual source authentication and destination verification for each named direction; test real client/consensus evidence, negative cases and costs. Secure EVE-profile acceptance additionally requires SEC1. A placeholder verifier or RPC-only observation cannot pass. |
| INT3 | INT2 + B8/B9 relevant recovery/release gates | Run fresh-checkout Ethereum/EVE and Solana/EVE transfers and returns, SDK/retry/incident tests, route matrix and independent review artifacts. No live route is enabled by local success. |

Ethereum and Solana work may proceed independently after shared interfaces. If one verifier needs unavailable infrastructure or unapproved extra trust, record its exact blocker and continue endpoint/SDK/other-route work. Do not close the named integration target by substituting two EVE networks, or change core/security completion to hide the unmet target.

Implement `cargo xtask verify --interop INT0` through `INT3` in the task runner. These are planned commands, not available tools claimed here. Include structural gates, fail on zero/missing requested tests, and record profile, endpoints, source/verifier versions, real test count and evidence hashes. Mainnet economics, funds, deployment and extra trust approvals remain owner gates.

## I11 — Required tests

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

Record `ETHEREUM_INTEROP_DEV`, `SOLANA_INTEROP_DEV`, `INTEROP_DEV_ACCEPTED`, per-direction verifier status, trust mode, quantum coverage and live approval separately. Initial values are NOT_STARTED/NOT_ACHIEVED; live routes are DISABLED_NOT_APPROVED. No generic SUPPORTED flag hides an incomplete direction.

`INTEROP_DEV_ACCEPTED` requires INT0–INT3 and T-I01–T-I12 on actual integrated code under the explicitly declared local/test profile. Simulator-only contracts or stubs are partial progress, not this acceptance. A two-way Ethereum result does not complete Solana or arbitrary other chains. End-to-end PQ, stronger majority tolerance and mainnet readiness remain their own evidence/approval gates.
