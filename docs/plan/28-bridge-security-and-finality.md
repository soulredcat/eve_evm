# 28 — Bridge security, finality and quantum boundaries

Status: mandatory design and local implementation/testing work, added 2026-09-30. No live bridge, custody deployment, external route approval or security certification is implied.

## BRS01 — Scope and initial route

Build reusable bridge verification/accounting components, not an unrestricted cross-chain mint endpoint. The first executable fixture connects two task-owned EVE devnets with different chain IDs/genesis hashes and fake native/ERC-20 test assets. This provides a concrete end-to-end target without inventing an external chain's API or claiming that every route already works.

Alephium and other external routes require separate researched adapters and route-specific approval. The owner's security requirement authorizes planning, local code, fixtures and tests; it does not authorize real funds, production keys or mainnet deployment. Keep unsupported routes disabled with a precise reason and resume task.

## BRS02 — Trust model per route

For every direction, record source/destination genesis and chain identity, custody contract/module, token identifiers, verifier version/code hash, finality model, trust anchor freshness, validator-set updates, proof limits, upgrade policy and quantum profile. Token symbols or matching addresses on different networks are not asset identity.

Prefer authenticated light-client/header and inclusion verification where implementable. A quorum-signature bridge has an additional signer/custody trust assumption; label it honestly. A relayer is an untrusted transporter, never a source of chain truth. Several relayers/RPC endpoints do not create several independent consensus systems.

