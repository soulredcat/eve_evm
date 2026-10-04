<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Master authenticated archive follower

This capability belongs to master. It keeps the actual canonical imported state
in RAM and maintains a private complete-state recovery repository together with
the native validator proofs and journals needed to reauthenticate it. It depends
on canonical validator state/finality verification, the public-owned recovery
component, and the shared sync client. It does not depend on the public runtime
or its private applied orchestration.

The scoped entry point is `follow-dev`, requiring exact `MASTER_SYNC_ONLY`,
`CLASSICAL_DEV` genesis and explicit unsafe-development acknowledgement. It has
no voting, proposal, producer, release-signing or private-key capability. Its
plain TCP source is explicitly loopback-only classical development; this is not
a production authenticated transport or quantum-security claim.

```sh
cargo run --locked -p eve-master -- follow-dev --root . --data local-tests/master-follower --genesis local-tests/development.json --mode MASTER_SYNC_ONLY --acknowledge-unsafe-development --native-address 127.0.0.1:26657 --through-height 2
```

Catch-up is finite and ordered. Each next H requests its delta and actual native
H/H+1 frames through the same shared untrusted assembly used by the public
follower. Master independently calls the canonical import verifier and compares
the complete computed target with advertised metadata before any disk write.
The importer verifies parent auxiliary metadata, historical certificates,
transaction/header context, EVM/system/receipt/code/history consistency, and
the applicable application commitment. It does not reexecute transactions or
reapply fees. Independent execution replay remains a distinct verifier mode.
Auxiliary content digest is a local complete-data binding; it is not separately
certified by the EVE application commitment.

## Ordered durability and restart

One exclusive staging file receives the canonical logical V2 proof body. The
file is synced, made read-only and promoted without replacement to its immutable
height name; the directory is synced before canonical `commit_state` performs
the actual synced database/WAL commit. Namespace creation also syncs every
containing directory through the validated local root. Acknowledgement and RAM
publication follow both proof and state sync completion. Any ambiguous write,
sync or publication outcome fences the owner; further imports require reopen.

Startup selects its trust only from local canonical genesis. It sequentially
decodes and reauthenticates every retained native proof, then compares the
resulting exact whole commit with each retained database commit. Local roots,
checksums, files and database consistency alone grant no finality authority.
Missing completed proof, corrupted completed body, filename gap or unexplained
suffix rejects startup. A single completed proof ahead of the database is
reauthenticated and resynced before state reconciliation. A complete staging
proof is reauthenticated, synced, promoted and reconciled. Neither recovery path
reapplies imported fees.

Malformed uncommitted staging may occupy one bounded rejected-staging slot.
That body remains present and a fresh valid attempt may proceed if capacity
allows. An occupied rejected slot refuses another malformed staging body
without replacing either body. Completed proof files and retained state commits
are never pruned. Configuration changes that cannot contain existing recovery
material refuse startup instead of silently losing it.

Proof access is descriptor-relative through safe pinned rustix APIs. Symlinks,
nonregular files, hardlinked proof files and unexpected names reject. Exclusive
creation and NOREPLACE promotion prevent overwriting recovery material. The
verified file/directory-sync implementation currently requires Linux; other
native platforms reject before creating an archive. These guarantees are
ordinary fallible syscall ordering, not a hardware power-loss certification or
protection from a privileged operator deleting the namespace.

## Bounded resources and truthful status

The local reference profile declares a 512 MiB logical working pool, a separate
32 MiB CLI genesis-input/decode allowance, bounded RocksDB buffers/cache/jobs,
the existing 8 MiB logical state and 16 MiB complete-commit limits, 4,096 completed
proof files, at most 1 GiB of actual retained proof/staging bodies, and at most
1 GiB of actual retained canonical complete-commit payloads. Both staging slots
and next-commit headroom must fit configured ceilings. This is not a retention
time-window promise; physical RocksDB file sizes, compaction overhead and RSS
are not those logical payload counters.

Real owned semaphore permits cover raw file allocation before read, bounded
canonical sizing scratch, count-derived decode/reencode/native-proof copies,
candidate cloning, retained imported state, and the conservative repository
decode/root/projection/batch envelope. Shared downloads retain both actual
caller-provided leases until their wire leaves scope. The master conservatively
retains its whole import charge with the current private capability. Pressure,
overflow or capacity exhaustion returns an error before covered allocation or
fresh persistence; it does not lower consensus limits or trigger pruning.
Archive inventory is incremental and never collects a filename listing.

The status reports only the confirmed RAM/durable/authenticated prefix.
Genesis is locally trusted with no certified H outcome. A fenced owner reports
unknown storage outcome explicitly because its actual disk head may have
advanced after a lost acknowledgement. Peer head and lag remain null and ready
remains false until authenticated freshness observation is implemented.
Historical authentication alone does not claim current readiness.

## Source regression coverage

The master unit cases use actual signed H/H+1 fixtures from the canonical
development-fixtures component through a dev dependency, with local journal
projection and V2 encoding. Thin local conversions construct this consumer's
wire DTOs and delegate native signing to that shared fixture. Their signing or
consensus logic is not copied into master.

Cases cover exact whole-commit restart, exclusive owner, real permit pressure,
role/profile/acknowledgement rejection, count capacity without pruning, invalid
signatures and wrong certified roots, proof-ahead sync, lost state
acknowledgement, valid and partial staging, an occupied rejected slot, auxiliary
whole-commit mismatch, absent/corrupted proof, unexpected names, symlink and
hardlink refusal, and oversized-file rejection before payload allocation.

Reproduction: `cargo test --locked -p eve-master --lib sync::tests`.
Source coverage is not a test-result attestation. Actual native-process catch-up,
master-offline public operation and full B4 acceptance remain separate gates.
