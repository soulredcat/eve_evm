# 27 — Post-quantum authentication and migration

Status: mandatory security work, added 2026-09-30. The current classical devnet baseline is NOT post-quantum secure. No cryptographic integration or acceptance test is claimed complete.

## P01 — Scope and claims

Interpret the owner's “pos quantum” requirement as post-quantum cryptographic security, distinct from proof-of-stake consensus. Protect the actual authorization paths: validator proposals/votes, account operations, staking/control keys, checkpoints/light clients, bridge custody, upgrades/releases and recovery. Inventory transport authentication and key exchange separately.

An ML-DSA library, encrypted NVMe, TLS, more validators, a random pool ID or a post-hoc signature on a root does not by itself secure the chain. Every trust path and bypass must be checked. Post-quantum signatures also do not solve malicious-majority consensus behavior from plan 26.

## P02 — Standards and implementation policy

Use reviewed implementations of standardized algorithms, with pinned sources, parameter sets, artifacts and test vectors. Do not invent cryptography or claim FIPS module validation merely because an algorithm appears in a FIPS standard.

Research baseline: ML-DSA-65 for online signature integration experiments. Benchmark other standardized parameter sets only through an explicit recorded decision, never a silent speed-driven downgrade. SLH-DSA is a standardized alternative to evaluate for independent release/recovery designs; it is not automatically added to every transaction.

ML-KEM is for establishing shared secret keys, not for signing transactions or votes. A hybrid key-exchange transport can still use quantum-vulnerable authentication certificates. Audit authentication, confidentiality and application authorization independently.

