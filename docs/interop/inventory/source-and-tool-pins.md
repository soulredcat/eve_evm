<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Historical external source and tool pins

Inspected on 2026-09-30 for B0/INT0, with evidence preserved at source revision
`038fe80f412754e5a7080240ca0e57ba3c19e368`. Under D40, external consensus/Agave
pins and INT work are `DEFERRED_UNTIL_EVE_TESTNET`, not active core provisioning
requirements. Solidity, EVM primitives, Shanghai execution vectors and pinned
developer clients remain core correctness inputs. All pins below are historical;
a later separate program must review its actual sources/tools before use.

These reference/tool pins identify inputs; they are not
approval of a deployed network, trust anchor, custody model or source license for
EVE. All downloaded raw archives/binaries remain ignored task-local artifacts.
No remote installer script was executed. Exact Cargo dependencies and checksums
remain in the integrator-owned manifest/lockfile.

| Input | Pin / primary source | License / execution status |
|---|---|---|
| Ethereum consensus specs | [v1.6.1](https://github.com/ethereum/consensus-specs/tree/5fa6edcca8ab4cf548653e6680b17b9d3e04d225), commit `5fa6edcca8ab4cf548653e6680b17b9d3e04d225` | CC0-1.0; source inspected, complete fixture suite NOT_RUN |
| Solidity compiler | [0.8.37](https://github.com/argotorg/solidity/releases/tag/v0.8.37), commit `f401782df49be312ea4ef52a2d467cf5183b5906` | GPL-3.0; Linux binary digest/version verified and Shanghai ABI compile smoke passed; endpoint deployment NOT_IMPLEMENTED |
| Agave client/program test source | [v4.3.0](https://github.com/anza-xyz/agave/tree/825efd18292aff6ffcf9daa0f7612f21b3531a72), commit `825efd18292aff6ffcf9daa0f7612f21b3531a72` | Apache-2.0; source inspected; validator/program tool install and integration NOT_RUN |
| Agave Rust toolchain | Pinned source `rust-toolchain.toml` selects 1.97.1 | Same compiler revision as EVE workspace; Agave source build NOT_RUN |
| EVM integer/address primitives | `alloy-primitives = 1.7.3` | MIT OR Apache-2.0; actual interop compilation/tests passed |
| Solana base58 display | `bs58 = 0.5.1` | MIT OR Apache-2.0; actual codec API compilation/tests passed |
| EVM raw hex display | `hex = 0.4.3` | MIT OR Apache-2.0; actual codec tests passed |

Verified downloaded source archives:

| Archive / download identity | SHA-256 |
|---|---|
| [Ethereum specs codeload at exact commit](https://codeload.github.com/ethereum/consensus-specs/tar.gz/5fa6edcca8ab4cf548653e6680b17b9d3e04d225) | `337faec80801162ab917337b8c1158a8ab33eda90d03c2d7fbadd6a2001b2b36` |
| [Agave codeload at exact commit](https://codeload.github.com/anza-xyz/agave/tar.gz/825efd18292aff6ffcf9daa0f7612f21b3531a72) | `f1f1d7ccf6fc2946a42488d63a503c4a45862015fc326bbb055e0ae7c6c187d8` |
| [Solidity Linux compiler](https://github.com/argotorg/solidity/releases/download/v0.8.37/solc-static-linux) | `5de843c2c93563cc66425c99a4fb13fdbf32b4c4ae07469480faaf126e14404a` |

Publisher release artifact pins, not yet downloaded/executed:

| Artifact | Publisher SHA-256 / pending status |
|---|---|
| [Ethereum specs general fixtures](https://github.com/ethereum/consensus-specs/releases/download/v1.6.1/general.tar.gz) | `87898ba7a3fa16bc66b08c88b4ccab008833a517a7602cf421c6ce79734e07e9`; NOT_RUN |
| [Ethereum specs minimal fixtures](https://github.com/ethereum/consensus-specs/releases/download/v1.6.1/minimal.tar.gz) | `e1d0c9dc3d09fb74a98b324c40b2587a17dc555e6149d3f9741601fffcf43901`; NOT_RUN |
| [Agave Linux development distribution](https://github.com/anza-xyz/agave/releases/download/v4.3.0/solana-release-x86_64-unknown-linux-gnu.tar.bz2) | `c97289a8abb1d0efb497d8b5cb285baabd9b7f8ea6647f5d145c5dc8ff3611e8`; NOT_INSTALLED |

Agave's pinned manifests identify API families: solana-program 4.1.0,
solana-pubkey 4.3.0, solana-message 4.5.0, solana-transaction 4.2.0,
solana-packet 4.3.0, spl-token-interface 3.0.0 and
spl-token-2022-interface 3.1.0. These are inspected upstream inputs, not new EVE
dependencies. A subsequently authorized INT1 must pin actual program/SDK lockfiles and SBF
compiler tools before integration; no claim is made that these crates already
compile together as an EVE adapter.

Inspect Agave's actual feature activation and dependency audit before a runtime
decision. Its stable release notes contain ignored advisory entries; source
availability is not security acceptance. Solidity endpoint builds must explicitly
target EVE Shanghai; external Ethereum header/fork verification follows the actual
source fork rather than EVE's execution target. Compiler version output was
`0.8.37+commit.f401782d.Linux.g++` with exit 0. The task-local standard-JSON API
probe targeting Shanghai compiled 2 ABI entries and 433 bytecode bytes with no
compiler error. It is an ABI/compiler smoke, not a deployed bridge or verifier.

The maintained OpenSSL source/tool pin and exact experimental authentication
limitations are in [crypto-sources](../../security/inventory/crypto-sources.md).
