<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Native RPC JSON decoding

Validator owns the one canonical decoder for pinned CometBFT native JSON block,
header, commit and validator shapes. The B3 acceptance helper reexports these
operations so runtime and test callers cannot drift into independent parsers.
Certificate/signature admission remains in the consensus certificate operations.

The decoder bounds chain names, numeric strings, timestamps, hashes, transaction
count/bytes and classical validator/signature counts. It verifies native block
header identity and ordered transaction Data.Hash using canonical adapter helpers.
Unsupported evidence arrays and key types fail closed. Raw JSON parsing and its
working resources belong to the caller; this decoder does not acquire a node pool.

A returned NativeBlock or commit is untrusted data. Applicable validator identities,
authentication profile, height mapping, transition history and execution/state
binding must still be checked by the locally anchored finality verifier. Current
support is CLASSICAL_DEV; it makes no PQ or production light-client claim.