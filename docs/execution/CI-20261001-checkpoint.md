# Application checkpoint observation repair — 2026-10-01

The latest failed hosted consensus command at
[1fe643e](https://github.com/soulredcat/eve_evm/actions/runs/36780831271)
reported checkpoint context after roughly three seconds. Its raw child output
was not published, so the historical error is not independently recoverable.

The pinned Comet source establishes a concrete race: status reads BlockStore
height, while consensus saves that block before application execution and ABCI
Commit. The lifecycle fixture observed RPC height 6, stopped the engine and then
required a synced application checkpoint at least 6. RPC height alone cannot
satisfy that requirement.

An isolated actual-engine reproduction uses unchanged source identity and
inserts a test-only, interruptible 750 ms pause after the FinalizeBlock 6 response,
outside the state mutex, before reading Commit. The control passes. The fault
reports RPC height 6, committed application height 5 and pending height 6; the
unchanged checkpoint assertion fails after 3.08 seconds. This reproduces the
failure category and timing without proving the discarded hosted error text.

The repair waits for the separately synced application marker before shutdown
and after restart. The marker advances only after write, sync, close and rename.
Bounded try-lock polling respects the deadline during storage work and rejects
poisoned locks. The original minimum checkpoint and restart assertions remain.
Two tracked regressions distinguish a missing marker from delayed publication;
fixed diagnostics identify the exact checkpoint condition without raw output.

The identical injected fault passes after repair: four cases, none failed,
ignored or filtered, exit 0, 4.49 seconds. Scoped tooling executes 89 cases with
exit 0 after supplying the required pinned Go environment. An initial standalone
tooling attempt without that environment failed and was retained locally.

Local-only evidence: local-tests/ci-checkpoint/repro-20261001/REPORT.md, SHA256
c9cb8bdaad2cb7366e62557f57c08ccbf3ec5a02c583f91fbfb3c5053c00ffe6.
Control/fault/repaired commands, exits, source/fault identities and output hashes
are recorded there. Diagnostic control/fault selection is not a whole-gate pass.
The isolated clone is clean, engine processes stopped and fixture threads joined.
Raw generated keys, logs and databases remain ignored and unavailable on GitHub.

Integrated `cargo xtask verify --bulk B1`: exit 0, 273 executed cases, zero failed,
ignored or filtered, format, strict workspace lint and release build pass.
Structure covers 792 files with zero warnings/violations. Base HEAD is b0b9efa;
tested dirty bundle SHA256 is
684bc0aae9526976e4dcd97508dcbf2ec45a55548e33ce233e2254b944caf830.
Local-only run: local-tests/verify-202-1790809936653868201.
Report SHA256: aab23bea6f78d3caea06db58311537118ff141f86349c068cd6cf7b9f564677e.
Structure SHA256: 7e46480034150c17b7fc3a945d903edee834f296734dfdb1735acd70908588fa.
Final publication prose gets separate structure/index checks; code/config stay
identical to this passing gate.

Reproduce with pinned provisioning and `cargo xtask verify --bulk B1`. Hosted
acceptance must be observed separately; local component results do not establish
validator finality, signing durability or production network acceptance.
