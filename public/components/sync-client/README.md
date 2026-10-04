<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Bounded own-chain development downloads

Public owns this component's request transport. Public and master followers may
reuse it without importing a private master implementation. It has no signing,
voting, account execution, release installation, or remote-chain behavior.

The current transport accepts only explicit loopback TCP endpoints, bounded
ASCII GET paths, one request deadline of at most 30 seconds, finite HTTP headers,
configured body lengths and a bounded chunk count. It rejects ambiguous framing,
unsupported transfer/content encodings, non-success statuses and remote JSON
errors. There is no decompression, redirect following, arbitrary URL resolution,
credential forwarding, or production TLS claim. A later transport can preserve
these request contracts while supplying its separately reviewed authentication.

`required_native_rpc_reservation` estimates 256 times the configured response
ceiling, twice the request ceiling and 65,536 control bytes. This deliberately
conservative JSON/container charge is an operator admission estimate, not an
allocator or RSS guarantee. The numeric argument to a download does not acquire
resources. Every caller must hold an actual working-capacity lease before calling
and through every returned JSON/native value's lifetime. Large response ceilings
can fail the node's working policy; do not raise it silently. Concurrent connection
and request admission belongs to the calling follower's bounded scheduling.

Native JSON decoding belongs to the canonical validator-owned consensus adapter.
`fetch_native_frame` obtains the actual block and matching commit at one requested
height, checks their exact header/block identity and preserves ordered transaction
data. These remain untrusted inputs. Only the canonical verifier's locally anchored
validator history and H/H+1 checks can authenticate application outcomes; a parsed
frame, transport identity, endpoint claim or storage marker cannot grant finality.

Tests use actual loopback sockets for ordinary/chunked response parity, oversized
lengths, conflicting framing, unsupported compression, unavailable responses,
missing reservations, unsafe paths and a slow peer deadline. Public and master
development followers now use this component. [B4 scoped evidence](../../../docs/execution/b4/README.md)
records actual follower, checkpoint activation and peer-tail recovery slices;
the complete B4 local/hosted gate and publication remain NOT_RUN.

Delta downloads bind the exact parent, target metadata, offsets, chunk widths and
whole-body hash. Each request uses the remaining total download deadline; successful
materialization and final body hashing also recheck expiry. These checks refuse late
results; they are not a preemptive CPU/OS scheduling latency guarantee. Client callers
must still validate native history and locally prepared targets before publication.

The [checkpoint transport](src/checkpoints/README.md) fetches uncompressed manifests,
bounded content chunks and execution metadata, then actual native frames separately.
Locally configured genesis and exact requested target/source identities constrain
every response. Sealed preflight precedes owned decode; downloaded artifacts/witnesses
retain caller working/staging leases through their lifetimes. One absolute whole
deadline continues through requests, materialization and final checks without rebasing.
These are untrusted delivery results: canonical private checkpoint verification,
durable base ACK and conditional publication belong to their respective consumers.
The explicit loopback classical profile neither implements dynamic owner/profile
transitions nor proves independent fresh-head availability or production transport.
