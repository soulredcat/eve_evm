# Resumption handoff

Updated: 2026-10-01. B0 including SEC0/INT0 and B1 passed their complete local
gates. The owner authorizes integrating each verified bulk into main and pushing.
Inspect actual branch, HEAD, index and status; no background continuation is implied.

## Verified position

Read [STATUS](STATUS.md), [B1 evidence](B1-20261001.md), prior B0/CI records,
AGENTS/goal and mandatory plans. Latest local B1 gate executes 271 cases with
zero failed/ignored/filtered, strict lint, format, structure and release build.
Its input/source/config/tool/output identities are recorded in the reviewed B1
record and ignored raw report. Post-run evidence/status gets final checks before
the coherent B1 commit; verify actual publication rather than assume a branch.

Twelve packages have explicit ownership. Validator owns immutable protocol,
logical state, one canonical EVM/system root implementation and execution/auth/
bridge/consensus contracts. Public owns complete recovery repository, cache/
snapshot/policy and interop metadata. Master owns the explicit development-only
composition; tools/tests own gates and independent acceptance. No private master
dependency or root crates/create dumping directory is allowed.

The master CLI uses DEV_ALL_IN_ONE with explicit acknowledgement and dedicated
ignored data. It performs real genesis/init/reopen/EVM apply/snapshot/restore,
rejects production/master-sync-only/private-signing material and advertises no
authenticated validator finality. Genesis debits bonds into f100 exactly once.
Full roles, four-validator signing/finality, public worker/network recovery,
standalone copied distributions, security/custody and TPS remain unfinished.

## Actual environment and CI

Pinned task-local tools are provisioned under local-tests/toolchain-b0, recipe 2
with readonly Go source modules, no enclosing EVE VCS stamp and exact upstream
revision identity. Historical recipe 1 tools were preserved in a separate ignored
namespace. Do not silently reuse old recipe receipts. Source/archive/binary/
client-tree hashes and checked child environment are revalidated by every gate.

Linux reference uses Rust 1.97.1, clang 19/libclang 19, GCC 14.2, make 4.4.1, Perl 5.40.1
and the explicit recorded native/reference pins. Local WSL Cargo is task-owned
and may not be on PATH; consult prior commands in ignored reports, never alter
unrelated toolchains/services. No complete-role service is running at checkpoint.

Hosted CI attempts are separate: extraction locale, checksum-manifest LF,
enclosing Git provenance and checkout UID trust were repaired in scoped commits.
The latest observed hosted failure reaches consensus after crypto/bridge, while
the local actual-engine fixture passes. HTTP chunk framing was repaired and
fixed safe diagnostic categories added; inspect the current hosted run before
claiming it passes. Raw logs/keys/databases are not uploaded or published.
Never restart unrelated trading bots.

## Exact next step — B2

First command: `git status --short --branch`. Then read plans 13/18/20/23 and
the public/validator/state/execution/recovery READMEs. B2 is real serial EVM,
fee/RPC/developer flow, not another in-memory scaffold:

1. Add a thin public JSON-RPC runtime with bounded admission, request/response
   types, validation and cache-first immutable state service. Raw DB remains in
   the canonical repository; deterministic execution performs no network calls.
2. Decode/recover actual signed Shanghai envelopes, execute through the canonical
   complete-state serial oracle and expose real receipts/logs/code/storage/
   balances/block hashes. Keep native/EVE fee differences explicit.
3. Integrate local development production without granting master production
   finality. Four-validator consensus/sign durability remains B3, not a fake
   certificate or trusted-master fallback.
4. Add actual Solidity deploy/native/ERC20/swap/revert developer fixtures and
   applicable pinned upstream compatibility vectors. Wallet/client/tool versions,
   resource/nonce/fee bounds and error behaviors must be measured/declared.
5. Extend B2 gates with nonzero exact coverage, retain B0/B1 regressions and
   structure. Run full gate, record evidence, commit and integrate only on pass.

B1 whole-state copies/recovery encoding are a bounded correctness baseline;
they are not the secured 1M storage/execution architecture. Clone reservations
include both oracle images; a partial lazy cache is never a complete state root.
Stored/root-matched data and local checksum snapshots are not finality proofs.
Use the existing retained-history/content digest, canonical codecs and explicit
durable acknowledgement; preserve exact replay and atomic fee/system effects.

## Non-negotiable limits

Validators own finality; no quorum lowering or master takeover. Sign only after
execution/data and durable anti-double-sign state. Public RAM state retains
durable recovery inputs with separate applied/durable/authenticated heights.
Zones/nearest endpoints grant no authority. Preserve fee 40/30/30 and secured 1M
aggregate finalized TPS target, which remains unverified.

Baseline assumes less than one-third Byzantine weighted power and strictly more
than two-thirds unique valid power. 51% continuity remains unsatisfied. Native
hybrid activation stays unsupported/rejected until real SEC1 enforcement.
Custody/routes remain disabled; no primitive/interface/unit result certifies the
network or external chain. Deferred liquidity/stabilization remains out of scope.

Publish English source/docs and compact reviewed evidence. Never stage ignored
local tests/tools/raw logs/private material/databases/dependency/build output.
Review index/outgoing history and preserve unrelated edits. Mainnet/genesis
economics, real keys/funds/custody, paid infrastructure, license and visibility
changes retain explicit owner boundaries.
