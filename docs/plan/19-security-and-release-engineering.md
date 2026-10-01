<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# 19 — Security and release engineering

## Q01 — Threat model

| Threat | Required behavior and test |
|---|---|
| Compromised master | Forged state/root/history rejected without the appropriate validator-history/commitment proof |
| Compromised public RPC | No validator/master secrets; malformed relays and fake query data cannot become consensus state |
| Minority Byzantine validator | Invalid proposals/equivocation handled under the stated BFT assumptions; no invented stronger guarantee |
| Eclipse/Sybil peers | Multiple routes, bounded connections/queues, authenticated history and no peer-count voting power |
| Data withholding | Obtain necessary execution data before voting; preserve durable recovery paths |
| Snapshot/delta corruption | Staging import, bounded decompression, content checks and authenticated roots |
| Signing-state rollback | Refuse unsafe signing; separate fencing and recovery procedure |
| Release-key compromise | Threshold release verification, scoped roles, audit trail and revocation/rotation procedure |
| Malicious dependencies/build inputs | Pin sources/digests, review updates, verify artifacts and isolate build credentials |
| RPC/VM resource attacks | Gas/size budgets, queue bounds, simulation isolation and overload shedding |
| Regional partition | No minority finality or master takeover; retained data and explicit recovery |
| Reward farming | Assigned work only, deduplication, funded pools and adversarial accounting tests |

No topology guarantees that attackers cannot reach the master through software vulnerabilities. Isolation, validation and least privilege are layers of risk reduction. More public nodes do not automatically create independent operators or Byzantine security.

## Q02 — Keys and privileges

Separate EVM transaction/staking keys, validator consensus keys, P2P identities, master transport credentials, release keys, management credentials and backup recovery secrets. Keys are never stored in Git, logs, test reports or container images. Redact environment values in command evidence.

Use engine-supported durable signing or an external signer with equivalent tested anti-equivocation guarantees. Hardware-backed signing can be supported later; do not claim an HSM exists without deployment evidence. Signer failover is not ordinary stateless service scaling.

Master initiates authenticated data synchronization where feasible. Limit inbound services, use independent management access, separate OS users and filesystem permissions, and deny public/validator access to master databases. Native system operations cannot invoke arbitrary host functions.

## Q03 — Supply-chain and dependency gate

Pin Rust toolchain, crate versions/lockfile, compiler/tooling, consensus binaries, container image digests and test corpora. B0 records selected versions and license/security checks. Do not use mutable `latest` tags for reproducibility. Do not run downloaded shell installers or scripts without reviewing their provenance and effect.

Add dependency advisory/license checks and secret scanning to CI where supported. Report unavailable scans as NOT_RUN, not passed. Security issues in consensus, signing, execution, proofs or recovery block release until resolved or explicitly assessed with evidence; an agent must not suppress the check to finish the goal.

## Q04 — Public distribution boundary

Build public and validator source/binary packages from an explicit allowlist of runtime and shared crate paths. Exclude master implementation, dev secrets, local configuration and sensitive logs. A public-only source package must build in a clean environment with no access to the original monorepo's master directory.

Workspace manifests/lockfiles for the package must be generated reproducibly and tested; merely running `cargo build -p eve-public` in a complete private tree is not proof of independent distribution. Preserve applicable third-party notices. Do not choose a binding project license or change repository visibility without owner approval.

The owner requires copying the complete public or validator role directory alone to an unrelated clean location and building/running it there. Include all required local dependency sources/artifacts, lockfile/toolchain, sanitized examples and notices; no path may escape to the original repository or sibling roots. A master-host bundle may explicitly compose all roles with separate credentials/authority. Reusable component copies must match one canonical owner; do not maintain divergent consensus-critical implementations. T-Q04/T-L06 and clean-copy smoke tests must exercise actual role behavior, not just a compiling empty entry point.

The current repository is public. Putting code under `master/` does not make it private. The source-publication boundary is a packaging goal, not secrecy already achieved.

## Q05 — Authenticated releases

Design a signed manifest containing version, source commit, platform/architecture, binary digest, artifact size, protocol compatibility range, minimum/maximum schema support, migration requirements, expiry and monotonic release counter. Initial development release policy uses two-of-three independently generated release keys; keys remain in ignored local test storage.

Verify manifest threshold signatures, platform, digest, size, expiry and rollback counter before staging an artifact. A hash without an authenticated manifest is insufficient. The operator explicitly enables any auto-update policy. Master sync messages never carry executable installation authority.

Roll out by voting power and role redundancy, keeping the required consensus power online. Drain RPC, stop signing safely, preserve signing/application data, apply compatible migration, restart, catch up and verify readiness before the next batch. Binary rollback must never restore stale signing state or incompatible database schemas.

Production release trust roots, key ceremony and independent review require owner-approved operational procedures. Do not claim a local signed test release is a production release.

## Q06 — Logging and operations

Structured logs include role, network, heights, block/hash identifiers, operation, duration and stable error code. Metrics include queue depth/age, invalid messages, execution conflicts, consensus round, durable height, sync lag, retained bytes, compaction stalls and RPC limits. Avoid high-cardinality full-address/transaction labels in metrics; detailed IDs belong in sampled logs/traces.

Expose private metrics/health endpoints without admin powers. Separate alive, ready and signing-enabled indicators. Alert on quorum loss, signer uncertainty, root mismatch, growing persistence backlog, expiring trust anchor, disk pressure and failed release verification.

## Acceptance

T-Q01: compromised-source fixtures cannot inject state or invoke privileged APIs.
T-Q02: secret scan and privilege-boundary tests cover every package/config artifact.
T-Q03: malformed codec/proof/RPC inputs and decompression bombs remain bounded under fuzzing.
T-Q04: public/validator packages build and run without master source access.
T-Q05: unsigned, corrupted, expired, wrong-platform and rollback manifests are rejected.
T-Q06: rolling upgrade preserves quorum and never reuses a stale signing snapshot.
T-Q07: dependency/source pins reproduce the intended build inputs; unavailable scans are explicit.
T-Q08: logs/metrics distinguish stale/read-only operation from healthy finality without leaking secrets.
