<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Owned development engine process

This subtree owns verified native engine initialization, configuration, process
ownership and termination for the explicit unsafe classical four-validator
development composition. Consensus signing, application execution, validator
enrollment, callback dispatch and topology acceptance remain with their owners.
The supported reference is Linux 5.4+ with real current-UID 0700 directories,
pidfds and private Unix sockets. Unsupported facilities fail closed. NTFS mode
emulation is not evidence of Linux access control or storage durability.

The private API has two initialization/launch steps:

1. `initialize_engine_home(config, home, native_public_genesis, expected_sha256)`
   verifies the trusted launch digest, actual binary bytes and exact native
   version/source revision before running the pinned binary's real `init` command.
2. `start_engine(config, home, application_socket, signer_socket, expected_sha256)`
   verifies the recognized home, writes bounded operational configuration and
   launches the real binary. `engine_node_id` uses its actual `show-node-id` command.

The caller supplies the digest from its verified provisioning receipt. Matching
version text alone cannot authorize execution. Binary hashing uses a fixed 16 KiB
buffer and a 512 MiB file/work cap; executable bytes are checked again after the
version probe. The sealed `VerifiedEngineImage` retains that descriptor and exact
device/inode/length/mtime/ctime identity. The launched `/proc/PID/exe` must match
immediately, without a post-launch byte scan that delays native signer connection.
A vfork-style spawn can resume the launcher before the kernel installs the child's
new address space, so the launched process may briefly still show the launcher's
own executable. `start_engine` alone polls for at most two seconds while it shows
exactly that image, then applies the unchanged strict binding; a persistent caller
image fails as `ENGINE_PROCESS_EXEC_INCOMPLETE` and any other image still fails.
Every authenticated channel binds to this same prehashed image; changed identity
fails closed. This is a local launch
binding, not in-memory attestation or protection from a privileged or trusted local
operator. The current pinned binary reports
`0.39.0+0880b4d378f347ab16e54ec677ff50d803f37d62`.

`home/` creates only a new immediate child of the validated task data directory.
An OS exclusive lease protects this namespace across initialization, configuration
and the child lifetime. Its persistent empty lock file is never deleted to bypass
a lease. Release explicitly unlocks the open file description before closing it,
including error returns; a retained duplicate handle cannot prolong the old lock.
This is single-host ownership, not a distributed fencing service.

Initialization writes an `initializing` receipt, performs native initialization,
installs caller-supplied public genesis bytes and binds hashes of genesis,
configuration, native node key and generated dummy native PV key in a `ready`
receipt. Writes use exclusive temporary files, file sync, rename and directory
sync. Existing incomplete, foreign, changed or missing prerequisites reject
without reset, cleanup or key regeneration. Recognized reopening requires exact
immutable genesis and identity. Operational configuration changes require the
previous configuration digest and atomically update their receipt; an interrupted
configuration/receipt pair fails closed. These sync boundaries do not prove
resistance to coherent copied-home rollback or every filesystem/power-loss model.

Native `init` generates its own node key and dummy PV files. The dummy PV address
must not be in the supplied EVE validator roster. The real EVE signing seed is
never read, passed as an argument or copied into the native home. The configuration
always selects the external Unix signer, whose retained anti-double-sign state
belongs to the signing/storage domains. Existing signer/application stores are
never repurposed by this module.

`configuration/` rechecks the canonical frozen public genesis contract: four
classical validators, initial height one, `eve-local-v1`, 4 MiB block bytes,
30 million block gas, evidence age 1000 blocks/24 hours and 1 MiB evidence bytes.
Canonical validator admission and cryptographic genesis validation remain in the
protocol/application domains. RPC and P2P bind distinct nonzero loopback ports;
at most 16 unique loopback persistent peers are accepted. PEX/seeds are disabled,
duplicate local IPs are allowed, unsafe RPC/GRPC/pprof are disabled, and RPC,
mempool and transaction limits are finite.

The EVE application listener exists before launch and Comet dials `proxy_app`.
Comet creates and listens on `priv_validator_laddr`; EVE's external signer dials
that private endpoint. The signer socket must be absent before launch. Both paths
are bounded absolute immediate children of the same owned data directory. Foreign
files, directories and symlinks reject.

`OwnedEngine` privately retains the actual `std::process::Child`, its original
pidfd, sealed held image, the lease and private diagnostic paths. `engine_child` provides the tracked
child to the supervisor for short lifecycle/authentication checks, never across
network I/O. `engine_authentication_context` provides disjoint child/image borrows
for immediate peer authentication under that short supervisor lock. It exposes no
image File or constructor. Native diagnostic output stays in 0600 task-owned files; API failures
report safe categories. Probe commands are bounded to 15 seconds and 64 KiB per
output. Long-lived diagnostic backlog is checked at lifecycle polls with a 64 MiB
per-file limit; this is polling containment, not a hard kernel disk-write quota.

`stop_engine` signals the original pidfd, waits up to five seconds, then uses the
tracked Child's kill/wait fallback. It terminates only this module's child. After
confirmed termination and under the held lease, it may remove only the designated
still-owned signer Unix socket that was absent before launch. It preserves a
foreign replacement and every application socket, key, database and state path.
No untracked orphan is killed; native database locks can reject restart. The thin
Drop adapter delegates this same stop operation as best-effort cleanup.

Dedicated tests require the actual pinned Comet binary and digest through
`EVE_COMET` and `EVE_COMET_SHA256`; missing prerequisites fail rather than skip.
They exercise actual version/init/node-ID, recognized reopen, digest/version
spoof rejection before execution, malformed input, OS exclusivity, duplicate-lock
release, actual launch/abrupt termination and foreign-file preservation. Test
directories and native generated keys are disposable private Linux data. These
cases do not substitute for the separate four-validator network acceptance run.

Run `cargo test --locked -p eve-validator --lib development::engine` with the
pinned native build environment and actual tool receipt. Keep raw outputs and
machine-local runtime files under ignored `local-tests/` or private temporary
test directories; publish only reviewed commands and evidence summaries.
