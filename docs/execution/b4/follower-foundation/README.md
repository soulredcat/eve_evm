<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Authenticated follower foundation: local evidence

This local B4 continuation is not published or integrated on main. Accepted B3
main remains c41ba2f. Full B4 is IN_PROGRESS with its complete gate unimplemented.
The following are measured component/runtime slices, not full-bulk acceptance.

## Behavior implemented

Public reconstructs complete segmented bodies from actual durable rows, checking
ordered physical membership, canonical offsets, whole-body SHA and complete
markers. Orphans remain stored; index-zero restarts reuse bounded RAM. Only later
canonical V2/H/H+1 verification can advance an authenticated logical prefix.

One applied owner supports compact and segmented persistence. It reserves every
part before candidate preparation, derives the verified target binding, checks
untrusted target metadata before enqueue and publishes one immutable RAM view.
ACK validation preserves actual logical/physical progress. Shutdown retains the
full unvalidated tail and its known physical acknowledgement metadata.

The shared download component leases through its caller's actual pool, bounds
HTTP/chunks/deadlines and assembles untrusted native H/H+1 material. Public derives
its reader and budget from the admission owner; another service cannot supply its
resource pool. Master independently authenticates the same wire, syncs protected
proof material before state/WAL commit and reauthenticates its archive on restart.

RPC and mempool retain the actual applied publication/generation across captures.
Current queries, proofs and simulation reuse canonical operations; unavailable
older history is explicit. eve_getStateRoots exposes both actual roots and honest
authentication metadata. Valid historical state never implies fresh-head readiness.

Streaming checkpoint verification starts from a private locally anchored imported
state and validates execution/native witnesses through H+1 without a history Vec.
Sealed complete-state preflight admits same bytes/frozen limits before owned maps,
code, system data, header and receipt materialization. These components do not yet
implement resumable snapshot files, durable checkpoint-base schema or activation.

## Executed local verification

| Scope | Verified result |
|---|---|
| State / complete-state preflight | Full 79 cases passed, including six deltas and twelve preflight cases |
| Complete-state preflight | 12 cases passed; strict lint passed after test-only repairs |
| Segmented recovery scanner | 12 new cases passed; storage packet 88 passed at that stage |
| Validator runtime | Full 88 passed with actual pinned native executable and required aliases |
| Verifier | Full 112 passed after checkpoint wiring and canonical fixture migration |
| Public packet | Full 92 release cases passed after root/CLI guard and structure repairs |
| Added public slices | Applied 8, segmented persistence 22, mempool 3, RPC 6, owner binding 1 passed |
| Root observability / CLI guards | Two root API and two profile/guard cases passed |
| Sync client | Eight cases passed, including actual sockets and modeled late materialization |
| Master archive | Full master packet 23 passed; seventeen archive cases and strict lint pass |
| Shared fixture component | Three canonical genesis/native/serial-chain cases passed |
| Cargo gate tooling | Full 154 cases passed, including five profile tests; no test skipping |
| Actual public CLI | Four validators + one public bootstrap/graceful restart slice passed in 75.21 seconds |

The actual CLI case finalized two nonempty transactions, compared receipts,
EVM/system roots, application commitment, balances and nonces with independent
replay, stopped/restarted the public child and verified restored durable-prefix
catch-up. It ran no master. It deliberately required NOT_READY for independently
unknown freshness. This is T-N01 and graceful process recovery evidence, not N05,
hardware power-loss, independent-host availability, secure throughput or all B4.

The large V2 input retained its 255 distinct unused-code auxiliary entries and
original limits. In unoptimized debug execution, preparation exceeded the default
2-second queue age and correctly refused admission. All eight cases passed in
optimized release in 1.05 seconds. The public gate now explicitly selects release
for listing, execution and documentation tests; other groups keep their default.
Evidence records cargo_profile. Neither workload nor admission limits were raised.
Unused code/contentDigest remains checked local auxiliary data, not separately
certified execution. Dynamic allocation-counter/RSS measurements are NOT_RUN.

Initial compile/lint/fixture failures are retained locally: missing native aliases,
large result/enum shapes, invalid test parent metadata, test type inference/borrows,
fixture mode mismatch and repeated private log paths. Repairs preserve security
checks and assertions. No skipped case is counted as passing.

## Required continuation and reproduction

Mandatory source checks initially rejected 12 role/function-policy violations.
Repairs use one validator-owned development-fixtures Cargo component through
DEV dependencies, named operations and exact test cfg boundaries.
Final structure 2,410 files and ownership 2,444 files passed with zero violations.
Format, strict lint and full workspace release build passed (48.05 seconds).
Raw reports, keys, databases and private process output remain ignored local-only
artifacts under local-tests/b4-preparation; they are not GitHub evidence assets.

Reproduce with pinned tool inputs and the versioned group inventories. The actual
CLI case additionally requires the built release EVE_PUBLIC_BINARY, actual
EVE_VALIDATOR_DEV_BINARY, EVE_VALIDATOR_NORMAL_BINARY and checksum-verified
EVE_COMET/EVE_COMET_SHA256. Do not substitute a version-text-only executable.

Finish authenticated snapshot transfer/resume/activation, actual master-offline
missing-tail loss recovery, retention/last-copy policy, independently observed
readiness and resource/interference/fault acceptance. Preserve all mandatory B4,
structure, ownership, classical regression and core security requirements. Publish
one completed bulk batch with English evidence, require hosted acceptance and
integrate normally into main. This report grants no production launch authority.