# Validator-owned CometBFT boundary

Canonical owner: `validator/components/consensus-comet/`. This component owns the
pinned engine's ABCI wire binding, bounded socket framing, height-mapping checks,
and explicit authentication capability boundary. It does not implement finality,
EVM transitions, private master storage, or a second consensus algorithm.
Dependencies flow from role runtimes/verifiers into this component; this component
has no private master dependency.

Read plans [12](../../../docs/plan/12-consensus-spec.md),
[14](../../../docs/plan/14-block-and-state-commitment-spec.md),
[25](../../../docs/plan/25-folder-hierarchy-and-file-function-policy.md),
[27](../../../docs/plan/27-post-quantum-cryptography-and-migration.md), and
[29](../../../docs/plan/29-security-implementation-and-acceptance.md) before changes.

## Exact API pins

- Engine: [CometBFT v0.40.0](https://github.com/cometbft/cometbft/tree/0880b4d378f347ab16e54ec677ff50d803f37d62), commit `0880b4d378f347ab16e54ec677ff50d803f37d62`, Apache-2.0.
- Upstream minimum compiler: Go 1.25.0; local source build uses patched Go 1.27.1. Its pinned ML-DSA implementation is Cloudflare CIRCL 1.6.3. This does not certify an entire EVE security profile.
- Rust wire generation: `prost`, `prost-build`, and `prost-types` 0.14.4; portable compiler source package `protoc-bin-vendored` 3.2.0. Exact resolved artifacts belong in Cargo.lock.
- The engine still uses the `tendermint.abci` protobuf package and `proto/tendermint/` paths. The [vendor notice](vendor/NOTICE.md) records exact source identities and SHA256 bytes.
- `build.rs` delegates to `src/generation/generate_upstream_bindings.rs`; the generator checks vendored source digests and writes only to untracked Cargo `OUT_DIR`.

The exact release commit retains a stale upstream `TMCoreSemVer = "0.39.0"`
constant. The source build preserves that code and sets the upstream-supported
`TMGitCommitHash` linker metadata to the exact commit. Its checked CLI output is
`0.39.0+0880b4d378f347ab16e54ec677ff50d803f37d62`; source and executable digests
establish the v0.40.0 release identity. Do not infer engine parity from that stale
fallback version or rewrite upstream code to conceal it.

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
