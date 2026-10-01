<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Resumption handoff

Updated: 2026-10-01. The owner authorizes integrating each verified bulk into
main and pushing. Inspect branch, HEAD, index and dirty files before work.
No background continuation or unobserved hosted result is implied.
Latest owner steering supersedes the earlier finish-B3-first boundary: pause
development now because the PC runs other programs. Publish the current-state
checkpoint branch and create a draft PR to main without merging, then stop.
Do not resume development, tests/builds, B4 or another bulk without new input.
For the following publication after this checkpoint and new owner resumption,
`.md` files are forbidden except README.md. The owner explicitly deferred this
migration for the current PR. Preserve content, migrate formats/references/gates
and implement filename enforcement before that later publication; not done now.

Current primary branch: codex/validator-consensus-b3, base/main/origin B2 commit
b5f98df359eadfc83983b22eea39a249bd03fdbb. B2 was pushed non-force and both
checkouts were clean before B3. The current checkpoint is explicitly incomplete
and stays on its draft branch; main remains at the accepted B2 commit. Its current
commit/PR identity is verified by Git and the attached PR, avoiding self-reference.
Hosted B2 run 36812752566
at that exact commit completed SUCCESS; actual job log confirms 348 executed cases.
Read CI-20261001-B2.md. This is separately observed hosted B2, not B3 acceptance.

The B2 integration contains verified serial RPC, Redcat notices and D40 changes,
tested at base 038fe80 with an unchanged before/after source bundle. The commit
containing this record is identified by Git, avoiding a self-referential hash.
Verify main/origin publication and hosted B2 independently; neither is inferred.
Read [STATUS](STATUS.md), [B2 evidence](B2-20261001.md), AGENTS, goal,
plans 13/18/20/23/24/25/29/32 and the affected directory READMEs.

## Current scope and ownership

D40 defers SEC2, INT0–INT3, bridge and external-chain programs until EVE testnet
and a subsequent separate-program instruction. Future programs adapt to EVE's
public APIs. External RPC, confirmation speed, routes and custody never govern
core execution, voting, finality or durable acknowledgement. Plans 28/30 and
config/deferred-programs retain future safety contracts and historical provenance.
The unused interop/bridge packages are removed; their baseline is recoverable
at 038fe80 and their task-owned noticed copies remain ignored locally.

Current core contains B0–B11 plus SEC0/SEC1/SEC3: 15 gates, with mandatory core
T-M/T-P/persistence coverage retained. Fourteen packages have explicit ownership.
Validator owns canonical protocol, state, execution, authentication and Comet API
contracts. Public owns admission/RPC, verified-view policy and recovery repository.
Master owns private composition/storage entry points. Public and validator do
not depend on private master. Standalone copied distributions remain B6 work.

The owner requested Redcat permission-only notices. Commentable first-party
files carry SPDX/permission comments; exact REUSE associations cover JSON,
generated locks and preserved upstream material without altering canonical bytes.
LICENSE and LICENSES retain statutory/platform/third-party rights. This is a
copyright policy, not cryptographic file signing or proof of legal ownership.
Every implemented bulk runs check-ownership alongside structure; never waive either.

## Verified position and B2 scope

Historical B0 including then-authorized SEC0/INT0 passed 205 cases. B1 passed
271 cases, followed by the repaired 273-case gate. Hosted runs at b0b9efa and
038fe80 separately passed 271 and 273; see the checkpoint evidence record.

B2's complete local gate exits 0 with 348 exact Cargo cases, zero ignored/failed/
filtered, format, strict lint, release, structure and ownership. Tooling has 133
cases; nested 18 client flows and 3495 corpus variants are not added again.
See the integrated record for exact source/config/tool/output hashes and commands.

The canonical block builder handles headers/basefee, signed admission, isolated
simulation and inactive f100 dispatch. Per-transaction EIP-161 cleanup repairs
touched empty accounts without changing upstream goldens. Shared trie operations
provide bounded local membership proofs, never validator certificates.

Public has real HTTP/WS RPC and nonce-aware bounded admission. The development
producer applies canonical execution and synced storage before publishing local
receipts/events. Complete history indices preserve canonical B1 bytes, bootstrap
in bounded whole-block prefixes, bind completion to database identity and fence
ambiguous writes. Actual interrupted children and signed-history corruption
tests are distinct from simulated faults and hardware power-loss certification.

The independently compiled Node/TypeScript/viem client executes 18 enforced
flows, 12 Solidity sources, two independent public stores/processes and one
master RPC composition. Strict compilation emits no unverified client.
The pinned unchanged Shanghai subset covers all 194 state files/3495 variants;
EVE compares 3118 protected executions with its declared economic effects.
84 Ethereum-valid unprotected envelopes remain explicit EVE rejections; all
88 unprotected inputs are tested. Reference gas is 120M; live development is 30M.

## Exact next commands

