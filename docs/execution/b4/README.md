<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# B4 authenticated followers and durable checkpoint progress

Status on 2026-10-04: PAUSED_BY_OWNER. Accepted B3 classical development runtime
remains on main at `c41ba2f351f17ad349efccb1e447e49822480c99`.
The frozen B4 implementation candidate is `f10482e3f7cbf9557e682929cc459d421cb8c24c`.
The owner authorizes publishing this unfinished checkpoint as a draft PR, without
B4 runtime integration into main. Resume development only when the owner asks.

The complete local `cargo xtask verify --bulk B4` attempt at clean `f10482e`
failed before executing tests: the worktree toolchain receipt was missing.
Its structure/ownership checks passed; the packet executed zero tests.
Canonical provisioning then completed Go/Comet extraction, Comet build/module
verification, OpenSSL configuration/build and Node extraction. The owner pause
stopped provisioning during client installation. No complete tool receipt or
passing full B4 packet is claimed. Task-owned local actors have ended.

Component results below are executed local source packets, not complete B4
acceptance. The executable manifest and T-N09/T-N10 coverage are IMPLEMENTED;
hosted B4 acceptance remains NOT_RUN. Keep mandatory gates before runtime merge.
No background monitoring or automatic resumption is scheduled.

Review and remaining work are split into [three phase drafts](phases/README.md).

## Implemented source capabilities

The [follower foundation](follower-foundation/README.md) records the earlier compact
and segmented integration. Public retains one immutable charged RAM publication;
finalized recovery bytes follow an isolated, ordered, bounded WAL/sync path.
Actual marker acknowledgment advances logical durability. Physical record sequence
is distinct from execution height. Orphans and failed tails retain their bytes and
leases; queue admission, age, lag and storage errors preserve truthful watermarks.

Public and master use one bounded own-chain download component and canonical
native H/H+1 verification. Master remains a private authenticated archive follower,
outside the mandatory transaction path and without validator voting authority.
Public/validator source does not depend on private master implementation.

The [checkpoint application](../../../public/src/sync/applied/checkpoints/README.md)
authenticates complete state against locally configured genesis and ordered native
execution witnesses through H+1. [Content/proof storage](../../../public/components/recovery-store/src/checkpoints/README.md)
supports resumable private staging, immutable completed content, bounded safe-file
checks and conservative repair. Unknown, foreign, future or oversized staging
metadata cannot be deleted as a benign retry. Checksums establish local integrity;
they do not establish finality.

A typed base record uses the existing sole writer's real synced acknowledgment.
Conditional activation preserves the old usable charged publication until the
complete replacement is durable and the captured parent still matches.
Checkpoint-aware reopen reauthenticates retained content/proofs from local genesis,
then verifies the actual base and ordered suffix. Nonzero checkpoint height cannot
be paired with a fabricated bootstrap cursor. Missing/corrupt artifacts refuse.

[Validator snapshot queries](../../../validator/src/consensus/application/serving/snapshots/README.md)
use current RAM cache first and one matching durable repository snapshot; historical
decode is charged from the actual retained encoded length. Manifest/chunk/execution
responses remain untrusted. The [shared client](../../../public/components/sync-client/src/checkpoints/README.md)
retains actual caller capacity, exact source identities and one absolute deadline.

The [public CLI](../../../public/src/runtime/follower/README.md) accepts an explicit
development checkpoint height and otherwise reopens checkpoint-aware storage.
It uses 32 KiB chunks, bounded proof passes, a 300-second whole deadline and
five-second request ceilings. Its current loopback native JSON profile may refuse
larger valid responses. Ordinary queries capture one coherent applied version;
unavailable old history and independently unknown freshness remain explicit.

Owner-bound storage admission enforces two active storage jobs and 16 MiB raw
staging, retaining a real working-pool control lease. The controller size helper
includes its body and Arc counters. Linux thread CPU accounting cooperatively
paces the sole writer at 2500 basis points after each bounded physical record,
without banking idle credit. Database background work and other roles remain
outside that writer CPU scope.

## Executed local regression and process evidence

All successful packets listed here had zero failed or ignored cases. Full-source
format, structure, ownership, release and complete B4 verification must still be
rerun on the final frozen source and recorded with exact identities.

| Executed scope | Recorded result |
|---|---|
| Full public release, after controller repair and maintenance extension | 150 passed; 9.71 seconds; strict all-target Clippy passed |
| Earlier full public attempt | 148 passed, two failed; repaired and superseded by the 150-case rerun |
| Finality verifier checkpoint/import packet | 125 passed; strict lint passed at that source stage |
| Snapshot messages / validator snapshot serving | Nine / five passed |
| Shared nonempty-tail fixture addition | Two new cases passed; previous three cases preserved |
| Real storage admission / writer CPU cases | Four / two passed |
| Simulated signed nonempty-tail recovery | Two passed; 0.24 seconds |
| Targeted T-N09 measurement | One passed; 0.27 seconds |

The actual process cases use four real validator/native-engine processes and
release public/master executables with pinned executable identities. No master
serves public recovery in the public cases.

| Actual single-host development case | Result and scope |
|---|---|
| Public bootstrap and graceful restart | Passed; 75.21 seconds; nonempty roots/receipts/nonce/balance versus replay |
| Public SIGKILL and later validator-tail retrieval | Passed; 75.51 seconds; same namespace and authenticated durable peer data |
| Checkpoint bootstrap, nonempty tail and default restart | Passed; 68.77 seconds; actual completed checkpoint and reopened suffix |
| Interrupted checkpoint and same-content resume | Passed; 81.09 seconds; old prefix preserved before activation |
| All configured public replicas lost, masters absent | Passed; 87.66 seconds; rebuilt from retained durable validator data |
| Master absent during another transaction, same-archive restart | Passed; 68.07 seconds; complete retained commits/proofs versus replay |

