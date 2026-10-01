<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Validator runtime

The runtime implements the classical Linux development consensus path. Complete
B3 verification passes 518 cases, including all 16 actual consensus acceptance
cases and 99 validator cases. This single-host result is not production security,
post-quantum activation, standalone distribution or secured-throughput acceptance.

Role ownership and standalone distribution are absolute requirements. Components own deterministic execution, logical state, authentication, pinned consensus and immutable EVE genesis/native/header contracts. Validator signing/consensus wiring stays here; public readiness/source/persistence stays public-owned. A copy-ready validator must contain all dependencies, locks/toolchain, sanitized configuration and notices and build/run after copying this directory alone. Private master implementation and unresolved outside paths are forbidden. Standalone copied distribution remains B6 work and is not established by a monorepo package build. External programs are deferred until testnet under D40.

Owns transaction execution/replay, proposal validation, consensus participation, finality and validator lifecycle integration. Uses shared EVM/protocol/state modules and a reviewed BFT adapter; no private master database dependency.

Mandatory durable data: consensus WAL, anti-double-sign height/round/step and sign-byte history, key-fencing state, recent finalized block data and recoverable application state. Hot RAM does not replace these records.

A four-validator equal-power devnet needs three votes for a commit under the selected more-than-two-thirds rule. Loss of quorum stops finality; master never takes over. More validators do not automatically increase TPS because replicas verify the same ordered workload.

The initial one-master/one-public topology retains these four distinct validators. A single-validator or all-in-one harness is local-only development, with no distributed-consensus acceptance claim. Later two-master mutual replication and ten-master regional placement do not change validator quorum, signing authority or mandatory data availability.

Carry `zone_id` as operational routing/failure-domain metadata from the start, separately from network/genesis identity and EVM chain ID. Zone membership does not authorize independent state writes or finality. Transaction propagation, execution validation before voting and live finalized-block P2P distribution remain independent of master availability or public bulk-sync endpoint selection.

Public nodes have RAM working state plus durable verified block/checkpoint recovery data by default. Their persistence policy cannot replace validator WAL, anti-double-sign records or recent independently recoverable state/data. An authenticated master transport identity is never a substitute for validator history and finality proofs.

Read plans 12–17 and 20–22, plus 32 (local-only: `../docs/plan/32-regional-masters-and-public-persistence.md`). A co-located public RPC process does not receive consensus keys. Never clone live signing keys into an active standby.

`eve-validator init-dev` initializes one private development namespace and prints
public identity metadata. `serve-dev` runs the application/signer actors and the
owned pinned engine in the foreground, with bounded shutdown on SIGTERM/Ctrl-C.
Both require `--genesis`, `--data`, `--signing-seed`, `--comet-binary`,
`--comet-sha256` and `--acknowledge-unsafe-development`. The seed file is exactly
32 binary bytes, owned by the current Linux user with mode 0600; the data parent
is a private Linux directory. Do not place either in Git or use production keys.
The expected engine digest comes from the verified task tool receipt. Native
init's generated dummy signer is outside the active roster and receives no EVE
signing secret. The EVE signer synchronizes complete anti-double-sign history
before releasing each new signature.

RPC/P2P defaults bind loopback. `--rpc-address`, `--p2p-address`,
`--persistent-peers`, optional loopback `--advertised-p2p-address` and `--zone-id`
configure the disposable topology. An advertised proxy endpoint supports honest
fault injection; zone/endpoint configuration grants no voting power. Readiness
requires actual authenticated native handshakes and safe recovered application/
signer state. Its metadata alone does not prove finality; verify the real native
commit certificate and the appropriate next-height application anchor.

The [runtime](src/consensus/runtime/README.md), [application](src/consensus/application/README.md),
[signer](src/consensus/signing/README.md), [approval](src/consensus/approval/README.md)
and [engine](src/development/engine/README.md) documents define ownership and limits.
[Temporary acceptance adapters](src/development/acceptance/README.md) require the
explicit `development-acceptance` feature and a genesis-bound manifest; normal
builds reject that mode. B5 replaces this test-only lifecycle path. No master,
remote chain or production signing/custody authority is introduced.
