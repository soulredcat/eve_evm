<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Private local-engine transport

The supported reference is Linux 5.4+ with private Unix sockets. Unsupported
platform/pidfd facilities fail closed. This transport authenticates a task-owned
local engine process; it is not public P2P authentication, validator enrollment,
source finality, post-quantum protection or a global/operator compromise defense.

`authenticate_engine_peer` consumes one real established UnixStream, the active
tracked Child and a sealed `VerifiedEngineImage` from the development engine owner.
That owner hashes the actual held executable descriptor before launch with a fixed
16 KiB buffer and 512 MiB file/work cap, checking the trusted digest and exact native
version. Kernel SO_PEERCRED must match the Child's exact PID and current effective
UID/GID. The held verified descriptor and running /proc/PID/exe must have identical
device/inode/length/mtime/ctime identities. No full byte scan delays signer startup
or repeats per native channel. The child is polled before/after this binding.
A pidfd binds the original process independently
of later PID reuse; nonblocking waitid uses NOWAIT and never reaps the supervisor's
child. Missing/exited/reaped children cannot authorize saved capabilities.

The immutable `AuthenticatedEnginePeer` owns that socket and pidfd. Its fields
are private; no boolean/metadata constructor, Clone, deserializer, raw descriptor
or stream accessor exists. A sanitized PID accessor is available only to tests.
`ensure_application_engine_peer` checks the application channel, original live
process, current effective identity and executable binding. Changed
device/inode/size/mtime/ctime fails closed, even when byte content is unchanged.
These checks do not attest in-memory code or protect against privileged or trusted
local operator attacks, a compromised engine, inherited/copied keys or full backups.

The supervisor owns Child. Hold its mutex only for authentication/lifecycle polls,
release it before blocking framed reads/writes, and poll the same Child again
after I/O and before dispatching business operations. Private callback wrappers
also require the sealed peer and application check. Root's proposal issuer accepts
that proof plus the request actually decoded from the same FD; transport never
fabricates a consensus header or implements signing/execution policy.

`framing/` uses the one canonical Comet uvarint/prost codec. The application body
cap is 4 MiB plus 64 KiB metadata; the signer body cap is 64 KiB. These are encoded
body limits, not total decoded-object/OS-memory guarantees. Prefix overflow,
overlong/truncated lengths and oversize reject before body allocation. Truncated
bodies reject within the checked allocation bound. Writes check encoded length
before serialization. Protobuf decode
errors expose a fixed safe category; packet/signing/private-key bytes are never
included in diagnostics.

Each read/write has one absolute Instant deadline, at most 60 seconds. Thin
registered adapters update the remaining socket timeout before every syscall,
so partial progress cannot restart the deadline. Stream decode/write faults close
that FD instead of attempting unsafe resynchronization. Connection supervision,
state readiness, signer recovery and cancellation policy remain with their owners.

Idle native query/snapshot connections are polled with `engine_peer_read_ready`
before starting a whole frame. Kernel PEEK/DONTWAIT never consumes the prefix;
false is only idle I/O readiness, never an authentication assertion. Polls are
bounded to one second and leave healthy idle streams open. The close-only
`EnginePeerShutdown` capability is created only from a live sealed peer and can
wake a blocked worker; it exposes no raw descriptor, packet I/O or proof constructor.

Production listeners must live in task-owned 0700 directories. Peer tests use real
child processes, accepted sockets, image checks, pidfds and a private directory;
their test executable is not a claim of complete Comet/network acceptance. Fixtures
copy the running test image into their private directory before hashing/launch,
so concurrent build replacement cannot change an authenticated worker image.
Fixtures kill/wait only their own children, including failure paths. Native ABCI and privval
typed framing, wrong-process/digest/channel, saved-proof exit, prefix/body bounds
and slow-progress deadlines have dedicated cases. No simulated metadata yields a
production peer capability.
