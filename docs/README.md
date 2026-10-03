<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# EVE documentation

This directory contains the shared contributor context available from a GitHub
clone. Read the following documents before proposing an implementation change.

| Document | Responsibility |
|---|---|
| [Architecture](architecture/README.md) | Runtime ownership, authority, persistence and topology contracts |
| [Capabilities](capabilities/README.md) | Implemented engineering capabilities, partial slices and unachieved release targets |
| [Development](development/README.md) | Contribution rules, file organization, verification and publication |
| [Testing](testing/README.md) | Reproducible test ownership, gate commands, fault scope and evidence |
| [Performance](performance/README.md) | Workloads, finalized TPS, latency, persistence and target acceptance |
| [Execution status](execution/README.md) | Verified checkpoint, failed hosted evidence and the next repair |
| [Core roadmap](plan/README.md) | Dependency order, remaining requirements and deferred scope |
| [Security inventory](security/inventory/README.md) | Core authentication coverage, assumptions and required security gates |
| [Specialist responsibilities](agents/README.md) | Scoped agent ownership and integration review responsibilities |

The execution status is the public authority for measured progress. Architecture
and roadmap requirements describe intended behavior; they do not claim that an
unfinished feature exists. Component READMEs describe their actual interfaces.
Current owner instructions and the permission terms in [LICENSE](../LICENSE)
remain binding.

Only Markdown files named `README.md` are published. `AGENTS.md`, `goal.md`,
`CONTRIBUTING.md`, detailed plans and detailed execution records remain local in
their existing paths. They are not prerequisites for reading this public context
and must never be force-added. Raw output, credentials, disposable databases,
machine configuration and exploratory scripts remain in ignored local storage.

Update the affected public contract and execution summary with implementation
changes. Bind reported test results to the exact tested revision and distinguish
local verification, hosted verification and unfinished acceptance requirements.
