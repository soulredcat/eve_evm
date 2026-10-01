<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# SEC0 security inventory

This inventory contains authorization, commitment, crypto-source and adversary obligations registered at B0.
It is not an independent audit, activated protected profile or `SECURITY_PROFILE_ACCEPTED`.
Primitive vectors and signature wrappers do not close missing consensus, account,
recovery, release, transport or client integration. This main tree retains accepted B2;
validator signing/consensus runtime is not implemented here. Incomplete B3 remains
separately on draft PR #1 with an unresolved C06 recovery failure; it is not merged.

## Core scope, authority and fault model

D40 retains SEC0/SEC1/SEC3 and T-M01–T-M10/T-P01–T-P10 as mandatory EVE core work.
SEC2, INT0–INT3, T-BR and T-I are `DEFERRED_UNTIL_EVE_TESTNET` for subsequently
authorized separate programs. Deferral is neither completion nor a failed core
gate. Testnet does not automatically authorize an adapter prototype or custody.
Later programs consume EVE's versioned headers, receipts, finality and proofs.
External clients, RPC, confirmation speed, routes and custody cannot govern EVE
execution, block production, transaction voting, finality or durable acknowledgement.
EVM Shanghai standards, compatibility libraries and reference corpora remain core
correctness material; they do not introduce a remote-network dependency.

Unique weighted power S forms a certificate only when `3*S > 2*T`. Safety assumes Byzantine
power below one third; liveness also needs honest participation and eventual delivery.
One-third withholding can halt. A 51% coalition alone lacks strict two-thirds power,
but malicious overlap can enable conflicting quorums. Correct continuous finality at 51%
remains `UNSATISFIED_BY_BASELINE`: no quorum lowering, master finality or schedule-based immunity.
Paired signatures cannot make an authorized quorum honest; public count grants no voting power.
Master stays outside mandatory voting/per-transaction authority. Preserve 40/30/30 fees
and secured 1M aggregate finalized TPS; neither secured capacity nor a stronger threshold is proved here.

## Authorization boundaries and required coverage

Canonical paired authorization belongs to `validator/components/authentication/`.
Its caller-supplied identity is not trusted enrollment, validator finality or
production signing. Classical development paths and `HYBRID_EXPERIMENTAL` laboratory
coverage remain separate from any future verified profile. Activated hybrid
authorization requires both signatures for the same enrolled identity and message
throughout the actual engine and clients; an ignored post-finality stamp is insufficient.

