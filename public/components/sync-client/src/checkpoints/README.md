<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Bounded development checkpoint downloads

This public-owned component downloads own-chain checkpoint messages through the
existing loopback native RPC transport. It uses the canonical recovery-storage
message codec and the canonical validator-owned witness encoder and native parser.
Transport responses, local manifest hashes and assembled witnesses remain untrusted.

`fetch_checkpoint_response` reserves the configured native RPC envelope before
request encoding, JSON and base64 allocation. The sealed response preflight supplies
the actual encoded-size decode allowance. A second caller lease precedes owned
decode and exact canonical reencoding. Both leases remain with the private response
until it drops. The public accessor borrows that response and cannot detach it.

The client checks the requested kind, positive height, configured genesis identity,
manifest chunk width, and requested chunk manifest/index/width. Canonical framing
checks full-width chunks except the final tail. Remote query errors expose fixed
categories, excluding untrusted peer diagnostic text. Manifest integrity and these
equalities grant no finality, freshness, execution or durable artifact authority.

`fetch_checkpoint_witness` takes explicit checkpoint and witness heights. Heights
through H obtain execution metadata; only H+1 produces a closing lookahead.
Owned metadata moves into the witness under its original retained leases.
Actual borrowed native protobuf sizes and transaction counts determine another
caller charge before component and final wire encoding. The maintained canonical
measure and encoder still validate their own limits. No cryptography is duplicated.

One absolute configured deadline covers reservations, encoding, socket requests,
canonical parsing and final materialization. Native block and commit requests use
remaining time from that same deadline. The original native download API now also
uses one deadline for its two requests. Valid large responses can refuse under
unchanged caller budgets. Logical reservations are not allocator or RSS proofs.

The HTTP deadline variant takes the caller's actual Instant directly. It selects
the earlier of that Instant and its configured entry timeout, then derives the
connect timeout immediately before connecting. Formatting and admission cannot
receive a fresh timeout through conversion to a duration and rebasing.

Public response and witness deadline variants also accept an absolute outer
deadline and clamp it to their configured operation timeout at entry. A long
bootstrap deadline cannot relax the per-operation limit. An already expired
outer deadline refuses before invoking the caller's reservation callback.

The real loopback source tests use canonical shared development fixtures. They do
not establish authenticated checkpoint acceptance, public runtime integration,
production TLS, quantum security, independently corroborated freshness or full B4.
