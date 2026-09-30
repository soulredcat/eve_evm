# 25 — Recursive folder hierarchy, function files and size limits

Status: mandatory implementation policy, added 2026-09-30. The structure command, T-L01–T-L06 coverage and CI integration are implemented and pass complete local B0/B1 gates. Hosted results remain separately recorded in the execution evidence; standalone package gates remain unfinished.

## F01 — Owner requirement

Use clear folders, subfolders, further subfolders and leaf files for individual functions. There is no fixed maximum nesting depth and no rule limiting the project to three directory levels. Keep `master/`, `public/` and `validator/` separate, and share reusable behavior through the domain crates in plan 22.

This policy refines plans 22 and 23 and applies to every implementation bulk. It does not change protocol semantics, consensus authority, fee allocation or throughput acceptance.

The owner explicitly elevates role clarity and file placement to an absolute integration rule ("level 1000"). Every file must belong to a named role/domain and responsibility. Keep public/master/validator-specific orchestration in its runtime root; reusable domain behavior belongs in a specifically named crate. Forbidden role/dependency edges, mixed-role modules, opaque splits and hidden facade/adapter behavior block integration even when compilation or a benchmark succeeds. Semantic ownership review complements automated coverage; never claim an unimplemented package/structure check passed.

## F02 — Organize by domain, then narrow the responsibility

The recurring pattern is:

```text
runtime or crate/
  src/
    domain/
      capability/
        sub-capability/
          operation/
            further meaningful subdivisions as needed/
              descriptive_function_name.rs
```

Every level must narrow meaning or establish an ownership/API boundary. Group related function files under the smallest coherent parent. Add another subfolder when a domain develops distinct sub-responsibilities; do not flatten everything into one directory to avoid nesting. Conversely, do not manufacture empty layers, duplicate names or placeholder directories to reach a depth target. Depth follows real responsibility, not a number.

Do not create a root `crates/` or `create/` directory. Reusable Rust packages remain named, responsibility-specific components under an explicit owning role. Copy-ready role distributions carry reproducibly packaged dependency components and standalone manifests/lockfiles; package copies must match canonical source and cannot introduce private-master dependencies or unresolved paths outside the copied directory.

Use `snake_case` Rust module and function-file names. Keep established runtime/crate root names from plan 22. Prefer domain names for folders and verb-plus-object names for behavioral files. Do not create catch-all `utils.rs`, `helpers.rs`, `common.rs`, `manager.rs` or a generic `shared` crate for unrelated logic. Do not split a large file into `part1.rs`, `part2.rs` or `misc/`; split by actual function.

Example future paths, not a claim that these runtime implementations exist:

```text
master/
  src/
    main.rs
    sync/
      finalized/
        batches/
          import/
            import_finalized_batch.rs
        checkpoints/
          recovery/
            restore_finalized_checkpoint.rs
public/
  src/
    main.rs
    ingress/
      transactions/
        submission/
          relay/
            forward_signed_transaction.rs
validator/
  src/
    main.rs
    consensus/
      proposals/
        preparation/
          prepare_proposal.rs
      voting/
        prevote/
          verification/
            verify_prevote_context.rs
        precommit/
          signing/
            request_durable_precommit.rs
validator/
  components/
    state/
      src/
        commitments/
          accounts/
            proofs/
              verification/
                verify_account_proof.rs
```

These files call the existing shared contracts. They must not duplicate an EVM engine, consensus algorithm or storage implementation in each runtime. Module declarations and narrow re-exports must make the nested paths buildable. Keep internal functions private or crate-visible unless an actual consumer needs a public API.

## F03 — One behavioral file = one primary function

Each handwritten production behavior file owns one named operation and one primary nontrivial function, whether free-standing or an associated method. Name the file after that operation. Other named executable helpers with their own behavior belong in separate, appropriately nested files, even if private. Several unrelated functions in a file are prohibited even when the total is below 200 lines.

For example, use separate `decode_transaction.rs`, `verify_signature.rs`, `check_nonce.rs` and `reserve_balance.rs` files under their relevant capability folders; do not hide all four inside `process.rs`. A coordinating function may call these operations and own their sequencing, but may not inline their implementations into a giant orchestrator.

A small inline closure implementing one local expression does not need its own file. A long closure, macro or nested function that hides another operation is not an exception. Avoid duplicating logic or introducing unnecessary runtime dispatch just to split files.

Non-behavioral file categories are explicit:

| Kind | Allowed responsibility |
|---|---|
| Type/trait declaration | One cohesive type, contract or closely bound type family; no unrelated behavior implementations. |
| `mod.rs` / crate facade | Declarations, narrow exports and module documentation; no domain execution logic. |
| `main.rs` | One thin entry function calling configuration/bootstrap/shutdown components. |
| External trait adapter | A cohesive trait implementation whose methods only delegate to single-function files; any multi-method adapter must be registered and reviewed, remain at most 200 lines, and contain no hidden business logic. |
| Tests | A function-focused test file may contain multiple test cases and test-only setup; it must not become a second production implementation. |
| Docs/configuration | A cohesive document/configuration unit, subject to the applicable file-size ceiling but not a function-count rule. |

Where Rust trait/privacy/visibility constraints require an adapter, keep the exception narrow and preserve compile-tested interfaces. Do not declare ordinary multi-operation service files to be adapters. Reviewed adapter declarations record exact path, kind, reason and owner; they cannot bypass the 600-line hard limit.

