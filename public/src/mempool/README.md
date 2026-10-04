<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Public mempool head ownership

Public owns admission, nonce/replacement limits, expiry, selection and revalidation.
The actor holds one immutable MempoolHead. Local heads share their original
StateCommit Arc; applied heads retain the actual AppliedPublication and generation
lease. Captures keep that owner through validation and pending selection. Borrowed
commit access and standard dereference neither clone the state nor grant authority.

`start_mempool` and `committed` preserve the local development API. Applied followers
use `start_applied_mempool` and `applied_committed`, sharing verified RAM directly.
An old captured head remains charged after a pool refresh until its last owner
drops. Refresh revalidates nonce/account constraints and removes included or stale
transactions using the existing canonical admission operations.

A returned pending transaction hash is neither finality nor durable storage.
Network relaying and fresh-head readiness are runtime/network responsibilities;
the pool creates no validator vote or master dependency. Tests cover shared local
identity, actual applied-generation lifetime and revalidation after a real signed
transaction import. Results belong to the recorded root gates.