| Boundary | Required authorization, binding and unresolved obligation | Gates |
| --- | --- | --- |
| Proposal, prevote, precommit | Engine-native classical AND ML-DSA over the same canonical sign bytes, identity, chain/genesis, height/round/step, profile and epoch; B3 classical work does not supply activated hybrid consensus. | T-P02/T-P03 |
| Commits/certificates | Authenticate the applicable historical weighted set; count each identity once after required components, enforce strict quorum, execution validation and necessary data. | T-M01/T-P02 |
| Validator enrollment/rotation/retirement | Possession of both enrolled keys where activated, bonded power, authenticated previous history, epoch/profile and delayed set transitions; no downgrade. | T-P04/T-M07 |
| Durable signer/fencing | Sync full signing intent/sign bytes and actual returned signature before release; bind retries and restart identity, preserve safety history, fence uncertainty and corrupt/stale state. One local lock does not prove general coherent-backup rollback detection or cross-host clone fencing. | T-M06/T-P06 |
| Legacy EOA/ecrecover | secp256k1 authorization remains classical; preserve Ethereum transaction and opcode semantics. | T-P03/T-P10 |
| Protected account deployment/execution | Explicit ABI, gas, nonce and expiry; paired owner authorization binds every operation field. | T-P03/T-P05 |
| Approve/permit/session/delegate | Bind allowance, spender, expiry, revocation and contract authority; no unprotected delegated-owner bypass. | T-P05 |
| Account admin/upgrade | Enforce the active profile on implementation selection, storage migration and owner changes. | T-P05/T-P09 |
| Guardian/account recovery | Enrolled guarded authority, recovery delay, allowance/pending-operation migration and no classical-only bypass. Production recovery/genesis authority requires owner decisions. | T-P05/T-P06/T-P09 |
| Staking/delegation/withdrawal | Protected owner authority, nonce, delayed activation and durable funds/ownership rules. | T-P04/T-P05 |
| Slashing/evidence/unbonding | Authenticate historical identities, both components where required, retention/evidence windows and trust consistency; reject arbitrary accusations and guessed operator identity. | T-M06/T-M07/T-P04 |
| Public/bootstrap/master followers | Bind historical profile, set and anchor; verified replay or authenticated commitments establish trust. Master has no finality key. | T-M05/T-P07 |
| Stored checkpoints/snapshots | Authenticate history/set/profile, replay tail and expiration; checksums do not authenticate sources and old classical anchors do not become retroactively PQ. | T-M08/T-P04/T-P07 |
| Master admin/endpoint access | Separate credentials, least authority and authenticated transport; cannot vote, approve a route or rewrite history. | T-M05/T-P09 |
| Conflicting EVE finality | Quarantine affected authentication/readiness and preserve evidence; master cannot select replacement history or clear the conflict. | T-M09 |
| Releases/upgrades | Proposed two-of-three development identities are not an implemented control: both signatures per counted identity, manifest/artifact binding and anti-downgrade are required, separate from consensus/custody. | T-P09 |
| Transport authentication | Maintained protocol, authenticated peers, downgrade protection and explicit residual classical-certificate assumptions; full protected integration remains required. | T-P09 |
| Transport key exchange | ML-KEM is key agreement, not signatures; maintained authenticated hybrid transport must assess confidentiality separately. | T-P09 |

## Commitments, identities and recovery trust

| Material | Format and required security distinction |
| --- | --- |
| ML-DSA-65 pure external message | FIPS 204 `0x00`, context length, context and complete message with SHAKE internals; bind domain/context and never silently switch to prehash or precomputed-mu interfaces. |
| Paired message | Versioned encoder binds genesis/network, purpose, identity/key epoch and payload; trusted enrollment, canonical engine-byte relationship and activation remain separate requirements. |
| Ed25519 votes/key IDs | Freeze actual selected-engine sign bytes and complete enrolled keys/epoch; do not truncate paired identity into an unsafe key selector. Internal SHA-512 does not remove classical key assumptions. |
| Ethereum signing/accounts | EIP-155 and typed envelopes use Keccak-256; the last 20 public-key-hash bytes form a 160-bit routing identity. Routing, collision and targeted substitution differ from signature strength; stronger outer signatures do not imply a 128-bit generic quantum preimage margin. |
| Consensus block/header IDs | Canonical engine hashing must bind the applicable historical set, parent and application commitment; assess hash properties separately from signatures. |
| EVM trie/code/address commitments | Keccak-256 MPT state/storage/receipt/transaction roots and code/CREATE/CREATE2 derivations preserve deterministic EVM semantics. Signatures authenticate bytes, not correct execution or available preimages; supplemental formats require versioned migration. |
| EVE execution/application roots | Versioned execution header/system-root binding authenticates execution H through native header H+1. An arbitrary root plus H certificate is insufficient. The existing genesis state content digest is only the explicit development H0 anchor, not an invented AppCommitment(0). |
| Checksums/WAL/durable cursors | Corruption detection and successful durability are not source authentication or power-loss proof. Distinguish applied, durable and authenticated watermarks. |
| Snapshots/deltas/chunks | Versioned chain/profile-bound anchors and replay tails; reject stale/foreign roots and retain the only recoverable finalized copy. Fresh nodes cannot treat a newly supplied classical anchor as PQ trust. |
| Release/package/source digests | SHA-256 pins identify artifact bytes, not audit, execution validity, trustworthy genesis, licensing permission or consensus authority. |