The nonempty loss unit cases explicitly model SIMULATED_UNSYNCED_LOSS: H1 data and
marker are acknowledged at physical rows 1/2, H2 data at row 3, and its marker is
unwritten. A new real WAL namespace receives only those actual acknowledged rows.
Original data is preserved. Recovery reads a separate real peer WAL, verifies
signed H/H+1 input, reproduces the whole canonical H2 commit/receipts/nonce-two/fee
state, rejects duplicate effects, and reopens exactly. Missing/corrupt-only selected
peer data preserves H1, missing-from height 2 and NOT_READY. Thirty-two repeated
bad/stale reads preserve baseline leases and zero pending queue/parts.

These simulations and real process cases are distinct evidence. Combining them
does not establish hardware power-loss behavior, independent operator/failure
domains, unlimited retention or impossible recovery after every valid copy is lost.

## T-N09 measured local contract

The versioned [measurement contract](../../../config/gates/measurement-b4.toml)
was frozen before the targeted CLASSICAL_DEV_LOCAL run. It uses 128 baseline
rounds and 128 pressure rounds, each with two concurrent real production loopback
HTTP calls: eth_getBalance and eth_call. Pressure performs 128 actual checkpoint
content/proof sync jobs while the real public WAL writer is paused. Oracle values,
old/new captured views, shutdown and reopen are checked.

Declared limits are 100 ms small-fixture RPC p99, one-second operation deadline,
512 MiB observed process peak RSS, 256 MiB estimated working capacity, 16 MiB raw
staging, two storage jobs, four retained parts, 32 MiB encoded queue, two-second
queue age and two-block logical lag. This is a local interference contract.

| Metric from that targeted run | Recorded value |
|---|---|
| Baseline balance / call p99 | 0.275687 / 0.845390 ms |
| Pressure balance / call p99 | 0.313717 / 0.821512 ms |
| Sampled peak RSS / process lifetime HWM | 20,447,232 / 20,447,232 bytes |
| Peak storage jobs / raw staging | Two / 16,777,216 bytes |
| Peak estimated working reservation | 32,374,217 bytes |
| Writer measured CPU / elapsed wall | 152,679 / 149,236,597 ns |
| Writer pacing sleep / largest record CPU burst | 100,949 / 144,209 ns |
| Writer physical record operations / observed queue age | Two / 150 ms |

The RSS samples and lifetime HWM belong to this small targeted process workload.
Logical leases are separately enforced admission estimates. No OS memory or CPU
hard quota is claimed. Writer pacing excludes RocksDB background threads, public
verification and RPC CPU. The bounded pause is a test fault, not hardware fsync
stall or instantaneous CPU enforcement. No finalized TPS result is measured here.

The extended standalone release measurement passes in 2.10 seconds with 128 actual
checkpoint jobs, 128 KEEP_ALL compactions and 128 secondary-index jobs, alongside
production HTTP RPC and the paused sole WAL. Compaction/index namespaces are
separate; one auxiliary DB opens at a time within the unchanged combined cache,
buffer, job and file limits. Retained payloads/cursors/indexed state remain exact.
Balance p99 is 0.263691/0.443699 ms; eth_call is 0.758976/1.080138 ms
(baseline/pressure). Sampled RSS and process HWM are 22,716,416 bytes.
Reads peak at 2, staging at 16,777,216 bytes and working estimates at 32,374,217.
Writer CPU/wall/sleep are 95,019/1,984,855,491/43,585 ns; its maximum record
CPU burst is 85,499 ns across 2 records. Queue age is 1,985 ms. These newer
metrics attest only this declared local fixture, not sustained network capacity.

## Reproduction, gates and remaining acceptance

Use the repository-pinned toolchain, Cargo.lock, tool/binary identities and complete
versioned test inventories. Preserve the explicit release profile for timed public
tests. Native process cases require the actual pinned Comet binary/digest and
development/normal validator plus release public/master binaries prepared by xtask.

```text
cargo xtask check-structure
cargo xtask check-ownership
cargo xtask verify --bulk B4
```

The last command failed before tests on this checkpoint; rerun after completing
canonical tool provisioning. An executable
manifest is not acceptance. Complete remaining T-N01–T-N07, T-N09/T-N10,
T-S04–T-S08 and T-G04/T-G06 coverage, retention/last-copy refusal and truthful
freshness/readiness/resource behavior before marking B4 complete. Preserve all
prior classical/core/security/role gates; no test omission or budget increase
may manufacture success.

Raw logs, seeds, databases, executables and machine configuration remain ignored
local-only artifacts under local-tests/b4-preparation. This reviewed summary and
reproduction inputs may be published in the completed bulk; raw artifacts may not.
The owner-authorized draft checkpoint is an explicit publication exception to
completed-bulk batching. Audit its index and every outgoing commit; do not merge
unfinished B4 runtime code. Resume, pass local/hosted gates, then integrate into main.

Classical development success does not satisfy PQ SECURITY_PROFILE_ACCEPTED,
standalone role-copy build/run, independent master HA, production transport,
mainnet readiness or the 1M aggregate finalized TPS goal. External-chain and
bridge programs remain deferred until EVE testnet; EVE core has no remote-chain
runtime dependency.
