# Bulk verification

`cargo xtask verify --bulk B0` runs the registered foundation, including SEC0 and
INT0 subsets. A runnable gate is not a passed gate or a completed runtime.
Security/bridge/interop/runtime/capacity acceptance remains separate from B0.
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
cargo xtask verify --interop INT0
```

Provisioning validates archive/member/link containment, source digests, executable
versions, local receipts and the private client lock. It builds the pinned engine
and patched OpenSSL, then compiles the Solidity Shanghai and TypeScript/client API
probes. It does not install a global runtime, enable custody or launch a role node.
The gate revalidates `local-tests/toolchain-b0/provisioned-tools.json` and passes its
checked environment directly to child commands.

The gate runs structure inspection, formatting, strict workspace Clippy, exact
test discovery/execution (including registered documentation tests) and release
builds. Missing, duplicate, unregistered, ignored, filtered or zero requested
tests fail. The exact catalog is under `config/gates/groups/`; changing a required
case needs review. `check-structure` also inspects its own implementation.

```sh
cargo xtask check-structure --report local-tests/structure.json
cargo test --locked -p xtask
cargo xtask verify --bulk B1
cargo xtask verify --all
```

The `--all` command returns nonzero while later mandatory gates remain
unimplemented. `--all` cannot certify the project by selecting only completed
subsets. Registered T-M/T-P/T-BR/T-I/T-N and R requirements retain their individual
outcomes and owning dependencies; a primitive or metadata test does not close a
complete runtime requirement.

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
