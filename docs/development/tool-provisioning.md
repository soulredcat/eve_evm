# Pinned task-local B0 tools

`cargo xtask provision-tools` provisions the complete Linux x86_64 B0 reference
tool/API fixture beneath ignored `local-tests/`. It performs no global install,
package-manager upgrade, service restart, wallet operation or live deployment.
Exact sources, checksums, executable identities and licenses are declared in
`config/tool-pins.toml`; unsupported hosts and missing prerequisites fail.

Host prerequisites are Rust/Cargo 1.97.1, CA certificates, curl, GNU tar, GCC,
clang-19, Perl and GNU Make. CI pins its build container separately. Host build
tools/libc affect generated binary identities; source-build receipts do not claim
identical OpenSSL bytes across different paths/toolchains or a certified module.

```sh
cargo xtask provision-tools --root . --pins config/tool-pins.toml \
  --output local-tests/toolchain-b0 --jobs 2
```

Direct API: `provision_b0_tools(root, pins, output, jobs)` returns `ProvisionedTools`
and persists `output/provisioned-tools.json`. `validate_provisioned_tools(root,
pins, receipt)` verifies the report before a bulk gate consumes its environment.
Never source/evaluate the JSON as shell code; CI selects explicit checked keys.

## Sources and recipes

| Tool | Exact input / action | Evidence boundary |
|---|---|---|
| Go 1.27.1 | [Primary download manifest](https://go.dev/dl/?mode=json); Linux archive/executable SHA256 pinned | GOTOOLCHAIN=local prevents automatic compiler replacement |
| CometBFT v0.40.0 | Exact source commit `0880b4d378f347ab16e54ec677ff50d803f37d62`; Go readonly locked modules, trimpath, CGO=1, clang-19 and pinned commit linker metadata | Preserves stale upstream `0.39.0` constant; expected CLI is `0.39.0+0880...`; no improvised consensus/hybrid wrapper |
| OpenSSL 3.5.7 | Verified [security-patched source](https://github.com/openssl/openssl/releases/tag/openssl-3.5.7); explicit Configure linux-x86_64, GCC, no-shared/no-tests/no-docs, bounded make jobs | Default provider, no FIPS-module claim; EVE's required reference tests still run separately |
| Solidity 0.8.37 | Verified [Linux compiler](https://github.com/argotorg/solidity/releases/tag/v0.8.37); standard-JSON Shanghai ABI/bytecode probe | Compiler API smoke, no bridge deployment/finality proof |
| Node 24.21.0 LTS | [Primary release hashes](https://nodejs.org/dist/v24.21.0/SHASUMS256.txt), Linux archive and executable SHA256 pinned | No global Node/npm change; bundled npm 11.19.0 |
| TypeScript 6.0.3 / viem 2.57.1 | Exact private manifest/lock, task-local npm ci with install scripts disabled | Strict typed real viem ABI/client API probe and deterministic ERC-20 encoded bytes; no RPC request or transaction |
| Solana kit 8.4.0 | [Primary npm package metadata](https://registry.npmjs.org/@solana/kit/8.4.0), exact version/publisher SRI pinned | Declared source input for INT0; actual Solana SDK/endpoints remain INT1 work |

TypeScript 6.0.3 is selected for the dependency-free JavaScript compiler recipe.
The inspected TypeScript 7.0.2 package adds platform-native compiler dependencies;
changing that recipe requires an explicit compiler/source/lock update and smoke.
This choice is a development tool pin, not a claim that 7.x is unsupported by EVE.

No downloaded README/install script supplies executable instructions. Reviewed
operations invoke curl/tar/compiler binaries through argument vectors; no shell
recipe is assembled from source/config data. Go module downloads use the public
proxy/checksum database and the pinned engine's unmodified go.mod/go.sum.

The npm fixture is `xtask/tests/provisioning_client_fixture/package.json` and
generated `package-lock.json`: npm 11.19.0, lockfileVersion 3. Lock SHA256 is
`a7037971456231c2e08383627f51d66fe8ecb73d77cce79738ef88e7dd8ee400`; manifest SHA256
`c8a9a4300f6d5bdeda15e1a169ae7738d7e68245cc1116b1d4606176e55a0672`.
The lock retains exact dependency versions, publisher integrity and license data.
TypeScript is Apache-2.0; viem and Solana kit are MIT. Node/Go/OpenSSL/Comet compiler
and dependency notices remain separate from an EVE source-license decision.

## Containment, reuse and reports

Every output path must be an ordinary descendant of canonical root `local-tests/`.
Existing symbolic-link prefixes and absolute/parent escapes are rejected. Archives
are SHA256-checked before listing/extraction; member roots, traversal, links and
special devices are checked. The pinned Comet archive contains one ordinary
workflow filename ending in a space; that valid name is retained and regression
tested. It does not weaken absolute/parent/foreign-root/link rejection.

Per-tool receipts bind source SHA256, executable path/digest/version and reviewed
recipe. Prebuilt Go/Node/Solidity binaries also match declared binary hashes.
Full reports bind pinfile SHA256, five artifacts, npm lock and sorted module-tree
digest, plus exact environment paths. Reuse first validates cached archive/binary
bytes, source/pin/recipe identities, module bytes and environment consistency, then
reruns compiler/client probes. A changed or incomplete existing file fails; it is
not silently overwritten or treated as a fresh trusted tool.

`provision.lock` prevents concurrent provisioning of one output directory. A stale
lock/incomplete directory or download requires explicit inspection/removal of
that task-owned path before retry. There is no recursive cleanup of user data.
Raw child stdout/stderr is captured in named ignored `logs/*.log`; console output
contains only compact operation/status/local-log references and final JSON.

Receipt environment keys: `COMETBFT_BINARY`, `COMETBFT_SHA256`, `EVE_OPENSSL`,
`EVE_GO_BINARY`, `EVE_SOLC_BINARY`, `EVE_NODE_BINARY`, `EVE_TSC_SCRIPT` and
`EVE_B0_LOCAL_ARTIFACT_DIR`. The last path is a task-created descendant of
`local-tests/consensus-b0/`, matching actual lifecycle-fixture containment.
These are development test inputs, not credentials or finality authority.

The generated TypeScript/Solidity probes live only in ignored task output. Tracked
Rust generators/checks and fixture metadata remain subject to the structure gate.
Unit negatives cover artifact path escapes, tar traversal/link/device handling,
changed receipt/binary/source/recipe/path and changed client module bytes/paths.
The Linux suite additionally tests real symlink-prefix rejection.

B0 tools/API provisioning is not SEC1 hybrid consensus, authenticated external
source verification, custody execution, security acceptance or live-route approval.
Actual consensus, maintained-reference and full gate tests consume the validated
receipt and remain separate evidence with real selected test counts.
