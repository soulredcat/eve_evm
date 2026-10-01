// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod authentication;
mod frames;
mod idle_shutdown;
mod native_test_peer_worker;
mod spawn_test_connection;
mod types;

pub(in crate::consensus) use spawn_test_connection::spawn_authenticated_test_peer;
use spawn_test_connection::spawn_test_connection;
pub(in crate::consensus) use types::AuthenticatedTestPeer;
