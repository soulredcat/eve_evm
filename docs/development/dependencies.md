<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Foundation dependency decisions

Exact Rust registry versions, package sources and checksums are in `Cargo.lock`.
Runtime/API selections are constrained in root `Cargo.toml`; tools, client lock,
corpora and CI image/action identities have separate versioned pin files under
`config/`. No existing locked Rust package was upgraded for this B0 integration.
New entries provide the Comet protobuf/API, metadata codecs and role contracts;
retired PQClean wrappers and their unused helper are removed.

The selected production/API dependencies were inspected through actual Cargo
metadata and compilation, rather than guessed library methods:

| Boundary | Exact primary packages | Declared license metadata |
|---|---|---|
| EVM | REVM 43.0.3 | MIT |
| Ethereum types/roots | alloy-primitives 1.7.3, alloy-consensus/alloy-eips 2.5.0, alloy-rlp 0.3.16, alloy-trie 0.9.8 | MIT OR Apache-2.0 |
| Durable records | rocksdb 0.25.0 / librocksdb-sys 0.19.0+11.8.1 | Apache-2.0 wrapper; sys metadata MIT/Apache-2.0/BSD-3-Clause |
| Authentication | ml-dsa 0.1.1, k256 0.13.4, ed25519-dalek 2.2.0, zeroize 1.8.2 | Apache-2.0 OR MIT for RustCrypto/zeroize; BSD-3-Clause for Ed25519 |
| ABCI generation | prost/prost-build/prost-types 0.14.4, protoc-bin-vendored 3.2.0 | Apache-2.0 prost; MIT wrapper; upstream compiler/source notices retained |
| Wire/config/gates | serde 1.0.229, serde_json 1.0.151, toml 0.9.8, syn 2.0.117, sha2 0.11.0 | MIT OR Apache-2.0 |
| Encoding, including transitive codecs | bs58 0.5.1, hex 0.4.3 | MIT/Apache-2.0 metadata |

Primitive and cross-implementation tests do not establish an audit or secure
consensus profile. RustCrypto ML-DSA remains explicitly unaudited here. The native
Comet key oneof cannot enforce paired classical AND PQ authorization; its startup
guard refuses activated hybrid configuration. SEC1 must resolve that real engine
integration gap. See the [crypto inventory](../security/inventory/crypto-sources.md).

The maintained ML-DSA reference is source-built patched OpenSSL 3.5.7, with exact
source/recipe and per-build executable digest. Its default provider is used;
there is no FIPS-module certification claim. Comet's exact v0.40.0 source retains
an upstream 0.39.0 version string, so the source revision/linker suffix identifies
the executable honestly. Solidity, Node and Go prebuilt artifacts use exact
official digests; npm installs the private client fixture with scripts disabled.

The 2026-10-01 Cargo audit at RustSec database revision
`9b3a3b73a7f42606494c943e95f8196e9994df46` reports zero known vulnerabilities and
two unresolved unmaintained-package warnings:

- derivative 2.2.0 — RUSTSEC-2024-0388, retained in the resolved lock graph; the
  current default-target reverse dependency query has no active edge.
- paste 1.0.15 — RUSTSEC-2024-0436, a procedural macro reached through
  alloy-primitives. Its source/build maintenance risk remains visible.

Neither warning is suppressed or renamed into a passing security review.
Upstream-compatible replacement/review is a retained dependency-maintenance task;
these results do not approve production cryptography. Raw audit, metadata and tool
build output stay in ignored `local-tests/`.

Vendored Comet/gogoproto licenses and NIST notices remain adjacent to pinned
source/fixtures. Tool source archives retain upstream notices locally. The
metadata above is a bounded dependency inventory, not legal clearance, a license
grant for first-party code, or a complete distribution notice bundle. Standalone
packaging/release review must reproduce the full relevant notices before release.

## B2 networking, client and current audit

B2 adds jsonrpsee 0.26.1, Tokio 1.53.1 and their HTTP/WS/networking helpers.
The actual Cargo metadata contains 464 identities; no retained registry identity
or checksum changed. All 56 newly introduced upstream Rust identities have
permissive MIT/Apache or CC0/MIT-0 alternatives. Two new first-party packages
carry the owner's Redcat permission-only terms; no dependency is relicensed.

The independent client pins EthereumJS common/MPT/RLP/util 10.1.3 and Node type
definitions. MPT/RLP/util declare MPL-2.0; they are test utilities rather than a
runtime redistribution grant. Some upstream crate/npm archives omit root license
texts. Exact upstream VCS notices were reviewed locally; a complete copied-role
distribution notice bundle remains mandatory in B6/release work.

The actual B2 Cargo audit 0.22.2 exits 0: 464 identities, zero known vulnerabilities
and the same two unsuppressed derivative/paste maintenance warnings. RustSec was
freshly fetched on 2026-10-01 at the revision above. The installed 32-package B2
client npm audit exits 0 with zero vulnerabilities. These are bounded known-advisory
and metadata/notice checks, not legal clearance or network/PQ certification.
See [the B2 record](../execution/B2-20261001.md) for commands and local-only hashes.