## F04 — 200 / 400 / 600 physical-line policy

Count the complete formatted file, including imports, blank lines, comments, declarations and tests. Count an unterminated last line as a line. Do not use minification, removed comments, long packed expressions or formatter suppression to pass a line limit.

| Physical lines | Policy |
|---|---|
| 0–200 | Normal target. A behavioral file still has only one primary function. |
| 201–400 | Review threshold. The reviewer must attempt meaningful decomposition and record why any retained larger file remains one responsibility. CI reports a warning, not silent acceptance. |
| 401–600 | Exceptional ceiling, never the normal target. CI fails unless a narrow, reviewed, unexpired exact-path exception exists. Each retained file has a concrete split task due no later than the next bulk. |
| 601 or more | Hard failure for handwritten source, tests and maintained documentation/configuration. Split before integrating. A size exception cannot override this ceiling. |

New behavioral files should stay within 200 lines and must not deliberately grow toward the exceptional ceiling. A file with one function but excessive length still needs extraction into smaller operations. The reviewer/integrator can approve a bounded 401–600-line exception inside the current development scope; this does not permit permanent exceptions or a change to user requirements.

Size exceptions must identify exact path, current line count, rationale, reviewer, related tests, split task and expiry bulk. A moved file needs review of the new path. Wildcards exempting entire runtime, test or documentation trees are prohibited. Expired, missing or malformed entries fail the gate.

Generated lockfiles, vendored upstream fixtures and machine-generated bindings may use an explicit exclusion manifest recording generator/source and why the file is not handwritten. Keep them in identifiable locations. Manual production code, generated wrappers used to conceal business logic, and an arbitrary `generated/` folder are not valid exclusions. Generated output must be reproducible and its handwritten generator remains subject to this policy.

## F05 — Tests, visibility and ownership

Test files mirror the operation/capability they test; multiple positive and negative test cases may share that function's test file. Split growing suites by scenario rather than appending thousands of lines. Small inline tests are permitted only within the same operation's file budget; prefer separate test modules as the suite grows.

Do not expose private production state solely to ease a test. Use existing contracts and restricted test support. Keep the serial execution oracle, root/receipt comparisons and crash-recovery gates intact during layout changes.

The lead assigns ownership at capability/subfolder level. The integrator owns shared manifests, module exports and cross-crate interface changes unless explicitly delegated. Folder separation is not permission to create cyclic dependencies, duplicate implementations or diverging serialization rules.

## F06 — Automated structure gate: implement in B0

Complete `cargo xtask check-structure` during B0 and wire it into every bulk verification command and CI. The initial command is available; its repository scan alone does not close the boundary/regression tests, package checks or CI requirements below. B0 cannot pass while required checker coverage is absent, stubbed or silently skipped.

The checker must:

1. Recursively inspect tracked first-party source, tests and maintained documents/configuration, plus newly created files in a working tree; use explicit generated/vendor exclusions, not an extension-only loophole.
2. Calculate physical lines and enforce all thresholds, warning explanations, exact-path exceptions and expiry rules.
3. Use language-aware parsing for Rust production functions, including associated methods and named nested helpers. Do not count `fn` substrings in comments or strings. Add equivalent coverage before claiming another handwritten language is checked; report unsupported source categories instead of silently passing them.
4. Identify behavioral, declaration, facade, entry, test and reviewed adapter files. Report multiple production operations, domain behavior in a facade, opaque numbered splits, and stale exception entries. Automated classification is backed by semantic review; a parser alone does not prove single responsibility.
5. Emit human-readable failures and a machine-readable report containing path, kind, lines, operation count, exclusions, exceptions and violations. Return nonzero on a hard violation or missing required coverage.
6. Run on its own handwritten implementation. Do not exempt `xtask/`, tests or generators to hide a large checker.

Persist the policy as versioned checker configuration during B0. Policy/manifest changes require an explicit reviewed decision, not a local config switch to make a bulk pass.

## F07 — Required checker and refactor tests

| ID | Required evidence |
|---|---|
| T-L01 | Nested modules beyond three subfolder levels build, test and package without duplicate logic or a fixed depth ceiling. |
| T-L02 | One production operation passes; two separate operations fail; comments/strings do not create false function counts; associated/nested functions are checked. |
| T-L03 | Boundary fixtures for 200, 201, 400, 401, 600 and 601 physical lines exercise warning, exception and hard-failure behavior. Include blank lines and a final line without a newline. |
| T-L04 | Valid bounded exception passes; expired/missing/wildcard exception fails; generated/vendor exclusions cannot hide first-party source. |
| T-L05 | Thin registered adapter, type/facade and test files are classified correctly; a facade or adapter containing unrelated logic is rejected by checker/review. |
| T-L06 | Refactoring preserves public/master/validator package independence, deterministic roots/receipts and relevant protocol/recovery tests. |

## F08 — Integration and existing code

Inspect actual code before each bulk; do not assume the repository remains documentation-only. Put new work directly in the appropriate hierarchy. Refactor impacted oversized or multi-operation files as part of the same verified bulk rather than accumulating a cleanup phase. A full-repository layout change still requires explicit file ownership and regression gates; do not overwrite unrelated work.

Every bulk records its structure report, warnings with rationale, active exceptions and completed split tasks alongside the existing correctness evidence. A folder tree in a README, a passing line count alone or a collection of empty one-function files is not completion. Implement the actual functionality and run the required gates.
