# Validator bridge protocol boundary

Canonical owner: validator bridge verification/execution contracts. This component
uses the canonical public interoperability metadata component and exposes
`SourceObservation`, `SourceFinalityVerifier`, restricted `VerifiedBridgeMessage`,
`DestinationExecutor` and `BridgeQuote` contracts required by plan 30 INT0.

Observation validation checks full namespaces/custody/origin/recipient/amounts,
chain-specific inclusion locators, policy reference namespaces and proof resource
bounds. A bounded observation is explicitly unauthenticated. Ingress must bound
decoding before allocating it; a matching root or RPC label is insufficient.

No public/default/deserialization or observation-to-verified constructor exists.
Compile-fail doctests prove external code cannot fabricate `VerifiedBridgeMessage`.
There is no positive finality verifier or custody executor implementation in B0.
A future reviewed checked verifier inside the canonical verified-message domain
must authenticate historical source consensus/profile/anchor and successful
custody inclusion before constructing that capability. Relayers cannot construct
it, and master has no approval/finality/custody authority.

The destination trait requires actual chain-owned atomic/durable replay consumption
and custody effects. An RPC-submitted transaction is not its checked receipt.
Economic transfer identity and proof-locator identity remain distinct; retrying or
cloning a verified capability cannot bypass the later replay/backing ledger.

`SourceVerificationContext` distinguishes policy anchor, source-verifier identity,
required historical authentication profile and destination implementation from
public route-binding metadata. They are reference inputs, not authenticated
approval. Complete deployment/admin/upgrade/exposure/pause authorization is SEC2
work; B0 exposes no route-approval API. Public metadata v1 remains non-approval.

Quotes carry exact source/destination/minimum amounts, chain-scoped recipient,
source/destination/relayer/account-creation fees, protocol-height expiry and crypto
disclosure. They are estimates, not authenticated finality or timeout-refund rights.
No external bridge charge changes the EVE 40/30/30 transaction-fee policy.

Run `cargo test --locked -p eve-bridge-protocol`, including compile-fail doctests.
These metadata/interface tests do not close T-BR or complete T-I03–T-I12, prove
external source finality, implement native wrapping/backing or approve live custody.
Standalone role packaging remains a separate unimplemented gate.
