# Vendored protocol source provenance

These files are unchanged upstream protobuf declarations and license notices,
not handwritten EVE production behavior. `SHA256SUMS` records their exact bytes.
The build script checks those digests before invoking the pinned protobuf compiler.
Generated Rust output goes only to Cargo's untracked `OUT_DIR`.

| Source | Exact identity | License | Included files |
|---|---|---|---|
| [CometBFT](https://github.com/cometbft/cometbft/tree/0880b4d378f347ab16e54ec677ff50d803f37d62) | v0.40.0, commit `0880b4d378f347ab16e54ec677ff50d803f37d62` | Apache-2.0; preserved `cometbft/LICENSE` | ABCI types and their crypto/types/version imports; canonical vote/proposal types |
| [Cosmos gogoproto](https://github.com/cosmos/gogoproto/tree/cf5213e4dcbf1fea203185c0af00840e566790d9) | v1.7.2, commit `cf5213e4dcbf1fea203185c0af00840e566790d9` | BSD-3-Clause; preserved `gogoproto/LICENSE` | `gogoproto/gogo.proto`, required by CometBFT declarations |

The canonical source paths are preserved beneath each upstream directory. No
first-party behavior may be added beneath these directories. The source checksums
are not a finality or execution proof. A dependency/source update requires review,
new byte comparisons, compatibility tests, and an updated pin decision.
