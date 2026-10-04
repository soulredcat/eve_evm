// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! SIMULATED_UNSYNCED_LOSS with actual acknowledged WAL rows and native certificates, not hardware power loss.
#[path = "nonempty_missing_tail/lose_nonempty_marker.rs"]
mod lose_nonempty_marker;
#[path = "nonempty_missing_tail/nonempty_import_chain.rs"]
mod nonempty_import_chain;
#[path = "nonempty_missing_tail/read_peer_tail.rs"]
mod read_peer_tail;
#[path = "nonempty_missing_tail/recover.rs"]
mod recover;
#[path = "nonempty_missing_tail/store_peer_tail.rs"]
mod store_peer_tail;
#[path = "nonempty_missing_tail/unavailable.rs"]
mod unavailable;