A bridge inherits the security assumptions of both endpoints and its verifier/custody/admin paths. Adding ML-DSA to relayers does not repair quantum-vulnerable source finality, user authority or destination custody. A proof of inclusion is not automatically proof of finality or solvency. Background: [Ethereum bridge security/trust models](https://ethereum.org/en/developers/docs/bridges/).

## BRS03 — Verification before value movement

The destination verifies: authenticated source history and applicable validator set; correct finality/profile; the correct state/receipt root binding; inclusion of the expected successful custody event; route/asset/amount/recipient; replay protection; limits; and current pause status. Only then may it atomically consume the message and mint/unlock.

For the EVE CometBFT fixture, preserve plans 12/14's consensus-versus-execution height and H/H+1 application commitment distinction. Receipt/event inclusion must be bound through the execution header, application commitment and authenticated consensus anchor, or established by full verified replay. `certificate(H) + arbitrary_root(H)` is not enough.

A source quorum loss stalls new proven transfers. A master outage does not waive source finality. Wrong-network proofs, stale trusted headers, unknown profiles and missing data fail closed. Unknown facts are not replaced with a relayer's success response.

## BRS04 — Message identity and replay

Freeze canonical byte encodings in the implementation fixture. A message includes:

```text
protocol/profile version
source genesis + chain ID
destination genesis + chain ID
route ID + source custody identifier
destination custody identifier
source height + block hash + transaction index + log/event index
asset identifier + exact base-unit amount + recipient
route sequence/nonce + any expiry/refund policy
```

The unique message ID commits to all fields using the reviewed hash/domain policy from plan 27. Bind the selected route/profile and destination deployment so replay across forks, testnets, contracts or security-profile versions fails. Compression/relayer encoding is not signed-message identity.

Persist consumed IDs and accounting in the same atomic state transition as mint/unlock. Retries after crashes are idempotent. Events emitted by lookalike contracts, reverted transactions or a different asset route are rejected.

## BRS05 — Accounting and custody

Use a simple allowlisted one-origin/one-destination lock-mint and burn-unlock model first. Wrapped supply is a liability backed by actual source custody, not expected transfer amounts. Explicitly account for finalized pending deposits, wrapped outstanding balances and proven burned-but-not-yet-released liabilities. No relayer or master can mint outside this ledger.

Test value conservation per asset/direction with checked integer arithmetic and exact decimals. Initially reject fee-on-transfer, rebasing, callback-bearing or otherwise unsupported tokens rather than assuming their nominal transfer amount was received. Later support requires separate verified semantics.

Cover reentrancy, failed transfers, revert rollback, double claims, duplicate events, contract upgrades and unauthorized token mapping. Administrative recovery cannot sweep locked backing or change a pending recipient. No arbitrary-call bridge or general message execution is enabled until its separate threat model and tests pass.

A timeout alone does not prove the destination never minted. The initial bridge has no unilateral timeout refund. A later refund/cancellation protocol must prove mutual exclusion between mint and refund, including delayed messages and partitions.

## BRS06 — Containment and incident response

Require per-transfer, per-window and total outstanding exposure limits for each route/asset, plus a configurable release delay and independently operated watchers. Development fixtures use bounded fake-token values; real economic parameters require explicit approval. Window/expiry rules affecting chain state use agreed protocol time/height, not local process clocks.

Implement authenticated pause evidence, stale-client rejection, rate limiting, alerting and a pending-transfer queue. An emergency pause role can stop future bridge execution but cannot mint, unlock, alter proofs, rewrite history or resume unilaterally. Resume requires the documented authorized policy, fresh verified anchors and reconciliation.

Monitors can be delayed, censored or partitioned. Delays and caps contain exposure only under their stated assumptions; they do not guarantee that a compromised source quorum will always be detected before release. Slashing source validators does not restore assets already released on another chain.

No blanket “51% safe bridge” claim: plans 26 and 27 apply to all source-proof and custody paths. A light client may accept a forged but cryptographically valid-looking history after its own source assumptions fail. Fail closed on observable violations, and keep that residual risk explicit.

## BRS07 — Post-quantum route profile

Maintain a per-route matrix for source consensus signatures/commitments, user authorization, destination verification, custody/admin/recovery keys and release trust. A route is `PQ_END_TO_END_VERIFIED` only when every required path meets the named profile and tests. Otherwise use `CLASSICAL` or `MIXED_TRUST`, never an end-to-end PQ claim.

For a hybrid signer-based experiment, count only identities whose required classical AND PQ components verify over the same message and current key epoch. This still adds signer trust and does not strengthen the underlying chain. Do not assume the destination VM can afford PQ verification: benchmark byte limits, gas and worst-case malformed input before enabling the adapter.

Any ZK/proof-compression substitution needs a reviewed quantum threat model for the proof system and setup as well as its verifier; a proof is not automatically post-quantum. Do not use an unverified proof-compression claim to remove source authentication.

## BRS08 — Module boundaries and folder policy

Create these domains when implementation reaches them; they are not existing binaries:

```text
crates/bridge/src/
  routes/registry/validation/validate_route_profile.rs
  clients/finality/verification/verify_source_finality.rs
  proofs/receipts/inclusion/verify_custody_event.rs
  messages/replay/consumption/consume_bridge_message.rs
  accounting/backing/reconciliation/reconcile_asset_backing.rs
  limits/exposure/checks/check_release_limit.rs
  incidents/pausing/evidence/verify_pause_evidence.rs
contracts/bridge/
integration/bridge/
```

Share finality/crypto/state contracts instead of cloning validators. Master stores finalized data; it has no bridge signer authority. Follow plan 25: meaningful recursive subfolders, one behavioral function per file, target 200 lines, reviewed 400/600 thresholds. Keep transport/relayer logic separate from deterministic verification and custody effects.

## BRS09 — Acceptance

| Test | Required evidence |
|---|---|
| T-BR01 | Two real local EVE networks execute lock-mint and burn-unlock with fake assets and verified proofs; no mocked finality. |
| T-BR02 | Wrong genesis/chain/route/custody/recipient/amount/profile and malformed proofs fail without value movement. |
| T-BR03 | Duplicate delivery, cross-direction replay, process crash and restart produce exactly-once economic effects. |
| T-BR04 | Reverted/log-lookalike transactions, invalid receipt inclusion and H/H+1 root misbinding are rejected. |
| T-BR05 | Source halt, stale trust, malicious RPC/master and missing data never permit proof bypass or emergency minting. |
| T-BR06 | Route limits, release queue, authenticated pause and controlled resume work under simulated delay, partition and concurrent claims. |
| T-BR07 | Conservation invariants cover pending deposits/burns, fees if explicitly configured, decimals, transfer failures and unsupported tokens. |
| T-BR08 | Reentrancy, admin/upgrade takeover, key rotation and replayed old signers fail according to the active authorization profile. |
| T-BR09 | The named hybrid fixture enforces both signature components through actual destination verification; classical endpoint limitations remain visible. |
| T-BR10 | Conflicting source-finality evidence stops the affected client/route when observed; test/report delayed-detection residual exposure, not universal majority protection. |
| T-BR11 | No timeout-only refund can race a delayed mint; unsupported refund routes remain disabled. |
| T-BR12 | Destination proof/PQ verification gas, bytes, latency and invalid-input CPU cost are measured and bounded. |

## BRS10 — Completion

`BRIDGE_DEVNET_ACCEPTED` requires T-BR01–T-BR12 for the declared fixture/profile. External route readiness requires a pinned real adapter, chain-specific positive/negative fixtures, independent review and explicit owner approval. No parameter change or README turns a two-EVE devnet result into an approved Alephium/Ethereum/mainnet bridge.
