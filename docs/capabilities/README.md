<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Capabilities and engineering maturity

EVE has working development foundations for deterministic EVM execution,
durable canonical state, bounded public RPC and real validator consensus.
Its engineering maturity is demonstrated by actual positive/negative tests,
independent replay and process fault scenarios. It is not a completed production
network. See [revision-bound progress](../execution/README.md) for results.

## Implemented development capabilities

| Capability | Canonical implementation | Evidence and practical boundary |
|---|---|---|
| Deterministic Shanghai EVM | [Validator execution component](../../validator/components/execution/README.md) | Actual signed transactions, contract execution, gas, logs, receipts and revert behavior; later forks require explicit work |
| Canonical complete state | [Validator state component](../../validator/components/state/README.md) | EVM/system roots, execution identities and replay/journal agreement; a root alone grants no finality |
| Atomic durable storage and snapshots | [Public recovery store](../../public/components/recovery-store/README.md) | Real sync/reopen, corruption rejection, exact replay and staged local snapshots; distributed recovery remains B4 work |
| Public Ethereum-style RPC | [Public runtime](../../public/README.md) | Actual RPC/client/Solidity flows, bounded admission/query/worker resources and restart tests; documented EVE compatibility/economic differences apply |
| Native consensus integration | [Validator runtime](../../validator/README.md) and [Comet boundary](../../validator/components/consensus-comet/README.md) | Actual separate native engines and execution-before-vote with strict weighted quorum; current hosted T-C07 remains unresolved |
| Durable signing safety | [Validator acceptance](../../tests/acceptance/validator-consensus/README.md) | Real signer/application crash recovery and signature-prefix checks; four processes on one host are not independent failure domains |
| Public resource contracts | [Node policy](../../public/components/node-policy/README.md) | Typed budgets, readiness/watermarks and source eligibility predicates; caller metadata is not verification authority or OS enforcement |
| Pinned verification infrastructure | [Testing contract](../testing/README.md) | Exact tools/source identities, registered tests, structure/ownership, strict lint and release gates; missing or skipped coverage fails |

Fees preserve 40% burn / 30% node rewards / 30% validator rewards with integer
accounting. Native staking, delegation, work rewards, slashing and user lifecycle
integration remain B5 requirements; the B3 transition fixture is temporary
acceptance material and cannot substitute for those modules.

The current B3 review candidate also checks native submission codes, exact
transaction hash identity and canonical positive height. Fixed diagnostic codes
identify the failing action and RPC stage without publishing private payloads.
Its complete local B3 gate passes 539 cases at `9362b22`; hosted candidate
acceptance remains pending. These checks improve validation and diagnosis,
not throughput.

## Partial foundations under development

The local B4 checkpoint adds source-independent native set/header succession,
private EVE application-anchor capabilities and bounded public handoff/storage
worker primitives. Its 42 focused tests include real native signatures, actual
Shanghai execution, downstream compiler privacy negatives and real repository
sync/reopen. That source is not yet integrated into main.

B4 still needs authenticated complete recovery payloads, consistent applied RAM
views, truthful durable/authenticated watermarks, follower runtimes, peer-tail
retrieval and integrated storage/fault/retention acceptance. Master remains an
archive/follower role and never receives validator finality authority.

## Unachieved release capabilities

| Area | Current boundary |
|---|---|
| Standalone copied public/validator packages | NOT_IMPLEMENTED / NOT_RUN; monorepo builds are not distribution proof |
| Parallel execution capacity | Serial equivalence and measured scheduling remain later gates |
| Master replication and regional deployment | Planned topology requires real verification, failover and operations evidence |
| Activated hybrid/PQ security | NOT_ACHIEVED; classical Ed25519/secp256k1 paths are not PQ-secure |
| Majority-adversary continuity | Baseline assumes less than one-third Byzantine power and strictly more than two-thirds valid weighted power; no unconditional 51% immunity |
| Sustained secured 1M finalized TPS | NOT_ACHIEVED; [performance contract](../performance/README.md) defines required evidence |
| Production/mainnet readiness | Unachieved software/security/operations and owner-controlled release requirements remain |

Bridges and external-chain programs are deferred until EVE testnet and later
scope. They do not enter core execution, voting, finality or durable acknowledgement.
The [roadmap](../plan/README.md) records dependency order; capabilities become
accepted only through implementation, integration and their actual required gates.
