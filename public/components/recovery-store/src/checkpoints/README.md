<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Bounded local checkpoint transfer

Canonical owner: public durable recovery storage. This capability stages a single
uncompressed canonical `StateCommit` body in an immutable content-addressed local
namespace. It grants no consensus finality, source trust, state activation, fresh
head observation or permission to prune previous recovery data.

`create_checkpoint_manifest` takes a canonical borrowed state preflight, rechecks
the body under the frozen local logical budget, and binds its exact canonical
target version. `preflight_checkpoint_manifest` checks framing, bounds, complete
reference order and equality to the caller's expected canonical version encoding.
It does not allocate a state or authenticate that expected version. A downloaded
manifest is untrusted until the caller completes the separate validator-owned
checkpoint proof/replay verification.

The V1 byte layout uses `EVE_CHECKPOINT_STORE_V1` plus NUL, two zero bytes for the
uncompressed format, a big-endian u16 target length, u64 body length, u32 chunk
width, u32 count, whole-body SHA-256, canonical target bytes, then ordered 48-byte
references: u32 index, u64 offset, u32 length and SHA-256. Reference offsets and
lengths must exactly cover the complete body. Maximum chunk size is 4 MiB,
maximum count 4,096, maximum target encoding 4,096 bytes and maximum manifest
262,144 bytes. The operator may use stricter bounds. Compression is unsupported
and fails closed. Checksums detect local corruption; they cannot authorize state.

The namespace name is the manifest SHA-256 in lowercase hexadecimal. All entry
names are generated internally: `manifest.bin`, `manifest.pending`, fixed-width
`chunk-NNNN.bin`, `complete.bin` and `complete.pending`. Linux safe Rustix calls
operate relative to a held directory file descriptor. The namespace is owned by
the current effective UID, has no group/world permission, and holds an exclusive
nonblocking `flock` for the entire transfer/completed-store lifetime. File opens
use no-follow, exclusive creation and regular-file/single-link checks. A process
with the same filesystem owner remains outside this lock's adversary boundary;
this is cooperative local single-writer exclusion, not remote fencing.

Each accepted chunk is individually hashed and actually file-synced; its directory
is then synced. Completion rereads all chunks in canonical order, checks each hash
and the full body hash, syncs each file, writes/syncs the exact completion marker,
promotes with `RENAME_NOREPLACE`, then syncs the directory. Unsupported filesystems
or platforms refuse these operations; there is no overwrite or weaker durability
fallback. Directory ancestors are supplied and made durable by the caller.

An identical manifest resumes preserved files and reports verified/missing/corrupt
chunk counts. Completion and body reads revalidate the actual files. Valid chunks
are never overwritten, and completed namespaces cannot be repaired or written.
Explicit repair APIs may remove only a rechecked invalid generated staging entry
inside an exclusively held uncompleted namespace. Symlinks are unlinked without
following their targets; hardlinked regular files and valid content are refused.
Invalid published metadata or unknown entries are preserved and fail closed.
An interrupted `manifest.pending` or `complete.pending` needs its named explicit
repair operation before ordinary resume. Completion-pending repair accepts only
an invalid regular single-link entry; correctly framed completion data, even for
another manifest, is preserved and refused. A complete published marker blocks
every repair operation.
Pending-manifest repair classifies borrowed known-format framing under fixed V1
format bounds, independent of this receiver's smaller body/chunk/count limits.
Unrecognized future metadata and files above the current manifest read ceiling
are conservatively preserved. An oversized file is refused before any owned read
or allocation. This does not enlarge transfer admission or the frozen logical
state budget; valid foreign metadata is not treated as corruption.
Completion repair also preserves unknown future marker prefixes, short unknown
metadata and files exceeding the 80-byte current marker ceiling before any
owned read. Only a provably truncated recognized V1 completion may be removed;
the V1 marker contains no codec flags. The borrowed `checkpoint_store_manifest_bytes`
getter exposes the exact already retained manifest under the completed store's
caller-held metadata lease, without a new allocation or path read.

`required_checkpoint_metadata_reservation` covers retained manifest copies,
directory iteration and bounded startup hash scratch. Its actual caller lease
must outlive the transfer/completed handle and any failure tail. The separate IO
reservation covers temporary scratch; borrowed incoming chunk bytes require their
own caller charge. The body reservation covers the exact reconstructed owned body
and scratch; its real lease must outlive `CheckpointBody`. Numeric arguments state
the required contract but do not create or verify an actual runtime lease. These
logical bounds do not claim process RSS, OS cache or strict physical disk caps.

`read_checkpoint_body` verifies completion, each file, full-body hash, canonical
state preflight and exact target binding before returning an owned buffer. The
sealed `preflight_checkpoint_body` then lets a separately reserved caller use the
one canonical state decoder. This module does not duplicate state maps, EVM roots,
execution validation, signature verification or validator history.

The Linux `checkpoint_transfer` tests cover actual file/directory sync and reopen,
no-destructor child exit, partial staging, explicit repair, canonical target and
stricter logical budget, corrupt/missing/reordered/oversized content, reservations,
exclusive ownership, unknown names, symlinks, hardlinks and immutable completion.
Artificial truncated/corrupt files are labelled local staging faults. Process exit
and filesystem checks do not prove hardware power-loss durability, malicious local
owner resistance, authenticated snapshot activation or full B4 acceptance.
