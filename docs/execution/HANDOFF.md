# Resumption handoff

## Current position

The project is documentation-only. The collaboration-policy task started on clean branch `main` at commit `027ef9dc5779098f1192c56308cf7eb6c21bd13e`. Read actual current HEAD before beginning; the policy changes are recorded in a later local commit.

No project runtime, devnet, test or benchmark process was started by this planning task. No production keys or funds were used. No build results exist to inherit.

The owner requires clean GitHub publication and English shared prose. Read `CONTRIBUTING.md` and the corresponding rules in `AGENTS.md`. Root `local-tests/` is ignored machine-local storage and must never be committed or pushed; recreate it locally when absent in a fresh checkout. Required reproducible tests and sanitized fixtures remain tracked. No push is authorized by this policy change.

## Next work

Start B0, not another open-ended planning round:

1. Inspect branch, HEAD, dirty files, installed tools and sandbox permissions. Preserve unrelated edits.
2. Read root AGENTS.md/goal.md/CONTRIBUTING.md and plans 12–31, especially accepted decisions, bulk dependencies, security/interoperability and current scope.
3. Select/pin compatible actual dependency/tool releases and run the minimal EVM/storage/ABCI spikes.
4. Freeze executable byte vectors, native ABI/gas schedule and consensus commitment-height mapping.
5. Create workspace and xtask verification/devnet/evidence contracts, run B0 gates and record actual evidence.
6. Continue B1 and subsequent ready bulks.

Initial inspection commands, where available:

```sh
git status --short
git log -1 --oneline
git check-ignore -v -- local-tests/README.md
git ls-files -- local-tests/ artifacts/ coverage/
rustc --version
cargo --version
```

The repository may not yet contain Cargo.toml or xtask; their absence is B0 work, not an excuse to report cargo tests as passed. Inspect tool availability rather than assuming a particular operating system path.

The exact next command is `git status --short --branch`; then inspect the toolchain and execute B0 including SEC0 and INT0. No task-owned background processes remain. Publication-policy verification is recorded separately in `EVIDENCE.md`; runtime and structure gates remain `NOT_RUN` / `NOT_IMPLEMENTED`.

## Decisions not to reverse

Validators decide finality; master follows and persists. No automatic master takeover. Separate runtime roots and independently buildable public packages. Durable validator signing/recovery records despite RAM working state. Authenticated snapshot roots with correct consensus height binding. Fee split 40/30/30. Keep the 1M aggregate finalized TPS target measured and unachieved until verified.

## Future handoff updates

Replace this next-action section with actual progress at each interruption: branch/worktree and HEAD; dirty-file summary; current bulk/tasks; passing/failing evidence; task-owned live processes; exact next command; blockers and resource requirements. Never claim automatic background continuation or hide a failing gate behind a completed checkbox.
