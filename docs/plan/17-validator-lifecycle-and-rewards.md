<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# 17 — Validator lifecycle, node work and rewards

Status: executable **development** economics. Parameter values below are test-network defaults, not approved mainnet tokenomics or a security/economic audit.

## V01 — Registration and custody

Public node operation needs no stake. Reward-bearing node registration and validator registration are separate roles and require their respective bonds. The same operator may hold both roles but cannot receive two payments for one work item.

Use a metered, journaled native system interface reachable by ordinary signed EVM transactions at reserved development address `0x000000000000000000000000000000000000f100`. The address is rejected as a user genesis/code deployment allocation. All native/system effects are versioned EVE extensions, not ordinary Ethereum precompiles.

B0 freezes ABI selectors, event signatures, canonical system-record encodings and gas-cost vectors for: registerNode, registerValidator, delegate, undelegate, claimUnbonded, claimRewards, rotateConsensusKey, setCommission, submitWorkReceipt and submitEvidence. Read calls expose registry, pending activation, bonded/unbonding balance, work tasks and claimable rewards. Owner authorization uses the recovered EVM sender; consensus-key registration/rotation additionally requires proof of possession bound to genesis, chain ID, owner, role and nonce.

The native interface writes a system-state journal and performs EVM escrow transfers in the same transaction overlay. A failure restores both. No handler accesses a master's database, external service or local wall clock. Meter signatures, calldata, proof bytes and storage using a frozen native gas schedule; unsupported/oversized calls fail explicitly. B0 must include worked gas vectors before B5 implementation.

Initial local minimum self-bond: validator 10,000 development EVE; reward-bearing node 100 development EVE; 18 decimals. Enforce real escrow funding, duplicate-key rejection and checked arithmetic. These choices must remain visibly development-only.

## V02 — Lifecycle

```text
REGISTERED -> BONDED/PENDING -> ACTIVE -> EXITING -> UNBONDING -> WITHDRAWN
                                  |
                                JAILED -> eligible rejoin or EXITING
```

Epoch length is 1,000 finalized execution blocks. Epoch E contains heights `E*L+1` through `(E+1)*L`. Registry changes are evaluated deterministically at the specified epoch boundary, and consensus-set updates follow the selected adapter's activation delay. Never change voting weights by reading an off-chain stake balance during voting.

Initial selection policy: up to 64 eligible validators, descending effective bonded stake, public-key bytes as a deterministic tie-breaker. Effective power is floor(backed base units / 10^18), subject to positive power and the engine's total-power bound. Staking eligibility requires the self-bond minimum; delegated stake does not waive it. Invalid/empty sets are rejected. This dev policy is transparent and stake-weighted; it is not a claim that a large number of identities means decentralized ownership.

Store historical sets/weights and pending updates. Withdrawal never makes previously slashable stake immediately unavailable. Initial unbonding requires both seven days of agreed consensus time and 2,000 finalized blocks after exit eligibility; evidence windows and adapter delays must fit inside it. Test-only accelerated profiles must use separate genesis/config identities.

Consensus key rotation activates through a finalized registry change. Retain old-key evidence and signing safety; fence the previous process. Rotating a P2P identity alone does not rotate a consensus key.

## V03 — Validator availability and work

Use consensus-authenticated participation, not a process heartbeat or self-reported uptime. For each height, count whether the validator signed the relevant decided commit and its applicable voting power. The agreed last-commit information becomes available to application accounting in the following block; close an epoch only after its last height's participation has been accounted for.

Development reward weight is the sum of applicable voting power for successfully signed commit opportunities in the epoch. This incorporates availability and completed consensus work; stake alone without participation earns nothing. Late locally observed gossip is not an additional reward input. Slashing/jail eligibility and missing-vote windows are deterministic state.

Do not automatically slash a node merely for disagreeing with a root, receiving a delayed packet or rejecting a malformed proposal. A penalty requires an objectively specified evidence rule.

## V04 — Node work: no rewards for invented traffic

The first rewarded work class is protocol-assigned delivery/availability of authenticated block or snapshot samples. Tasks identify network, epoch, unique task ID, assigned node, content commitment, bounded sample/chunk request, fixed work units, deadline height and eligible attestors. Assignment is deterministic from finalized protocol inputs and rotates among eligible registrations under a per-block task budget; no arbitrary master assigns monetary scores.

