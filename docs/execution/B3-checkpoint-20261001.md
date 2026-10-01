<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# B3 checkpoint — incomplete and paused by owner

Status: PAUSED_BY_OWNER / INCOMPLETE. The owner requests stopping development
because the PC runs other programs and publishing the current state as a branch
checkpoint with a draft PR to main, without merge. This is not completed B3 or
authorization to resume. Main remains at accepted B2 commit b5f98df until a later
review/owner decision. The checkpoint commit and PR are identified by Git and the
attached artifact rather than a self-referential source hash.

## Implemented checkpoint scope

The validator runtime composes the pinned native engine with private authenticated
Unix ABCI/signer actors, actual canonical execution, durable anti-double-sign
history and exact application replay. Four independent Linux development processes
use separate disposable keys and engine/WAL/state/signer/replay namespaces. Master
does not participate in ordering, voting, signing or durable acknowledgement.

Non-nil voting requires actual execution/data approval for the current complete
parent/context, with synchronization before a new signature is released. The
native partial ProcessProposal callback is trusted for its validated hash/data
binding through the actual child/credentials/pidfd/held-image capability. It is
not an independently reconstructed full-header proof. Portable certificates use
canonical header/data/set hashes, individual native ZIP215 signatures and the
applicable trusted historical roster.

Public-owned opaque storage provides bounded exclusive synced compare/append.
Validator-owned records enforce identity, H/R/S, canonical sign bytes, signature
validation and recovery fencing. EVE point enrollment is canonical, nonidentity
and prime-order through existing pinned Dalek APIs. Native generic ZIP215 hashing/
verification remains compatible, including the preserved mixed-order case.

The disabled-by-default acceptance adapter binds an exact bounded manifest and
authority in canonical genesis contract code. Real signed EVM calls/successful
receipt events drive bounded rotation/leave/jail. Native activation remains H+2;
tests check H+1 next-set and actual persisted H+3 callback metadata. Normal builds
reject marked fixture genesis. B5 must replace this temporary development adapter.

## Actual complete-gate result

Command: cargo xtask verify --bulk B3. Outcome: FAIL, exit 1.
Base: b5f98df359eadfc83983b22eea39a249bd03fdbb, task-owned dirty branch.
The attempted manifest contained 507 Cargo cases. The runner accepted 259 prior
cases before the network group returned 15 passed / 1 failed / zero ignored.
T-C06 failed because validator 1 exited before height 9 after all-node crashes.
This was not the earlier shared-checkpoint-selection failure. The underlying
Rust actor cause was lost by the then-generic error reporting and remains unknown.

Local-only report: local-tests/verify-254-1790848988209122086/report.json.
Tested source bundle:
9c9560cf74a1bd0e0ae27efe7a82bf374ebdd6ea13cd60b6448708702704e3e3.
Format, strict default workspace Clippy, tool identities, fresh client compilation
and preceding core groups succeeded. The snapshot structure checked 1,585 files
and ownership 1,619, both with zero violations. The reviewed 346-line declarative
policy was the only size warning. The runner stopped at the failed network group;
later validator/tooling groups and the final release/fingerprint were not completed
by this run. A failed group is not added to the report's accepted test count.

## Later scoped checks and remaining defect

Private bounded 0600 failure diagnostics now record fixed categories, application
height and last/requested H/R/S. Unknown strings, packets and key material are
redacted. The first actor failure cannot be replaced by a secondary shutdown error.
Three diagnostic tests, feature Clippy and structure passed at that snapshot.

Two diagnostic-only C06 reproductions passed on wrapper SHA256
f7e1d80e898bdfb8a14f6daa53ba04e868dfbf9127bb4b2c93dd8cc05e5210dc:
136.25 and 130.23 seconds, each one passed / zero failed or ignored / 15 filtered.
Neither produced a failure record or identified a fix. The harness now preserves
append-only restart logs, phase/binary metadata and a private failed namespace;
successful namespaces are removed after owned children stop. The original failed
namespace was removed by the previous lifecycle; its copied native logs remain.

A final library-only regression passed in 0.49 seconds for preserving an original
H/R/S refusal when the non-nil recovery cache is missing. It still returns an error,
emits no signature and leaves the durable cursor unchanged. This changes no signing,
refusal or storage policy. Strict lint/full gate were not rerun after that addition,
and the current CLI does not include it. The current registered validator inventory
is 92; the current complete manifest expects 511 cases, not a passing result.

A separate release workspace build completed in 1m04s during diagnostic preparation.
It is a scoped snapshot result, not the failed gate's final release or verification
of every later source edit. Raw commands/outputs remain local-only under
local-tests/b3-preparation/, including acceptance-tc06-diagnostic-1/2.txt and
role2-c06-refusal-cause-tests.txt. Do not publish generated binaries, logs or keys.

Native source review confirms WAL replay can attempt duplicate/older H/R/S,
and locked-block voting may omit a fresh ProcessProposal. Signing requests do
not themselves supply raw execution data. Native semantic refusal responses are
distinct from transport failure. These facts guide later diagnosis; they do not
prove which condition caused the observed exit or authorize bypassing EVE guards.

## Publication and boundaries

The checkpoint preserves the failing tests and B3 CI selection. Draft status and
owner-authorized publication do not convert failure to acceptance or authorize
merging. Lightweight final structure/ownership/index review is recorded separately
from software acceptance. Redcat permission-only notices and exact upstream rights
remain mandatory. Cargo audit 0.22.2 found zero known vulnerabilities for 469
identities, with derivative/paste maintenance warnings unsuppressed. No retained
registry identity/checksum was replaced. This is not a comprehensive security audit.

Classical safety assumes less than one-third Byzantine weighted power and strict
3*S > 2*T. No PQ activation, 51% continuity, hardware power-loss certification,
coherent-backup rollback detection, cross-host key-clone fencing, standalone copy
distribution or secured TPS target is achieved. B4/B5/B6 and later goals remain
unfinished; no mainnet/funds/custody/spending or external program is authorized.

The owner also requires future publications after this checkpoint, upon later
resumption, to contain `.md` files only named README.md. The owner explicitly
exempts this PR's timing. Document migration/reference repair/enforcement remain
NOT_IMPLEMENTED; current Markdown files are not claimed compliant with that future rule.

After new owner input only: inspect git status, rebuild the feature validator,
record its SHA, reproduce retained C06 and repair the verified root cause. Then
run the complete frozen gate. Development is stopped at this checkpoint.
