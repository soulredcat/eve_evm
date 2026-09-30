# 12 — Consensus specification

Status: normative development baseline. Exact dependency versions and executable protocol fixtures are pinned in B0. This is not a new consensus proof or a claim that the baseline reaches 1M TPS.

## C01 — Authority and implementation baseline

Use CometBFT through an ABCI++ adapter for the first distributed devnet. Keep the consensus adapter replaceable behind shared interfaces. Do not implement an improvised majority-signature loop or change locking rules for speed. A native Rust/high-throughput replacement requires an ADR, equivalent safety tests and independently reviewable protocol reasoning before activation.

The validator set, not the master operator, determines canonical ordering/finality. Masters follow. Proposer selection and round/timeout/locking behavior come from the pinned engine; there is no separate master sequencer and no application-invented random leader election. The devnet starts with four distinct equal-power validators. Later active-set changes are deterministic application outputs.

The baseline uses weighted BFT. For total active power T and valid unique signer power S, the threshold is strictly `3*S > 2*T`, with overflow-safe arithmetic. Exactly two-thirds is insufficient. Tests must include three-of-four acceptance and two-of-four rejection. Safety assumes less than one-third Byzantine voting power; liveness also needs adequate online power and eventual network delivery. Losing quorum halts finality, not the process's ability to preserve/read existing data.

References: [CometBFT architecture](https://docs.cosmos.network/cometbft/latest/docs/introduction/intro) and [consensus responsibilities](https://docs.cosmos.network/sdk/latest/learn/intro/sdk-app-architecture). The adapter is an engineering baseline, not a Cosmos SDK dependency requirement.

## C02 — Identity and validator-set history

Ethereum transaction keys use the declared EVM signature rules. Consensus keys use the pinned engine's supported Ed25519 implementation initially. P2P, consensus, staking owner, release and master identities are separate domains.

Store validator public keys, power, activation height, removal height and relevant transition commitments. Validate certificates against the set applicable at that height, never the latest set or a list supplied only by the data-serving peer. Count a signer once. Reject unknown, duplicate, revoked-at-height, wrong-chain, wrong-round, wrong-block and malformed signatures.

Follow the engine's canonical sign bytes, block ID and signature verification. Do not substitute a hand-built `hash(height + root)` signature. For ABCI++ validator updates returned at H, test the selected version's H+2 activation semantics and associated H+1/H+3 metadata; do not assume updates apply immediately. Native protocol migrations must explicitly replace this rule, not accidentally bypass it.

## C03 — Proposal validation and application lifecycle

`CheckTx` checks a local mempool candidate without mutating finalized state. Different peers may have different mempools.

`PrepareProposal` selects and orders admissible transactions under block byte/gas limits. `ProcessProposal` deterministically validates the block environment and transaction sequence, including execution against an isolated parent-state overlay. A locally missing mempool transaction is not a rejection reason if the proposal supplies its bytes. Reject invalid envelopes, invalid nonce progression, unavailable required state/data, invalid parameters and gas-limit violations. A valid transaction that reverts in the EVM remains includable with a failed receipt and charged gas.

Proposal processing must not mutate committed state, pay rewards or durably advance height. Speculative caches are keyed by the complete parent state, proposal bytes, environment and protocol/config version. Reusing an overlay for a different proposal is forbidden.

`FinalizeBlock` applies exactly the decided ordered block using the same transition function. `Commit` makes application state crash-safe. The same decided block may be replayed during recovery; all effects, fees, evidence and rewards must be idempotent. No side effects may depend on local wall-clock time, network replies, host identity or task scheduling.

See the official [ABCI methods](https://docs.cosmos.network/cometbft/latest/spec/abci/Methods) and [application requirements](https://docs.cosmos.network/cometbft/latest/spec/abci/Requirements-for-the-Application). Pin and test the actual version instead of assuming all versions expose identical fields.

## C04 — Do not confuse commit certificates with post-state proofs

For the ABCI++ baseline, the application hash returned after execution at height H appears in the consensus header at H+1. A commit certificate for H authenticates its consensus block, not an arbitrary post-state root that somebody attaches afterwards.

Define `ApplicationCommitment(H)` in plan 14. A full replaying node may derive H's state from an authenticated parent and the committed H block. A fast-sync node importing a snapshot/delta at H must authenticate the application commitment through the appropriate later certified header and validator-set history, or perform replay. It must not accept `certificate(H) + attacker_chosen_root(H)` as sufficient.

Use distinct fields/types for consensus height/hash, execution height/EVM hash, authenticated-state height and durable height. Empty consensus blocks remain enabled so the next-height state anchor can be produced even without user transactions. If the network halts before that anchor exists, reconstruct the latest state through replay rather than calling an unattested snapshot certified.

Master and public follower verification must implement this binding, not just compare a delta with its own claimed root. Fast import verifies signatures/headers, validator transitions, commitments, raw block data and root computation. It does not constitute an independent execution proof; advertise replay/fast-import verification mode honestly.

## C05 — Durable signing and recovery

Before emitting a signature, persist enough sign bytes and height/round/step state to prevent signing conflicting messages after a crash. Use the engine's WAL/signer behavior with tested durability and fencing. Never reset sign state to rejoin, run two active copies of one key, restore an old sign-state backup blindly, or assume a container restart preserves safety.

Application durability and consensus durability must agree at recovery. Maintain finalized block material sufficient to replay a decided-but-not-yet-applied block. A validator with uncertain signing state stops signing, reports the condition, and follows plan 16. A master outage never authorizes relaxing these requirements.

## C06 — Data and resource safety

Validators obtain full necessary proposal data and state before approval. A header/hash alone is not data availability. Keep bounded consensus queues independent of RPC/snapshot bulk traffic. RPC overload may shed public requests; it must not skip execution validation or fsync.

Recent finalized blocks/checkpoints must remain durable on validators or independently verified recovery sources. Pruning is governed by plan 15. If durability cannot be preserved, fail closed instead of signing a state that cannot be recovered.

## Acceptance

T-C01: four validators commit identical results with changing proposers.
T-C02: one offline validator permits progress; two offline stop finality.
T-C03: a 2+2 partition cannot produce two accepted final histories; heal and recover.
T-C04: reject wrong-chain, duplicate-signer, exact-two-thirds and wrong-set certificates.
T-C05: inject invalid proposals and correct reverting transactions; only the former invalidate the proposal.
T-C06: crash around signing/commit; no conflicting signature and no duplicated effects.
T-C07: rotate/leave/jail validators; verify certificates across activation boundaries.
T-C08: forged master root with an unrelated valid certificate is rejected.
T-C09: snapshot H is checked against the correct H+1 anchor; off-by-one proofs fail.
T-C10: master offline and RPC saturation do not become consensus dependencies while quorum/data are intact.
