<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Repository verification tools

`xtask` owns development gates, source structure inspection and reviewed evidence orchestration. It has no execution, validator signing, finality or private master authority. Its production source follows the same operation-file and physical-line rules it checks.

Failed groups report bounded registered test identifiers and reviewed fixed
categories. Unknown text, panic payloads, keys and private paths are never echoed.
Raw stdout/stderr stay in ignored local evidence. Reported failure identifiers are
diagnostic only; a failed command still fails the gate and contributes no accepted
group count. Missing failure metadata does not turn a failure into a pass.

Run the structure checker from the repository root:

```sh
cargo xtask check-structure --report local-tests/structure-report.json
cargo test -p xtask --locked --test structure
```

The versioned policy is `config/structure-policy.toml`. Exact generated/vendor exclusions, decomposition reviews, bounded size exceptions and external-trait adapter registrations require reviewed provenance. CLI reports contain per-file classification, complete physical-line counts, production operations, exclusions and violations; a hard violation returns a nonzero exit status.

The integration suite mirrors checker responsibilities under `tests/structure/`:

| Requirement | Regression coverage |
|---|---|
| T-L01 | A dependency-free Rust package uses meaningful modules deeper than three folders, runs its byte-encoding test and verifies its Cargo package offline. |
| T-L02 | Actual Rust AST parsing counts free, associated, default trait and named nested operations while ignoring comments, strings, foreign declarations and test-only helpers. |
| T-L03 | Complete 200/201/400/401/600/601-line files include blanks, comments and unterminated final lines; review, exception and hard-ceiling behavior is exercised. |
| T-L04 | Exact bounded policy registrations, expiry, stale entries, provenance, source digests and repository containment reject bypasses. Tests and generators remain first-party source. |
| T-L05 | Type, facade, entry, test and reviewed external-trait adapter categories are distinguished. Unknown expression/statement macros, hidden diagnostic blocks/branches/functions, production imports of test-category source, generic production directory segments, initializers and delegation behavior are rejected. Literal diagnostics and local pure-expression closures remain supported. |
| T-L06 | Current canonical execution source is copied byte-for-byte to a deeper role-owned component path and rechecked. Role fixtures build without master source. Missing/ignored local manifests, transitive master edges, patch/replace redirection, escaping Cargo target paths and explicit/conditional module-source imports are rejected. |

Fixtures use disposable Git repositories and isolated Cargo target directories. Cargo package/build commands are offline and require no funds, credentials or external service. Source-relocation comparison preserves the existing state-root, receipt, fee and transaction implementations; the normal `eve-evm`, `eve-crypto` and `eve-storage` regression suites must also pass in the integrated gate.

Source discovery includes tracked and new non-ignored paths and subtracts current Git deletions. Every compiled local module/target must be inventoried as a regular `.rs` source inside its owning package or role. Source edges cannot convert a test file into unchecked production behavior. Exact `cfg(test)` module boundaries retain test-only imports; conditional paths also check their ordinary fallback source. The reviewed generated `OUT_DIR` binding facade remains an exact digest-bound exception, with its handwritten generator checked normally.

The role fixtures exercise source and dependency rules, not a production node runtime. Complete standalone public/validator distribution, copied-role build/run and T-Q04 runtime acceptance remain B6 work until implemented and exercised. These tests do not certify finality, post-quantum security, devnet readiness or throughput.

Test groups may declare release = true for an explicitly optimized runtime profile.
The default stays test. One argv builder supplies the same profile/features to
ordinary and documentation discovery/execution; locked selection, complete inventory,
serial execution and capture settings remain mandatory. Group evidence records the
actual Cargo profile. Timed public persistence admission uses release with its
unchanged queue-age limits; debug late-admission refusal is recorded separately.