Primary standards: [FIPS 204 / ML-DSA](https://csrc.nist.gov/pubs/fips/204/final), [FIPS 205 / SLH-DSA](https://csrc.nist.gov/pubs/fips/205/final), and [FIPS 203 / ML-KEM](https://csrc.nist.gov/pubs/fips/203/final). Check the current standard pages, errata and implementation status when pinning dependencies. See [NIST PQC migration overview](https://csrc.nist.gov/projects/post-quantum-cryptography).

## P03 — Explicit security profiles

| Profile | Meaning |
|---|---|
| CLASSICAL_DEV | Initial Ed25519 consensus and secp256k1 EVM compatibility tests. Never advertised as PQ-secure. |
| HYBRID_EXPERIMENTAL | Classical AND PQ authentication enforced for the paths under test; incomplete paths remain listed. Not a production security claim. |
| PQ_PROFILE_VERIFIED | All mandatory authorization, migration, commitment, recovery and performance tests pass for a named profile and published assumptions. Not a claim that every legacy account or external chain is protected. |

At genesis or an authenticated scheduled upgrade, bind the security profile, algorithm IDs, canonical encoding versions and activation heights. Local configuration cannot disable required PQ checks on an activated network. Unknown/unsupported profiles fail closed; no negotiation fallback may downgrade authentication.

## P04 — Hybrid signature binding

For an activated hybrid authorization, require BOTH the applicable classical signature and the PQ signature for the SAME enrolled identity and SAME logical message. Verification is AND, not OR. Count voting power once and only after all required signature components pass.

Use unambiguous canonical encodings and domain separation. Bind network/genesis, protocol/profile, purpose, signer identity/key epoch, and the complete consensus/account/bridge payload. Bind chain, height/round/step/block ID for votes; account, nonce, expiry and operation bytes for user authorization; route and unique message identity for bridges.

Do not change the engine's canonical classical sign bytes casually. Specify and test the exact relationship between the existing engine message and its PQ-bound counterpart. Registration must prove possession of all required keys and authenticate their binding. Reject swapped keys, duplicate signers, truncated signatures, invalid lengths, wrong domains and retired keys.

No threshold-PQ or signature-aggregation property is assumed. Verify an explicit set of individual signatures initially. A later aggregation scheme needs its own reviewed security model, dependency decision and end-to-end benchmarks.

## P05 — Consensus integration is not an ABCI flag

The current engine's Ed25519 baseline is classical. Determine support in the actual pinned engine. Enforcing an extra application check after finality, or attaching PQ signatures that the vote/commit/light-client verifiers ignore, does not upgrade consensus authentication.

Implement a reviewed consensus-engine extension or maintained compatible alternative that enforces the hybrid rule throughout proposal, vote, commit, validator-set transition, evidence, signer, network and light-client paths. Preserve tested locking and fault assumptions; this is not authorization to write a new improvised consensus algorithm.

The protected master and bridge verifier must enforce the same profile and historical key-set binding. Existing durable anti-double-sign records still apply. Record signing intents, message identity and emitted signature material safely across retries/crashes; randomized signatures must not change the signed logical identity or permit conflicting messages.

If the selected engine cannot support the requirement, record the concrete failing interface, alternatives and implementation work. Keep `PQ_PROFILE_VERIFIED` unmet; do not relabel classical consensus as secure.

## P06 — Accounts and EVM compatibility

Standard secp256k1 EOAs remain a classical compatibility mode, not quantum-resistant accounts. Preserve standard EVM opcodes and ecrecover semantics; do not silently reinterpret a legacy Ethereum transaction as a PQ signature.

First protected-account path: an explicitly versioned contract-account or native authorization extension that verifies the required PQ/hybrid user operation. Freeze the ABI, authorization encoding, gas schedule, account derivation and replay rules through executable fixtures. A [ERC-4337-style account-abstraction design](https://eips.ethereum.org/EIPS/eip-4337) is a possible transport pattern, not a claim that support already exists.

An untrusted bundler may relay an authenticated operation but cannot change recipient, value, calldata, fees/limits, nonce or expiry. Cover deploy/initialize, upgrade/admin, approve/permit, sessions, recovery, guardian, staking withdrawal, bridge deposit and key-rotation paths. No classical-only owner or recovery bypass may control a PQ-protected account.

Do not claim a legacy EOA becomes protected by a frontend label or by registering a second public key alone. Migration must secure funds, allowances, contract authority and pending operations. Prefer fresh protected accounts when migrating legacy authority; document residual legacy exposure. Show actual wallet/tooling limitations instead of promising unchanged signature UX.

## P07 — Hashes, roots and identity strength

Inventory signature prehashes, consensus IDs, validator-key identifiers, Ethereum Keccak/MPT roots, bridge message hashes, code hashes, address truncation and snapshot commitments. Specify the required collision/second-preimage/authentication properties and quantum assumptions for each path.

A stronger signature or a 512-bit outer hash does not automatically strengthen a weaker inner commitment. Do not claim a uniform 128-bit or NIST-category security level for the whole EVM merely from the ML-DSA parameter choice. Analyze legacy 256-bit commitments and 160-bit routing identities before an end-to-end claim. Where insufficient, design and test versioned supplemental commitments or migrations without pretending existing Ethereum proof formats changed transparently.

Pre-register trusted key material and anchors before retiring classical authorization. A fresh node must authenticate profile and key transitions from a valid genesis/checkpoint, not from a newly supplied classical signature after assumed classical compromise. Expired anchors invoke an explicit rebootstrap procedure. Old checkpoints are not retroactively made secure by appending a new signature.

## P08 — Release, recovery and implementation security

Release manifests and recovery/upgrade authorities must meet the active profile. Hybrid two-of-three release authorization requires both signature types for each counted identity; master, admin and release keys remain separate from consensus/custody keys. A software-update signature does not authorize changing consensus history.

Use secure RNG, secret-memory hygiene, constant-time reviewed implementations where applicable, bounded decoding and tested error handling. Review implementation-specific side channels, dependency provenance and backup/restore. No test-only verifier bypass may enter a production build.

Transport upgrades use maintained protocols and explicit downgrade protection; do not hand-roll a TLS/KEM handshake. Record any remaining classical transport trust root and its consequences. Offline/private networking alone is not a cryptographic migration.

## P09 — Performance and accounting

Measure real public-key/signature sizes, key registration cost, signature verification/signing latency, memory, block/consensus bytes and storage replication. Separate per-user-operation authentication from per-validator-vote and per-bridge-message costs.

Capacity model: `signature_bytes_per_second = authenticated_operations_per_second * measured_signature_bytes`, plus keys/envelopes, validator traffic, replication and receipts. Batching commitments does not eliminate original authorization bytes or verification work. Do not assume PQ signatures compress or aggregate like BLS.

Meter worst-case verification and invalid-signature spam. Add limits before expensive verification, bounded CPU queues and consensus-valid deterministic gas charging. Run the target TPS benchmark under the activated security profile; a classical-only benchmark cannot demonstrate secured-profile capacity.

## P10 — Acceptance

| Test | Required evidence |
|---|---|
| T-P01 | Pinned algorithm/parameter/implementation vectors, cross-implementation verification and malformed-input rejection; no claimed module certification without evidence. |
| T-P02 | Classical-only, PQ-only, mismatched-message/key, wrong-domain and duplicate-signer inputs fail where hybrid is required. |
| T-P03 | Simulate classical-key compromise in a task-owned test network: actual vote/commit/light-client and protected-account paths still require valid enrolled PQ authorization. No production signature-forging bypass. |
| T-P04 | Enrollment, activation, rotation, retirement, stale checkpoint and mixed-version boundaries preserve authenticated history and prohibit downgrade. |
| T-P05 | Protected-account admin, recovery, permit/session and bridge paths cannot fall back to classical-only authorization; nonce/replay/expiry tests pass. |
| T-P06 | Signer crash/retry, backup restore, malformed large signatures and resource-exhaustion tests preserve safety and bounded queues. |
| T-P07 | Full RPC-to-execution-to-finality-to-master/sync verification enforces the profile, including the H/H+1 commitment rules. |
| T-P08 | Real security-profile benchmarks report byte/CPU/storage costs and unchanged serial/parallel semantics. |
| T-P09 | Release, upgrade, recovery and transport inventory tests expose or eliminate every classical trust bypass; missing coverage fails the relevant claim. |
| T-P10 | Commitment/address/proof analysis states residual quantum assumptions; stronger signatures alone cannot satisfy the whole-chain claim. |

## P11 — Completion

Publish a component-by-component crypto inventory, test evidence, supported secure-account operations, remaining classical dependencies, named security profile and migration/recovery runbook. Set `PQ_PROFILE_VERIFIED` only for the proven scope; keep unsupported components and external routes visible. Independent security review and owner-authorized deployment remain required before real-value operation.
