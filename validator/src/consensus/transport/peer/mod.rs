// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

#[cfg(unix)]
mod authenticate_engine_peer;
#[cfg(test)]
mod engine_peer_pid;
mod engine_peer_read_ready;
mod engine_peer_shutdown_handle;
mod engine_read_deadline;
mod engine_write_deadline;
mod ensure_application_engine_peer;
mod ensure_engine_channel;
mod flush_engine_peer;
mod read_engine_peer_bytes;
mod shutdown_engine_peer;
mod shutdown_engine_peer_handle;
mod types;
mod validate_engine_peer_child;
mod validate_engine_peer_liveness;
mod validate_linux_transport;
mod write_engine_peer_bytes;

#[cfg(unix)]
pub(in crate::consensus) use authenticate_engine_peer::authenticate_engine_peer;
#[cfg(test)]
pub(in crate::consensus) use engine_peer_pid::engine_peer_pid;
pub(in crate::consensus) use engine_peer_read_ready::engine_peer_read_ready;
pub(in crate::consensus) use engine_peer_shutdown_handle::engine_peer_shutdown_handle;
pub(super) use engine_read_deadline::engine_read_deadline;
pub(super) use engine_write_deadline::engine_write_deadline;
pub(in crate::consensus) use ensure_application_engine_peer::ensure_application_engine_peer;
pub(super) use ensure_engine_channel::ensure_engine_channel;
pub(super) use flush_engine_peer::flush_engine_peer;
pub(super) use read_engine_peer_bytes::read_engine_peer_bytes;
pub(in crate::consensus) use shutdown_engine_peer::shutdown_engine_peer;
pub(in crate::consensus) use shutdown_engine_peer_handle::shutdown_engine_peer_handle;
pub(in crate::consensus) use types::{AuthenticatedEnginePeer, EngineChannel, EnginePeerShutdown};
pub(in crate::consensus) use validate_engine_peer_child::validate_engine_peer_child;
pub(in crate::consensus) use validate_engine_peer_liveness::validate_engine_peer_liveness;
pub(in crate::consensus) use validate_linux_transport::validate_linux_transport;
pub(super) use write_engine_peer_bytes::write_engine_peer_bytes;

#[cfg(test)]
#[cfg(target_os = "linux")]
pub(in crate::consensus) mod tests;