A receipt binds the task ID, request nonce, node identity, returned content hash and completion height, and carries the node signature plus the required distinct active-validator attestations. Attestors validate the content against its committed source and the task deadline. Reward verification uses the applicable validator set and rejects duplicate/expired/unassigned tasks. Persist used task IDs to prevent replay across epochs, restarts and chains.

Initial task limits: at most 32 new tasks per block, bounded request/response sizes, and one payout opportunity per task. Work units are assigned before execution; a node cannot inflate them by sending extra bytes. A heartbeat, arbitrary RPC request count or self-generated benchmark request earns zero.

For node i: availability is completed assigned tasks / eligible assigned tasks. The initial minimum is 8,000 basis points; below it the epoch score is zero, otherwise score equals the sum of accepted assigned work units. Zero assignments produce zero work score, not automatic full uptime rewards. Rewards are paid only from the funded node pool.

This scheme provides auditable task/attestation accounting, not a proof that a node stores a unique independent replica, that attestors cannot collude beyond the consensus assumptions, or that every RPC customer was served. Nodes may retrieve a sample on demand. Economic/Sybil/collusion testing and independent review are required before mainnet. Do not label these receipts a novel proof-of-storage system.

## V05 — Fee ledger and allocation

Plan 13 credits fee pools once per block. At epoch close, split each pool proportionally to its nonzero role scores using integer floor division. Carry residual dust and unallocated funds forward in the same pool. An empty eligible set does not redirect the pool to master, treasury or proposer.

For a validator's gross allocation, deduct its snapshotted commission first; the remaining amount is shared proportionally across self stake and delegated stake. The operator receives commission plus its self-stake share. Development commission defaults to 10%, capped at 20%, with changes delayed to a future epoch. Implement bounded lazy reward indices and remainder accounting rather than iterating through every delegator per transaction. Joining/leaving mid-epoch must not capture earlier rewards.

Claims decrement funded liabilities and escrow before releasing value in an atomic journal. A repeated claim cannot pay twice. Keep a supply equation over actual EVM account balances, including escrow accounts, plus a cumulative burned counter. Do not count the same escrow balance again as circulating value merely because a reward liability exists. Genesis supply changes only through specified burns/authorized future issuance; default issuance is zero.

## V06 — Penalties

Use verified engine-compatible duplicate-vote/equivocation evidence with a unique evidence ID, offending height/set, signatures and age checks. Initial proven double-sign penalty is 500 basis points of applicable slashable stake, burned, plus jailing; delegated exposure follows its recorded backing at the offense. Evidence may be applied once only. Preserve slashable unbonding balances until expiry.

Downtime in the devnet reduces earned participation rewards and may trigger a deterministic jail after the configured missed-vote window; it does not automatically burn principal. Specify the threshold/window in chain config and test boundaries. Initial window: 1,000 eligible heights, jail below 80% participation, rejoin no earlier than the next epoch after owner acknowledgement. Consensus cannot process these transitions while quorum itself is unavailable; there is no off-chain master removal workaround.

Invalid node-work submissions are rejected. Missing tasks lose rewards. Financial slashing of node service failures is disabled until objective evidence and adversarial tests are implemented and approved; do not punish unverifiable customer complaints.

## Acceptance

T-V01: registration/delegation/exit transitions, duplicate keys and proof-of-possession checks.
T-V02: underfunded bonds, overflow and unauthorized owner actions fail atomically.
T-V03: validator activation/rotation/jail follows the actual consensus height delay.
T-V04: prior-epoch rewards cannot be captured by newly bonded stake; self/delegated shares and commission conserve pools.
T-V05: no-vote validator and online-but-no-work node earn no applicable work reward.
T-V06: self-created tasks, replayed/late receipts, forged attestors and inflated work units earn zero.
T-V07: restart/replay cannot repeat burn, claim, reward, evidence or task completion.
T-V08: valid equivocation slashes once; false/stale/wrong-chain evidence does not slash.
T-V09: unbonding remains slashable through configured windows and cannot withdraw early.
T-V10: randomized transfers, reverts, claims and penalties preserve the supply/escrow invariants.
