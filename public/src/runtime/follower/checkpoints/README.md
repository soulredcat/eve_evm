<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Public development checkpoint bootstrap

This module orchestrates bounded own-chain source downloads and public-owned
checkpoint staging before RPC or normal follower admission starts. It delegates
all wire/native decoding to the canonical sync client, immutable file integrity
to the recovery store and finality/activation to the charged public APIs.

The only filesystem names it generates are `.checkpoints`, `content` and `proofs`
beneath the contained ignored development data directory. Linux directory handles
check local ownership, 0700 permissions and no symlink following. Source data
selects no paths, trust policy, validator authority or master implementation.

Actual owner pool leases cover every client response, local genesis preparation,
target encoding and proof reference/manifest allocation. Downloaded wire objects
retain their client leases until dropped. Only one witness wire survives at a time;
the first proof pass retains bounded references, not a full history buffer.

Content chunks and second-pass witnesses are bound to immutable first-pass IDs,
lengths and checksums. Intact staging is reused; explicitly rechecked invalid
unpublished chunk/witness entries use the existing safe repair operations.
Before either transfer begins, actual charged canonical probes inspect both
pending metadata entries. Only explicitly recognized invalid unpublished bytes
can be repaired. Missing or identical valid entries preserve their meaning;
unknown, oversized, future or foreign data refuse bootstrap without mutation.
Complete content and proof checksums remain untrusted until full canonical
genesis proof verification, real base sync and conditional publication succeed.

The shared 300-second deadline reaches the client's absolute-deadline APIs and is
checked after download, materialization, sync phases and before base submission.
Actual kernel IO may block independently; the runtime does not fabricate durable
acknowledgments or abandon its live writer to enforce a timer.

This CLASSICAL_DEV loopback implementation doubles missing proof downloads and
can refuse valid larger input within its unchanged public working profile.
Runtime source and local accounting do not satisfy complete B4, PQ, freshness,
hardware power-loss, performance or secure-throughput acceptance by themselves.
