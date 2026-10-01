<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Disposable B3 acceptance adapters

This private development domain owns the explicitly compiled B3 lifecycle and
invalid-proposal fixtures. `development-acceptance` is disabled by default. Normal
builds have an uninhabited fixture type and reject both a fixture option and a
marked acceptance genesis. Enabled builds also reject marked genesis without its
exact manifest. This prevents different application rules under one accepted
genesis. Loopback, classical development and explicit unsafe acknowledgment remain
mandatory. No production staking, key ceremony, custody or majority guarantee is
provided; B5 must replace this temporary lifecycle adapter.

The public JSON manifest is limited to 16 KiB, version one, one future rotation key
and exactly three ordered transitions: rotate, leave, jail. The fixed genesis EVM
account `0x000000000000000000000000000000000000f1b3` contains the reviewed test
contract with `authority20 || EVE_B3_ACCEPTANCE_V1 || SHA256(exact manifest bytes)`
as its 72-byte footer. Canonical genesis identity/state hashing binds that code,
authority and manifest. File changes cannot silently activate another schedule.
Future enrollment requires a strong key and an existing owner's rotation; power
stays equal to that owner's previously backed power. Active sets keep at least two
members and valid bounded positive total power. Initial four-member genesis and
native quorum calculations remain unchanged.

Only a real chain-protected signature from the enrolled fixture authority,
calling `transition(uint8)` at the fixed address, can produce updates. The actual
canonical EVM receipt must succeed and contain exactly the expected address,
event signature, manifest digest and action. The fixture contract enforces action
sequence and rejects unauthorized or repeated calls. Native updates are derived
from committed transaction/receipt data; an operator file or master assertion
cannot trigger them. Retained FinalizeBlock replay derives the same response from
the same frozen manifest and canonical payload, without repeating EVM effects.
Native activation remains H+2, with the distinct H+1/H+3 metadata boundaries.

The optional bounded proposal fault selects one genesis validator and height. Its
first PrepareProposal returns one malformed envelope in that disposable process.
It never bypasses ProcessProposal, necessary data, durable signing, native locking
or non-nil vote approval. A restart can repeat this local fault; it is a test hook,
not production behavior or a durable exactly-once service. Invalid proposals must
be rejected while honest validators continue with a later valid proposer.

All functions have one named operation and narrow crate visibility. `enabled/`
contains the actual fixture operations; `unavailable/` implements the normal
build's absent capability. Reproducible Solidity/process/security acceptance belongs
in `tests/acceptance/validator-consensus/`; raw generated files, keys and logs stay
in ignored local storage. Passing these tests establishes only the measured
classical development boundary under the declared fault model.