No whole-chain security category follows from ML-DSA-65; inner 256-bit commitments and 160-bit namespaces
need use-specific review. A wider digest cannot repair lost inner collision resistance. Authenticate
key/profile migration before classical authority is assumed broken. Independent crypto, key-generation/backup
and side-channel review plus full T-P coverage remain open.

## Exact crypto sources, maintained reference and audit limits

| Pinned source/reference | Exact identity and provenance |
| --- | --- |
| RustCrypto `ml-dsa 0.1.1`, `zeroize` | Commit `f75d5b829948988f18d9463f286805fb9410bcdd`; package SHA-256 `add6b9d92e496f16f4526d68ff29da1483aba4b119baeab8bed3b9e3544a6f3d`; Apache-2.0 OR MIT. |
| NIST ACVP external/pure ML-DSA-65 | Commit `a7f283cdc87d2d6dd93c1bac59e5622c5f9f8324`; all 15 verification cases, exact source digests and complete NIST notice; public inputs contain no secret keys. |
| [FIPS 204](https://csrc.nist.gov/pubs/fips/204/final), checked 2026-09-30 | 2026-07-31 [potential-update spreadsheet](https://csrc.nist.gov/files/pubs/fips/204/final/docs/fips-204-potential-updates.xlsx), SHA-256 `5bc93ce63bc647e6d1d456cb2d3a171426c15aca4a7a0e0edd40d08b7a34c793`. |
| OpenSSL 3.5.7 maintained reference | Commit `8cf17aaeb4599f8af87fefd810b5b5fee90fe69e`; [archive](https://github.com/openssl/openssl/releases/download/openssl-3.5.7/openssl-3.5.7.tar.gz) SHA-256 `a8c0d28a529ca480f9f36cf5792e2cd21984552a3c8e4aa11a24aa31aeac98e8`, matched publisher digest before execution; Apache-2.0/upstream notices. |

RustCrypto upstream reports no independent audit: `EXPERIMENTAL_UNAUDITED`, without production approval
or FIPS validation. ML-DSA-65 keys/signatures are 1,952/3,309 bytes, context at most 255 bytes;
no parameter, prehash or precomputed-mu downgrade. Paired Ed25519 verifies both components on
one message, not enrollment. NTT/message/internal-bound potential corrections are not a new standard.
The pinned signing loop uses a 16-bit counter with parameter-dependent steps, not the outdated
814-attempt bound; signing-failure handling, side channels and full implementation review remain open.

OpenSSL's default provider is not a certified FIPS module. Reviewed [pkeyutl](https://docs.openssl.org/3.5/man1/openssl-pkeyutl/)
and [ML-DSA API](https://docs.openssl.org/3.5/man7/EVP_SIGNATURE-ML-DSA/) preserve pure mode/context;
[RFC 9881](https://www.rfc-editor.org/rfc/rfc9881.html) SPKI uses OID `2.16.840.1.101.3.4.3.18`, absent parameters/full raw key.
Required tests cover both implementation directions, wrong message/context, mutated/truncated signatures,
exact SPKI/key/signature sizes and all 15 NIST cases. Wrong/missing tool fails: no skipped fallback or runtime subprocess.
Historical patched 2026-09-30 run: exit 0, three tests, none skipped, 4.15s. A system-3.5.6 negative run
failed exact-version checking; earlier results are superseded by the [3.5.7 security patch](https://github.com/openssl/openssl/releases/tag/openssl-3.5.7).
Original recipe: `perl Configure linux-x86_64 no-shared no-tests no-docs`, `make -j2`;
GCC Debian 14.2.0-19, Perl 5.40.1, Make 4.4.1, Linux x86_64/WSL, Rust/Cargo 1.97.1 Windows GNU runner.
The OpenSSL upstream suite was not run (`no-tests`); mandatory EVE reference tests are separate.

| Artifact: original isolated spike unless stated | SHA-256 |
| --- | --- |
| `apps/openssl` | `a560fee7791ebf7dd1d139ba1a13ec37bfa869cae6b557da34edd045c4136acf` |
| `libcrypto.a` | `b208d373a9202b6fcc5755a82f74bdbf66ee0b4ff8389f96b1433dd082ee8db9` |
| `libssl.a` | `eca342df4db65a8618922c0c6bfefebd542ea1a5ece558a0bf5802a99f10c20a` |
| Runtime `libc.so.6` | `9792e3cbb541c8f44c7acf5f14f4022ea62998ecc787d326bed4d8b6547dfd92` |
| Runtime `ld-linux-x86-64.so.2` | `c8438e4fde1934e61c88311633f00949ff645d5c04cdb8671fa3d78164d2f307` |
| Fresh provisioned `apps/openssl` | `fee3bb2754da06f2855fbbda174a1ab42e4f02c72dbd9e9033b30b404a42868f` |

Receipts bind source/recipe/version/actual bytes; binary identities are environment-specific.
OpenSSL libraries were static, platform libraries dynamic; no system package was upgraded.
Ignored spike output: `local-tests/security-interop-b0/`; fresh reference: `local-tests/toolchain-b0/openssl/`.
Reproduction sets `EVE_OPENSSL` to the pinned tool (`EVE_OPENSSL_WSL=1` for Windows/WSL) and runs
`cargo test --locked -p eve-crypto --test maintained_cross_implementation -- --test-threads=1`; no new run is claimed.
Retired dev-only `pqcrypto-mldsa 0.1.2`, `pqcrypto-traits 0.3.5`, `pqcrypto-internals 0.2.11` and two tests
were removed after unmaintained advisories [0166](https://rustsec.org/advisories/RUSTSEC-2026-0166.html),
[0162](https://rustsec.org/advisories/RUSTSEC-2026-0162.html), [0163](https://rustsec.org/advisories/RUSTSEC-2026-0163.html).
Dated local `local-tests/b0-dependency-audit.json`: zero known vulnerabilities, residual unmaintained
derivative 2.2.0 / RUSTSEC-2024-0388 and paste 1.0.15 / RUSTSEC-2024-0436; not a current audit/security proof.
SLH-DSA release/recovery alternatives, authenticated hybrid transport and complete engine/account/client/control
integration remain unimplemented or unaccepted. No algorithm/library certifies EVE.

## Registered core gate outcomes

These are obligations, not passing attack simulations. Missing, empty, stubbed or
skipped mandatory implementation/test coverage fails discovery. Withholding,
equivocation, censorship, partitions, adaptive corruption, stale-set/long-range
history, eclipse, cloned signers, missing data and finality conflicts need real
engine/runtime evidence; above-baseline observations are schedule-specific.

| Gate | Required outcome |
| --- | --- |
| T-M01 | Checked weighted boundaries; exact two-thirds and duplicates rejected; below-one-third, one-third and 51% modeled distinctly. |
| T-M02 | Real engine schedules preserve supported-threshold honest agreement and recover eventual progress. |
| T-M03 | One-third-or-more withholding stalls without quorum reduction or master takeover. |
| T-M04 | Reject attacker-only 51% certificates; conflicting-quorum schedules expose violated assumptions without immunity claims. |
| T-M05 | Compromised master/RPC cannot change authenticated state, set or checkpoint. |
| T-M06 | Clone/restart/stale-backup/corrupt-signing-state failures are safe; stronger unmet fencing requirements stay visible. |
| T-M07 | Public count supplies zero voting power; real set/stake/delegation evidence is auditable. |
| T-M08 | Stale/long-range anchors, eclipse and delayed evidence restrict recovery/client trust, not create trust. |
| T-M09 | Authenticated conflicting finality quarantines affected authentication/readiness and preserves evidence; master cannot select history or clear conflict. |
| T-M10 | Honest full replay rejects invalid execution metadata; light-client limits are stated separately. |
| T-P01 | Standard vectors, maintained two-way interoperability and malformed encoding/context rejection. |
| T-P02 | Required hybrid rejects classical-only/PQ-only, mismatched keys/messages, wrong domains and duplicate identities. |
| T-P03 | Actual vote/commit/client/protected-account paths require enrolled PQ authorization after simulated classical compromise. |
| T-P04 | Enrollment/activation/rotation/retirement, stale anchors and mixed versions preserve history without downgrade. |
| T-P05 | Protected admin/recovery/permit/session/staking/control rejects classical fallback and enforces nonce/replay/expiry. |
| T-P06 | Crash/retry/backup/restore, malformed/oversized signatures and exhaustion preserve signing safety and bounded queues. |
| T-P07 | Actual RPC/execution/finality/master/sync enforces profile and authenticated H–H+1 binding. |
| T-P08 | Active-profile byte/CPU/storage benchmarks preserve serial/parallel semantics. |
| T-P09 | Release/upgrade/recovery/transport inventory exposes classical bypasses and fails required missing coverage. |
| T-P10 | Commitment/address/proof analysis states residual assumptions independently of signature strength. |

## Deferred separate-program inventory

Historical interop/disabled bridge source was removed from core under D40;
revision `038fe80f412754e5a7080240ca0e57ba3c19e368` remains recoverable.
`eve-route-binding-v1` was metadata without approval signatures, not a core trust registry.
Future IDs/manifests bind chain/genesis/route/deployment, asset/amount/recipient/sequence/profile,
verifier/anchor/upgrade policy. Retries create no new entitlement; deposits/burns need protected EVE authority.
Clients need actual finality/inclusion/historical identities and atomic replay/backing checks;
custody/mappings bind exact origin/deployment/conserved backing. Pause-only authority cannot emergency-mint/unlock;
resume requires fresh-anchor reconciliation. External wallets/consensus/admin stay route-specific and classical:
Ethereum fork-specific SSZ/SHA-256/BLS plus execution Keccak evidence, or Solana's actual ledger/certificate,
epoch rank/stake/key changes and custody execution; never a fabricated Ethereum receipt trie for Solana.

| Deferred gate | Required future program outcome |
| --- | --- |
| T-BR01 | Two real local EVE networks transfer fake assets through real consensus, inclusion and custody. |
| T-BR02 | Wrong chain/genesis/route/custody/recipient/amount/profile/proof cannot move value. |
| T-BR03 | Replay/duplicates/restart preserve exactly-once atomic effects. |
| T-BR04 | Revert/lookalike/invalid inclusion/H–H+1 misbinding rejected. |
| T-BR05 | Halt/stale trust/malicious source/missing data cannot bypass proof or emergency-mint. |
| T-BR06 | Exposure caps, bounded delay queues, authenticated pause and authorized reconciliation withstand faults. |
| T-BR07 | Deposits/burns/wrapped liabilities and exact decimals conserve backing; unsupported tokens fail. |
| T-BR08 | Reentrancy/admin/upgrade takeover and retired keys rejected under the active profile. |
| T-BR09 | Actual destination checks both required signatures; external classical assumptions stay visible. |
| T-BR10 | Observed conflicting finality stops the affected route; delayed-detection exposure remains explicit. |
| T-BR11 | No timeout-only refund races delayed mint; unsupported refunds remain disabled. |
| T-BR12 | Real proof/PQ gas, bytes, latency and invalid-input costs are bounded and measured. |

These extend core T-P05/T-M09 without replacing EVE coverage: user authority
T-BR09/T-I09; source updates/claims T-BR01–05/T-I05; custody/mappings
T-BR07–08/T-I06; pause/recovery T-BR06/10. Caps/watchers cannot repair source
consensus or guarantee detection before damage. External BLS/Ed25519/secp256k1
remain quantum-vulnerable even if EVE uses paired authentication. No production
custody, real funds, irreversible genesis/recovery authority or license grant is
authorized by this inventory; later owner scope and route-specific acceptance remain required.
