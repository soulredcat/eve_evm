<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Bulk verification

`cargo xtask verify --bulk B0` runs EVE foundations including SEC0. D40 defers
external SEC2/INT0–INT3 programs until testnet, outside mandatory core selection.
A runnable command is not a passed gate or completed runtime/security target.
`cargo xtask verify --bulk B1` retains those foundations and adds complete-state,
durable recovery, development composition and independent acceptance coverage.

The complete gate uses Linux x86_64, Rust 1.97.1, clang-19/libclang-19, GCC, make,
Perl, Git and HTTPS download tools. CI pins its Rust/container/checkout identities
in `config/ci-environment.toml`. Reference executables, source archives and client
dependencies are provisioned into ignored task storage from exact pins:

```sh
cargo xtask provision-tools --jobs 2
cargo xtask verify --bulk B0
cargo xtask verify --security SEC0
cargo xtask verify --bulk B2
```

Provisioning validates archive/member/link containment, source digests, executable
versions, local receipts and the private client lock. It builds the pinned engine
and patched OpenSSL, then compiles the Solidity Shanghai and TypeScript/client API
probes. It does not install a global runtime, enable custody or launch a role node.
The gate revalidates `local-tests/toolchain-b0/provisioned-tools.json` and passes its
checked environment directly to child commands.

The gate runs structure and Redcat ownership inspection, formatting, strict workspace Clippy, exact
test discovery/execution (including registered documentation tests) and release
builds. Missing, duplicate, unregistered, ignored, filtered or zero requested
tests fail. The exact catalog is under `config/gates/groups/`; changing a required
case needs review. `check-structure` also inspects its own implementation.

```sh
cargo xtask check-structure --report local-tests/structure.json
cargo xtask check-ownership --report local-tests/ownership.json
cargo test --locked -p xtask
cargo xtask verify --bulk B1
cargo xtask verify --all
```

The `--all` command returns nonzero while later mandatory gates remain
unimplemented. `--all` cannot certify the project by selecting only completed
subsets. Core T-M/T-P/T-N and R requirements retain their individual
outcomes and owning dependencies; a primitive or metadata test does not close a
complete runtime requirement.

B2 prepares the exact complete Shanghai state corpus and a fresh ignored private
client package, installs its lock with scripts disabled, and runs strict TypeScript
with noEmitOnError before actual public/master process flows and all Solidity inputs.
It binds checked tool/runtime identities and retains B0/B1 regressions. Retired
external groups remain historical evidence, not current PASS or core dependencies.

Each attempt creates a unique ignored `local-tests/verify-*/` directory containing
a report, structure scan and command stdout/stderr. Reports bind the revision,
complete current source/config digests, dirty state, tool/compiler identities,
fixture/profile limitations, timing, selected/executed counts and artifact hashes.
Source mutation during the gate fails and requires a rerun. These local-only raw
artifacts are unavailable from GitHub and must never be staged or uploaded.

Compact reviewed results and reproduction instructions belong in
`docs/execution/`. CI keeps raw generated development keys/logs/databases local to
its disposable job and does not upload them. A local gate does not imply that a
GitHub workflow, independent audit, power-loss test or mainnet run has occurred.
