# Resumption handoff

Updated: 2026-10-01. B0 including SEC0/INT0 passed the complete local foundation
gate. The owner authorized integrating each verified bulk into main and pushing
to GitHub. Inspect the actual branch/HEAD/index before starting; this handoff is
part of the coherent B0 publication commit, not a promise of background work.

## Verified foundation

Read [STATUS](STATUS.md), [B0 evidence](B0-20261001.md), root AGENTS/goal and the
required plans. The integrated gate ran on base HEAD
4e559fc8ed5dcdd77542ca6f0dd03eb1eeb3b9e2 with its recorded dirty source bundle:
205 cases pass, none ignored/failed/filtered; format, strict workspace Clippy,
structure and release build pass. The reviewed record contains exact source,
config, tool and raw-artifact identities. Raw reports/tools remain local-only.

Nine packages have explicit ownership. Public owns recovery-store, node-policy
and interop metadata. Validator owns execution, authentication, consensus-comet,
protocol-config and bridge-protocol. xtask owns development gates/provisioning.
Do not recreate root crates/create, relocate public policy under validator,
duplicate consensus logic or introduce private-master dependency edges.

Source-independent public/validator role distributions remain B6 work: existing
monorepo component dependencies do not satisfy copy-directory build/run. Full
role runtimes, four-validator finality/signing safety, actual public workers,
authenticated sync/recovery, secure profile, custody and TPS remain unfinished.

Fresh/reuse tool provisioning passed. Go/Comet/OpenSSL/Solidity/Node, the private
npm lock and real TypeScript/viem API probe are pinned. OpenSSL 3.5.7 replaces
retired PQClean test dependencies. Cargo audit has zero known vulnerabilities and
two retained derivative/paste warnings; do not suppress them. Native Comet hybrid
activation is unsupported and rejected. No FIPS-module/security audit claim.

No complete role devnet or task-owned service is running at this checkpoint.
Hosted CI execution is not inferred from the locally passing workflow lint or
gate; read actual GitHub run status when relevant. Do not restart unrelated bots.

## Exact next step — B1

First command: `git status --short --branch`. Then read plans 14/16/22/23 and the
component READMEs. Implement the atomic full-state/recovery vertical slice:

1. Freeze a validator-owned state view/journal domain under
   validator/components/state, reusing protocol record codecs and maintained
   trie primitives. Keep database handles and private master behavior out.
2. Extend public-owned recovery storage with actual EVM/system state, code,
   slots, blocks/receipts/roots and atomic durable metadata. One bounded synced
   WriteBatch initially; an ambiguous write/sync error requires reconciliation.
3. Adapt the validator execution reference to a bounded complete immutable view.
   Never hash a partial lazy cache as complete state or blindly use REVM
   CacheDB::nest().flatten(); untouched inner storage can be lost by replacement.
4. Add a thin master-owned DEV_ALL_IN_ONE harness with explicit development
   identity/task-owned data, no production voter/finality fallback, and loopback
   endpoints if a listener is needed.
5. Verify golden roots, missing/mismatched code, zero/delete/recreation, stale
   parents, exact replay, captured-view isolation, state/system/block/marker
   atomicity and staged-snapshot interruption. Real process termination is not
   hardware power-loss proof; label simulated failure schedules separately.
6. Extend `verify --bulk B1`, retain all foundation/structure regressions, run the
   complete gate, update evidence/status/handoff and integrate only after passing.

Current record storage persists opaque inputs/cursor, not complete authenticated
EVM state. Add content/root/recovery validation rather than renaming that cursor.
Genesis self-bonds debit funded owner balances and credit f100 once; preserve
supply and declared escrow, never add issuance.

B1 gate is deliberately NOT_IMPLEMENTED until its real coverage exists. The
20-bulk registry, R01–R12 and future security/interop/public-persistence tests must
remain present; `verify --all` currently fails rather than omitting them.

## Invariants and boundaries

Validators decide finality; master follows. Never sign without execution/data and
durable anti-double-sign state. Public RAM state retains durable recovery inputs;
applied/durable/authenticated heights stay distinct. Zones/nearest endpoints do
not grant authority. Preserve fee 40/30/30 and the secured 1M finalized TPS target.

The classical baseline assumes less than one-third Byzantine voting power and
strictly greater than two-thirds unique valid power. 51% continuity remains
UNSATISFIED_BY_BASELINE. No hybrid/secure-profile/custody/majority/throughput claim
comes from B0 primitives or metadata. Future liquidity/stabilization is deferred.

Use English shared prose and clean publication. Never stage local-tests/raw keys,
logs, databases, build/dependency output or personal config. Inspect the index and
all outgoing commits before a non-force push; preserve unrelated user edits.
Mainnet/genesis economics, real custody/keys/funds, paid infrastructure, licensing
and visibility changes retain explicit owner boundaries.
