<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Specialist execution responsibilities

These are role instructions for the available coding-agent runtime, not a claim that five agents have already been started. The lead explicitly reads/assigns them. When parallel subagents are unavailable, perform the responsibilities sequentially with the same gates; do not invent tool calls or claim independent agents ran.

| Role | Primary responsibility | Instruction |
|---|---|---|
| 1 Lead / Orchestrator | Dependency graph, bulk scope, file ownership, acceptance, continuity | [01](01-lead-orchestrator.md) |
| 2 Protocol / EVM Engineer | Execution semantics, consensus adapter, commitments, system rules | [02](02-protocol-evm-engineer.md) |
| 3 State / Network / Performance Engineer | Durable storage, sync, RPC/P2P, scheduling, capacity | [03](03-state-network-performance.md) |
| 4 Correctness / Security Engineer | Independent tests, threat cases, faults, invariants | [04](04-correctness-security.md) |
| 5 Reviewer / Integrator | Integrated diff, gates, documentation, commits, evidence | [05](05-reviewer-integrator.md) |

The lead does not assign 'implement everything' to several agents. A task packet states the bulk/task IDs, input facts, owned paths, interface dependencies, forbidden changes, required tests and handoff artifacts. Shared manifests, lockfiles, codecs and consensus interfaces have one owner at a time.

A normal cycle is: inspect -> scope bulk -> freeze interfaces -> parallel independent implementation/tests -> integrate -> run full gate -> review -> commit -> update status/handoff -> next ready bulk.

No agent may weaken a safety invariant or acceptance workload to obtain a passing result. Mainnet/keys/spending/visibility/licensing gates remain owner-controlled. Root [AGENTS.md](../../AGENTS.md) and [goal.md](../../goal.md) apply to every role.
