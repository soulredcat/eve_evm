<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Validator-owned CometBFT boundary

Canonical owner: `validator/components/consensus-comet/`. This component owns the
pinned engine's ABCI/private-validator wire bindings, bounded socket framing,
height mapping, native classical canonical bytes/hashes/certificate checks and
explicit authentication capability boundary. Complete application, signing
durability and consensus lifecycle wiring belong to the validator runtime.
Dependencies flow from role runtimes/verifiers into this component; this component
has no private master dependency.

Read plans 12 (local-only: `../../../docs/plan/12-consensus-spec.md`),
14 (local-only: `../../../docs/plan/14-block-and-state-commitment-spec.md`),
25 (local-only: `../../../docs/plan/25-folder-hierarchy-and-file-function-policy.md`),
27 (local-only: `../../../docs/plan/27-post-quantum-cryptography-and-migration.md`), and
29 (local-only: `../../../docs/plan/29-security-implementation-and-acceptance.md`) before changes.

## Exact API pins

- Engine: [CometBFT v0.40.0](https://github.com/cometbft/cometbft/tree/0880b4d378f347ab16e54ec677ff50d803f37d62), commit `0880b4d378f347ab16e54ec677ff50d803f37d62`, Apache-2.0.
- Upstream minimum compiler: Go 1.25.0; local source build uses patched Go 1.27.1. Its pinned ML-DSA implementation is Cloudflare CIRCL 1.6.3. This does not certify an entire EVE security profile.
- Rust wire generation: `prost`, `prost-build`, and `prost-types` 0.14.4; portable compiler source package `protoc-bin-vendored` 3.2.0. Exact resolved artifacts belong in Cargo.lock.
- The engine still uses the `tendermint.abci` protobuf package and `proto/tendermint/` paths. Vendored declarations retain their canonical upstream paths and exact `vendor/SHA256SUMS` bytes.
- `build.rs` delegates to `src/generation/generate_upstream_bindings.rs`; the generator checks vendored source digests and writes only to untracked Cargo `OUT_DIR`.

The exact release commit retains a stale upstream `TMCoreSemVer = "0.39.0"`
constant. The source build preserves that code and sets the upstream-supported
`TMGitCommitHash` linker metadata to the exact commit. Its checked CLI output is
`0.39.0+0880b4d378f347ab16e54ec677ff50d803f37d62`; source and executable digests
establish the v0.40.0 release identity. Do not infer engine parity from that stale
fallback version or rewrite upstream code to conceal it.

Vendored source is unchanged protocol declarations, not first-party execution
behavior. CometBFT is Apache-2.0 with `vendor/cometbft/LICENSE` preserved; Cosmos
gogoproto v1.7.2 at `cf5213e4dcbf1fea203185c0af00840e566790d9` is BSD-3-Clause
with `vendor/gogoproto/LICENSE` preserved. No first-party behavior belongs under
those source directories. Pins identify bytes, not finality/execution or audit.
Source updates require byte comparison, compatibility checks and reviewed pins.

The legacy Rust `tendermint-proto` 0.40.4 package's latest generated protocol
module is v0.38, so its version number does not establish v0.40 engine parity.
Its older key `oneof` does not expose ML-DSA. This component generates exact pinned
upstream declarations instead of silently dropping an unknown native key field.

## Executable contracts

For positive execution height H, `map_finalize_height` returns these distinct
heights with checked overflow:

| Output from FinalizeBlock(H) | Engine metadata height |
|---|---|
| Post-state application hash | H+1 header `AppHash` |
| Updated validator set hash | H+1 header `NextValidatorsHash` |
| Updated voting set | H+2 header `ValidatorsHash` |
| Updated last-commit voting metadata | H+3 ABCI requests |

`match_next_header_commitment` rejects another chain, H/H+2 anchors, and altered
or incorrectly sized hashes. A successful comparison authenticates neither a
certificate nor an execution result. The caller must verify historical validator
sets, the certified header, trust freshness, and snapshot/replay provenance.

Socket framing uses the engine's varint-length-delimited protobuf format. The
caller declares the byte bound; malformed, overlong, overflowing, truncated, and
oversized inputs are rejected before allocating the announced payload size.
Canonical vote encoding also matches the pinned engine's published byte vectors,
including fixed-width heights/rounds, the Go zero-time timestamp, and chain domain.

## Native hybrid incompatibility and bounded extension decision

The unextended engine has one public-key alternative per validator and one normal
signature per proposal/vote. `PublicKey.sum` alternatives include Ed25519 and
ML-DSA-65; concatenating both fields decodes to the last alternative. Permitting
both algorithms in `pub_key_types` allows different voters to select different
algorithms. It does not require both signatures for the same voter/message.
`native_hybrid_boundary` compiles and exercises this exact oneof incompatibility.
`require_supported_authentication` fails closed for `ClassicalAndMldsa65`.

Decision: retain v0.40.0 only for the labelled `CLASSICAL_DEV` lifecycle baseline.
SEC1 must provide a reviewed engine extension or maintained compatible alternative
before activating hybrid consensus. The bounded extension work must cover:

1. One enrolled hybrid key identity and paired signatures, counted once; preserve engine classical canonical sign bytes and bind the PQ envelope to those exact logical bytes, profile, genesis, key epoch, and purpose.
2. Proposal/vote verification, commits, validator updates/history, evidence, signer persistence/fencing, P2P encoding, and light-client/source verifiers. Unsupported historical profiles fail closed under their applicable rules.
3. Cross-implementation vectors, missing/swapped/retired component negatives, key-compromise cases, crash/retry sign intents, and quorum/locking regressions before profile activation.

ABCI vote extensions or post-finality application checks are insufficient to
replace these paths. `PQ_PROFILE_VERIFIED`, majority continuity, and secure-profile
acceptance remain unmet by this component. The baseline safety assumption is less
than one-third Byzantine voting power with strictly greater than two-thirds
unique valid power required for a commit; native ML-DSA does not alter that model.

## Actual lifecycle fixture

`abci_lifecycle` launches the exact source-built engine against a Rust API-test
application, exercises Info/InitChain/CheckTx/PrepareProposal/ProcessProposal/
FinalizeBlock/Commit, sends one fixed API transaction, inspects actual header
H/H+1 bindings and a power update at H+2/H+3, then restarts the engine and test
application from its synced test checkpoint. The fixture hashes are explicitly
`EVE_B0_API_FIXTURE_NOT_EVM`; this is an API compatibility test, not production EVM
execution, four-validator B3 acceptance, process/power-loss certification, or PQ
consensus. RPC sample headers are not independently authenticated by this test.

Run the component gate with `COMETBFT_BINARY` pointing to the pinned source-built
0.40.0 executable, `COMETBFT_SHA256` containing its checked lowercase SHA256, and
`EVE_B0_LOCAL_ARTIFACT_DIR` pointing beneath ignored root `local-tests/consensus-b0`:

```text
cargo test -p eve-consensus-comet --locked
cargo clippy -p eve-consensus-comet --all-targets --locked -- -D warnings
```

The lifecycle test fails if its binary/digest/artifact contract is absent; it never
silently skips actual-engine coverage. The upstream v0.40.0 GitHub release has no
binary assets, so build the exact source commit with the pinned Go toolchain.
Generated test keys, engine data, checkpoints, raw logs, and local evidence remain
inside ignored `local-tests/`; only sanitized reproduction summaries are publishable.

## B3 native classical signing and certificate operations

`consensus/signing/` owns pure native vote/proposal canonical conversion.
`encode_vote_sign_bytes` and `encode_proposal_sign_bytes` return the engine's
length-delimited protobuf bytes, with `sfixed64` heights/rounds, complete part-set
IDs and the native proposal POLRound rule. Missing nonnullable timestamps map to
Go's year-one zero time, distinct from Unix epoch; seconds/nanoseconds are bounded.
Chain IDs, message types, positive heights, nonnegative rounds, hash/address widths
and optional classical signature widths are checked before encoding. Nil votes
remain distinct from nonnil proposals. Baseline vote extensions fail explicitly.

Encoding grants no signing or execution approval. Proposer signing follows native
PrepareProposal ordering and precedes ProcessProposal; an application must not
require a prior ProcessProposal callback to sign its own prepared proposal.
Nonnil votes require actual execution/data approval for the exact block ID,
height, parent and applicable configuration/profile. Locked prevote/precommit
paths may reuse a previously validated block without a fresh callback, so runtime
approval and retained proposal data must survive/reconcile those rounds and crash
boundaries. These pure operations provide no approval ledger or durable signer.

`consensus/certificates/` owns native header/set hashes and classical certificate
verification. `canonicalize_validator_set` reproduces decreasing-power then
increasing-address order; `validator_address` is SHA256-20 of the raw Ed25519 key,
not its owner's EVM reward address. `hash_validator_set` hashes native
SimpleValidator protobuf leaves. `hash_consensus_header` hashes the native ordered
14-field RFC6962 tree using standard protobuf wrappers, not the execution RLP hash.
The header wrapper requires native block protocol 11 and bounded valid fields.
`hash_transaction_data` matches native Data.Hash: transaction SHA256 IDs become
the field-tree leaves. It bounds the development raw bytes/count before hashing;
complete BFT-envelope validity and actual EVM execution remain caller obligations.

`verify_commit_certificate` checks caller-expected chain/height/round/full block ID,
the header hash, applicable-height set hash, exact signer positions/addresses and
all nonabsent signatures, including nil votes. Only block votes count toward strict
`3*S > 2*T`; exactly two-thirds fails. Total power retains the native `i64::MAX/8`
limit. The 64-validator cap is an explicit local-development verification bound,
not production scalability acceptance. Duplicate/unknown/reordered signers and
malformed absent entries fail rather than reducing quorum.

The historical-set input carries the authentication requirement and rejects
activated hybrid use through the existing native guard. Its provenance and
freshness must already be authenticated by the caller. A result authenticates
that certificate only; it does not prove execution, establish validator-set history
or replace H/H+1 application anchoring. The certificate and height-binding checks
are tested separately. Genesis's development application anchor is the existing
height-zero StateVersion content digest; it is not an ApplicationCommitment(0).

Native Comet verification uses ZIP215 cofactored/noncanonical-point criteria.
Pinned `ed25519-zebra` 4.2.0 provides deterministic individual verification for this
boundary. Dalek's ordinary/strict equation is not substituted for native verification;
existing Dalek signing and protected-account/hybrid rules remain separate.
Native byte verification is distinct from EVE enrollment policy: successful raw
ZIP215 verification does not admit a weak key into EVE genesis or an update.
Native set hashing remains byte-based, matching Go even for a weak or invalid-point
key; actual signature verification decodes through Zebra. EVE enrollment must
separately enforce its approved key/possession policy.

Versioned public-only fixtures under `tests/fixtures/native-classical/` come from
the pinned Go engine's production canonical/hash/verification APIs, not the Rust
functions under test. `tests/fixtures/zip215-upstream/` preserves attributed
upstream edge inputs/notices with actual native outcomes. A mixed-order nonweak
key plus noncanonical R demonstrates the ZIP215/Dalek difference. See each fixture
README for provenance, identities and policy differences. Disposable fixture keys
are unsafe test identities and provide no live-network or custody authorization.

Run the dedicated `native_signing` and `native_certificates` suites, strict lint,
ownership and structure checks, then the integrated B3 gate when complete.
Four-validator operation, durable signing/fencing/recovery, partition schedules,
actual application lifecycle and complete T-C01–T-C10 acceptance remain separate
runtime/integration work. These scoped native operations do not establish B3
completion, majority continuity, PQ protection or a throughput result.
