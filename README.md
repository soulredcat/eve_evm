# EVE EVM

An engineering project for an EVM-compatible network with separate public access, validator execution/consensus, and developer-operated durable synchronization infrastructure.

**Status: B0 including SEC0/INT0 and B1 passed their complete local gates; latest B1 runs 271 tests, structure, format, strict lint and release build. Complete state, atomic persistent recovery and the development-only master harness are implemented. Complete network roles, standalone distributions, devnet/security acceptance and measured TPS remain unachieved.**

## Main objective

Build a correct, recoverable, independently verifiable network and pursue **1,000,000 aggregate finalized transactions per second**, using explicitly declared workloads and hardware. This target is not achieved by receiving requests, adding replicas, or publishing state hashes alone.

## Production authority

> Validators decide. Master remembers.

```text
Users / dApps
       |
Public nodes: RPC, transaction ingress, P2P, verified state
       |
Validator network: execute, propose, validate, vote, finalize
       |
Finalized blocks + authenticated state commitments
       +----------------------+
       |                      |
Public / validator peers      Master replicas
recent durable data           sync, durable state, snapshots, recovery
```

Master acknowledgements are not required for ordinary transaction finality. Public nodes are permissionless, with no fixed protocol-wide count; active validator membership and voting power follow staking rules. Master infrastructure is operated by developers, but its signatures do not create consensus authority.

A validator needs durable signing-safety and recovery records even when its working state is in RAM. Losing validator quorum stops new finality; there is no automatic master takeover.

Default public nodes keep active verified state in RAM and finalized recovery blocks/checkpoints durable through a separate bounded storage worker. Execution/current-state reads do not hold state locks while waiting for disk; applied and durable heights remain distinct. Storage still consumes resources, so latency interference and lag must be measured. Public restart recovery must work from retained verified data while masters are offline.

One regional master can serve several public nodes through a preferred nearby eligible sync endpoint with fallback peers. Public does not require private master topology or database access. The planned evolution is two masters synchronizing independently verified finalized history, then ten regional masters; this is not launch authorization or a throughput claim. `zone_id` affects routing/placement only. See [plan 32](docs/plan/32-regional-masters-and-public-persistence.md).

## Source layout

- `master/`: protected synchronization and storage runtime.
- `public/`: independently distributable public RPC/P2P runtime.
- `validator/`: validator runtime; it may be co-located with a public node.
- `public/components/recovery-store/`: bounded durable recovery record storage owned by the public role.
- `public/components/node-policy/`: validated public resource, watermark, readiness and source-selection policy.
- `public/components/interop/`: typed public chain/asset/route metadata and exact amount codecs; custody stays disabled.
- `validator/components/execution/`: deterministic EVM execution owned by the validator role.
- `validator/components/authentication/`: reviewed authentication integration owned by the validator role.
- `validator/components/consensus-comet/`: pinned engine API, framing, height binding and unsupported hybrid guard.
- `validator/components/protocol-config/`: development genesis, native ABI/gas and canonical record/header contracts.
- `validator/components/state/`: complete logical state, canonical roots, journals and structural recovery commits.
- `validator/components/bridge-protocol/`: restricted source-verification/custody capability interfaces; actual verifiers and custody remain pending.
- `xtask/`: structure, verification, evidence and task-local tool provisioning.
- `docs/plan/`: normative development specifications and dependency-ordered work.
- `docs/agents/`: role-specific execution responsibilities.
- `docs/execution/`: persistent progress, evidence and resumption state.
- `tests/`: reviewed reproducible test source and sanitized regression fixtures.
- `tests/acceptance/state-recovery/`: independent B1 root, execution, durable, corruption, snapshot and process-recovery acceptance.
- `local-tests/`: ignored machine-local experiments and test output; never published.

Complete public/master/validator node entry points and independent copy/build/run acceptance remain unfinished. Reference tools and component interfaces are pinned; current gate results and limitations are recorded in [execution status](docs/execution/STATUS.md). Follow [verification instructions](docs/development/verification.md) to provision the local reference tools and run a registered gate. These foundations are not a promise of 1M TPS. The execution component targets Shanghai; subsequent fork support is an explicit upgrade.

## Economics

Collected transaction fees: **40% burn / 30% node rewards / 30% validator rewards**. Integer rounding, uptime, verified work, staking, escrow and slashing are specified in the plan. No additional inflation or real-value genesis allocation is authorized by these documents. Fee redistribution differs from Ethereum's base-fee burn policy and must be advertised as an EVE difference.

## Start implementation

Read [AGENTS.md](AGENTS.md), [goal.md](goal.md), the [planning index](docs/plan/README.md), and [execution status](docs/execution/STATUS.md).

```text
/goal Implement goal.md end to end. Follow AGENTS.md and docs/plan/23-task-backlog-and-execution.md. Work in dependency-aware bulks, implement and test real functionality, maintain evidence and handoff files, and continue through all unblocked work. Never mark a target passed without its required evidence.
```

The goal file defines exactly what DONE, BLOCKED and TARGET_UNMET mean. Completing the documentation or a local prototype is not completing the whole project.

## Collaboration and publication

Read [CONTRIBUTING.md](CONTRIBUTING.md) before making changes. Shared source comments, documentation, configuration explanations, test descriptions, commit messages, and GitHub discussions must use English.

Keep GitHub clean: `local-tests/`, raw artifacts, build output, temporary databases, secrets, and personal configuration must never be committed or pushed. Required reproducible tests remain versioned so every collaborator can verify the implementation. Inspect the index and all outgoing commits before an authorized push; never force-add ignored local files.

## Scope boundaries

The first implementation is an independent devnet. Alephium bridging, settlement proofs, production keys, public mainnet launch, token issuance to real users, and paid infrastructure require separate explicit approval. No Alephium-inherited security is claimed. A public repository does not hide master source or secrets; source-distribution licensing and any private repository split are separate owner decisions.
