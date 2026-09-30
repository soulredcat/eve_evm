# Resumption handoff

## Current position

Current checkout: branch `codex/implementation-foundation`, checkpoint based on HEAD `5dc49bd24d833188719a9b9fb94bb4981bf00f71`. The owner explicitly authorized publishing this foundation checkpoint to GitHub. B0/SEC0 remain in progress and unaccepted; consult the current Git revision and recorded verification rather than treating publication as completion.

No complete role runtime/devnet has been started. A task-owned isolated RocksDB interface spike completed successfully (process-crash and snapshot/checkpoint checks, not power-loss certification). Rust 1.97.1 was verified on Windows GNU and installed for Linux/WSL; required clang-19/libclang-19 and Git packages were installed without upgrading existing Debian packages. Do not treat this environment setup or isolated spike as a full implementation gate.

The owner forbids root `crates/`/`create/` and requires copy-ready independent public/validator role directories. Source was moved intact to `public/components/recovery-store/`, `validator/components/execution/` and `validator/components/authentication/`; root Cargo members and fixture paths were updated. Required NIST fixture data now belongs to the authentication component. Master-host composition keeps separate role authority. Standalone runtime distributions and copy/build/run tests remain `NOT_IMPLEMENTED` / `NOT_RUN`.

The foundation checkpoint contains pinned Cargo manifests/lockfile, crypto/EVM/storage source, initial xtask checking code and absolute role-placement rules. After relocation and publication cleanup, 29 authentication, 8 EVM and 12 Linux storage integration tests pass; selected strict lint/format/structure checks pass. Native Windows release builds of xtask/authentication/execution and the Linux recovery-store release build pass. Full checker boundary tests, complete runtimes, CI and B0 SEC0/INT0 gates remain unfinished. Prior implementation agents are interrupted/errored; do not assume background work continues.

Dependency audit has zero known vulnerabilities and five unmaintained warnings: derivative 2.2.0, paste 1.0.15, pqcrypto-internals 0.2.11, pqcrypto-mldsa 0.1.2 and pqcrypto-traits 0.3.5. PQClean wrappers are dev-only cross-implementation tests, not an approved production authentication backend. Keep the warnings visible and choose a maintained reviewed cross-verification path before full SEC0 acceptance.

The owner requires clean GitHub publication and English shared prose. Read `CONTRIBUTING.md` and the corresponding rules in `AGENTS.md`. Root `local-tests/` is ignored machine-local storage and must never be committed or pushed; recreate it locally when absent in a fresh checkout. Required reproducible tests and sanitized fixtures remain tracked. The latest explicit owner request authorizes this checkpoint push; it does not authorize a mainnet deployment or imply completed bulk acceptance.

Read plan 32 and decisions D29–D33: default public working state is RAM-first, with durable finalized recovery blocks/checkpoints written by a separate bounded worker; applied/durable/authenticated heights are distinct. Keep master-independent live P2P/finality and synchronous validator sign safety. Zone IDs are routing metadata; public prefers a nearby eligible logical sync endpoint with fallback sources. Masters evolve from one development follower to verified two-master replication and a planned ten-master layout; none of these plans grants production deployment authority.

T-N09/T-N10 belong to B4, T-N11 to B6, and T-N12 to B8. Their registration/documentation is not implementation or a passing runtime test. B0 and SEC0 are `IN_PROGRESS`; downstream bulk acceptance remains unachieved. Preserve the 1M mixed-workload/security gate and deferred liquidity scope.

## Next work

Resume B0 from the existing dirty role-owned foundation; do not recreate root `crates/` or discard the source:

1. Inspect branch, HEAD, dirty files, installed tools and sandbox permissions. Preserve unrelated edits.
2. Read root AGENTS.md/goal.md/CONTRIBUTING.md and plans 12–32, especially accepted decisions, bulk dependencies, security/interoperability, public persistence and current scope.
3. Complete checker boundary/negative/package tests and gate discovery/CI, then compile/test authentication and recovery storage in the actual workspace. Current xtask/EVM compilation is not runtime acceptance.
4. Freeze executable byte vectors, native ABI/gas schedule and consensus commitment-height mapping.
5. Freeze measured versioned storage/source-selection/readiness budgets and typed watermark/recovery contracts; register T-N09–T-N12 without claiming their later distributed runtime coverage passed.
6. Create workspace and xtask verification/devnet/evidence contracts, run B0 gates and record actual evidence.
7. Continue B1 and subsequent ready bulks.

Initial inspection commands, where available:

```sh
git status --short
git log -1 --oneline
git check-ignore -v -- local-tests/README.md
git ls-files -- local-tests/ artifacts/ coverage/
rustc --version
cargo --version
```

Cargo.toml, Cargo.lock and initial xtask now exist. The workspace source lives under public/validator components with one canonical owner per reusable implementation. Copy-ready public/validator distributions still require self-contained manifests/dependency copies and isolated build/run tests; do not claim raw directory independence from monorepo metadata alone.

The exact next command is `git status --short --branch` using the verified checkout. Complete the structural boundary tests and CI, remaining component/interface acceptance, exact ABCI lifecycle, SEC0/INT0 inventories/fixtures and plan 32 contracts before closing B0. Current structure scan is verified, but mandatory T-L/full-bulk acceptance and standalone role build/run remain unachieved. Current verification commands/results belong in `EVIDENCE.md`; raw reports stay under ignored `local-tests/`. Publication is an explicitly authorized checkpoint, not a release or completed goal.

## Decisions not to reverse

Validators decide finality; master follows and persists. No automatic master takeover. Separate runtime roots and independently buildable public packages. Durable validator signing/recovery records despite RAM working state. Authenticated snapshot roots with correct consensus height binding. Fee split 40/30/30. Keep the 1M aggregate finalized TPS target measured and unachieved until verified.

## Future handoff updates

Replace this next-action section with actual progress at each interruption: branch/worktree and HEAD; dirty-file summary; current bulk/tasks; passing/failing evidence; task-owned live processes; exact next command; blockers and resource requirements. Never claim automatic background continuation or hide a failing gate behind a completed checkbox.
