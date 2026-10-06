<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# B4 phase 1 collaboration and verified local gate

Status on 2026-10-06: P1 DONE for the declared classical development contract.
The owner resumed after the local checkpoint. Complete local and hosted 743-case
acceptance is verified, and exact hosted revision 2202dc2 is integrated into main
by normal fast-forward. P2 follows; complete B4 closure remains in P3.

## Frozen local acceptance

Command: cargo xtask verify --bulk B3. On this revision its explicit profile is
CLASSICAL_DEV_B4_P1_AUTHENTICATED_STATE_RETAINING_COMPLETE_B3_NOT_PQ_OR_SECURED_TPS.
It selects all retained B3 packets plus the existing 85-case finality verifier.
This is the P1 packet; complete B4 closure remains in P3.

| Evidence | Verified result |
|---|---|
| Source revision | ca32145a08687687251cbe43f259e3e3687a46e8 |
| Source before/after | 79e51467cbb59a46706cb72a29e47b0168d97bcb9639943cbc8b19d120118694, unchanged |
| Working tree at gate start | Clean |
| Complete local packet | PASS: 743 passed; zero failed/ignored/pending/errors |
| Selected packages / recorded commands | 15 / 67 |
| Elapsed time | 2,985.475 seconds, approximately 49.76 minutes |
| Structure / ownership | 1,930 / 1,964 files checked; zero violations |
| Tool/compiler identities, format and workspace strict lint | PASS |
| Client lockfile installation and TypeScript compilation | PASS |
| Normal/acceptance binaries and workspace release build | PASS |

Raw evidence is local only: local-tests/verify-198-1791168973478164228/report.json.
A GitHub clone does not contain that report, generated keys, data or raw output.
Later pause documentation does not rerun or extend this frozen runtime acceptance.

| Selected package | Passed |
|---|---:|
| eve-crypto | 30 |
| eve-consensus-comet | 59 |
| eve-evm | 28 |
| eve-finality-verifier | 85 |
| eve-master / eve-node-policy / eve-protocol-config | 6 / 6 / 20 |
| eve-public | 48 |
| eve-serial-rpc-acceptance | 17 |
| eve-state / eve-state-recovery-acceptance / eve-storage | 61 / 37 / 60 |
| eve-validator-consensus-acceptance | 34 |
| eve-validator with development-acceptance | 103 |
| xtask | 149 |

## Claude repairs and integrator coverage

The candidate preserves a normal main/Claude history merge. Claude commits
7ee93f8 and a539d04 register four pre-existing storage prospective-cursor tests
and use the maintained one-shot admission/exact-block observation path for
submission scenarios. The integrator selects the omitted verifier group and
repairs the downstream compile fixture's incompatible state feature identity.
P1 has no sync-client package; later P2/P3 inventories are not copied into it.

The compile fixture still requires E0502 for mutating bytes while their borrowed
preflight is used. Its budget is inferred from the actual verifier API rather
than an independently selected incompatible eve_state artifact. Unexpected
success and unrelated compiler errors remain failures; no production verifier
logic, test identity or resource/security budget was changed for this repair.

The consensus packet passes all 34 cases in 858.23 seconds. Actual T-C05, T-C06
and T-C09 pass on this candidate, preserving malformed-proposal/nil-vote,
durable-signature/once-only effects and H/H+1 binding assertions. Public48 passes
in 312.48 seconds on the retained debug profile. No timed limit was raised.

Per-request three-second and connect one-second limits remain unchanged.
Submission helpers now have an explicit 90-second progress budget for the other
scenarios. Equal default propose/request timeouts alone do not prove failure
causation: startup waits height two before submission, and a complete rejected
proposal need not consume the propose timeout. Code/hash/height/index, exact
transaction bytes, independent certificate/replay and late-result checks remain.

Claude's separately reported two-core comparison and PR #9 CI cover its 658-case
packet. They are separate evidence and cannot replace the extra 85 verifier
cases or hosted acceptance at this complete candidate revision.

## Hosted acceptance and main integration

[Hosted run 37261123941](https://github.com/soulredcat/eve_evm/actions/runs/37261123941)
succeeds at exact 2202dc2d710f241ec2ae049e5d1a8a2d0755a83b and records PASS743.
Job 111608388041 passes all required steps through workspace release build.
The detailed hosted JSON is not uploaded; the maintained runner requires zero
failures/ignored/pending and unchanged final source to emit PASS. The duplicate
PR run 37261221907 is CANCELLED and provides no passing evidence.
Independent source/log review confirms the two README files are the only changes
from local ca32145 to hosted 2202dc2. This is documented source identity, not a
separate downloaded hosted JSON assertion.
Both main and the P1 branch were verified at exact 2202dc2 after normal atomic
fast-forward. PR10 and Claude PR9 are observed MERGED; phase PR6 is observed CLOSED
after source integration, not a separate GitHub merge commit.

Propagate this accepted base normally into P2 and P3 without rewriting published
history. P3 includes full B4 closure; there is no fourth phase. This acceptance
update is documentation only and does not rerun or broaden the runtime packet.

This is CLASSICAL_DEV component/consensus evidence, not PQ security-profile,
standalone role-copy distribution, independent-master replication, hardware
power-loss, sustained throughput or mainnet acceptance. B4 is not DONE yet.
