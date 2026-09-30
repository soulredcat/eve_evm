# Resumption handoff

## Current position

The project is documentation-only. The earlier implementation baseline inspected was commit `1d8b73c7eec223a1460c9e09cfb2d18a5627dee6`; this documentation update replaces exploratory contradictions with a goal and executable plan. Read actual current HEAD before beginning, since other work may have occurred after this file was written.

No project runtime, devnet, test or benchmark process was started by this planning task. No production keys or funds were used. No build results exist to inherit.

## Next work

Start B0, not another open-ended planning round:

1. Inspect branch, HEAD, dirty files, installed tools and sandbox permissions. Preserve unrelated edits.
2. Read root AGENTS.md/goal.md and plans 12–24, especially accepted decisions and bulk dependencies.
3. Select/pin compatible actual dependency/tool releases and run the minimal EVM/storage/ABCI spikes.
4. Freeze executable byte vectors, native ABI/gas schedule and consensus commitment-height mapping.
5. Create workspace and xtask verification/devnet/evidence contracts, run B0 gates and record actual evidence.
6. Continue B1 and subsequent ready bulks.

Initial inspection commands, where available:

```sh
git status --short
git log -1 --oneline
rustc --version
cargo --version
```

The repository may not yet contain Cargo.toml or xtask; their absence is B0 work, not an excuse to report cargo tests as passed. Inspect tool availability rather than assuming a particular operating system path.

## Decisions not to reverse

Validators decide finality; master follows and persists. No automatic master takeover. Separate runtime roots and independently buildable public packages. Durable validator signing/recovery records despite RAM working state. Authenticated snapshot roots with correct consensus height binding. Fee split 40/30/30. Keep the 1M aggregate finalized TPS target measured and unachieved until verified.

## Future handoff updates

Replace this next-action section with actual progress at each interruption: branch/worktree and HEAD; dirty-file summary; current bulk/tasks; passing/failing evidence; task-owned live processes; exact next command; blockers and resource requirements. Never claim automatic background continuation or hide a failing gate behind a completed checkbox.