First: git status --short --branch. Verify B2 publication, then read plan 12 and
the consensus component README. Next is B3: actual four-validator execution,
certificates and signing durability. B3 implementation and its complete gate now
exist; implementation alone is not a passing bulk. The complete gate remains FAIL
until the actual recovery defect below is repaired and every required check passes.
Read local-tests/b3-preparation role handoffs: native sign/cert/hash/ZIP vectors,
56-case storage and 45-case ownership conjunction checks are scoped passes.
They do not prove a four-validator runtime or full T-C01–T-C10 acceptance.
Private signer/approval/application/Unix transport are being integrated under
validator/src/consensus. The actual classical runtime and all ten process cases
now have scoped passing evidence; complete frozen B3 verification remains pending.
The current B3 manifest selects 92 feature-enabled validator cases and 16 independent
network/codec cases, retaining every current core B2 regression. Normal and test
validator executables are separately preserved and fingerprinted by the preparer.
The first complete B3 attempt fails at the network group: 259 prior cases passed,
then 15 of 16 network cases passed and T-C06 stopped when validator 1 exited before
height 9 after all-node crashes. Local-only report:
local-tests/verify-254-1790848988209122086/report.json, source bundle
9c9560cf74a1bd0e0ae27efe7a82bf374ebdd6ea13cd60b6448708702704e3e3.
This is distinct from the earlier common-checkpoint selection repair. Generic
stderr lost the underlying Rust actor error; bounded private diagnostics and
failure-only native namespace retention are being added for concrete reproduction.
Source is now frozen as an owner-paused checkpoint. B3 is not complete and its
classified failure has not been reproduced. Three private diagnostic cases pass;
a fourth scoped regression preserves the original signing refusal instead of
masking it as missing data, with no signature/cursor/policy change. The current
CLI f7e1d80e contains diagnostic version one, not the final cause-preservation
source patch. The complete gate has not been rerun at this newer source.
After new owner authorization, inspect git status first. Exact next development
command: cargo build --locked -p eve-validator --features development-acceptance.
Then record its SHA and coordinate retained C06 reproduction before another
complete frozen gate. Do not execute these commands or resume automatically now.

All publishable source must stay frozen during the full gate's before/after
fingerprint. Fix genuine failures, rerun affected checks and the full gate.
Record commands/counts/source/config/tool/output identities in reviewed evidence,
review the index/outgoing history, commit coherently, fast-forward main and push
without force. Keep primary/task-owned checkouts clean; preserve unrelated edits.
Observe the resulting hosted gate separately. B3 preparation found native FilePV
does not sync the parent directory after rename. Use a reviewed durable signer
contract, explicit non-nil vote execution/data approval, correct engine remote
priv-validator wire support and replay metadata; never treat process-restart tests
as hardware power-loss or general backup-rollback protection.

## Environment and continuity

Linux reference uses Rust 1.97.1, clang/libclang 19 and the recorded native pins.
Local WSL Cargo is under /home/admin/.cache/eve-evm-toolchain/cargo/bin;
CARGO_HOME and RUSTUP_HOME use that task toolchain, while CARGO_TARGET_DIR is
/home/admin/.cache/eve-evm-build/target. Never alter unrelated tools/services.
Task tools use ignored local-tests/toolchain-b0, recipe 2; pin/receipt/archive/
binary/client-tree identities are revalidated. No globally installed Python
or automatic service restart is needed. Raw logs/keys/databases stay ignored.

Comet RPC height can advance before application Commit. Lifecycle recovery
waits for both RPC and synced application marker; preserve the checkpoint/restart
assertions. Native hybrid activation remains rejected until actual SEC1 support.
Every child development process must be stopped/waited by its creating fixture.
Never restart unrelated trading bots.

## Remaining production boundaries

DEV_ALL_IN_ONE and LOCAL_DEV_UNAUTHENTICATED are explicit development only.
Local receipts, durability and root matching never establish validator finality.
Four-validator consensus/anti-double-sign durability is B3; authenticated recovery
and isolated public RAM persistence are B4. Staking/native work is B5, copied roles
are B6, parallel equivalence is B7, HA/releases are B8 and regional testnet is B9.
Whole-state copies/synced B2 persistence are bounded correctness baselines,
not the secured 1M architecture or a zero-storage-overhead promise.

Validators retain sole finality; sign only after execution/data and durable
anti-double-sign state. Zones/nearest masters confer no authority. Preserve
40/30/30 and 1M aggregate finalized TPS. Baseline assumes less than one-third
Byzantine weighted power and strictly more than two-thirds unique valid power;
51_PERCENT_CONTINUITY remains UNSATISFIED_BY_BASELINE.
No mainnet, real funds/custody, paid resources, license grant, visibility change
or irreversible genesis decision is authorized. Publish English reviewed source
and compact evidence; never stage local artifacts, dependencies, keys or raw logs.
