// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod cleanup_owned_engine_socket;
mod engine_authentication_context;
mod engine_child;
#[cfg(test)]
mod engine_pid;
mod finish_owned_engine;
mod owned_engine_drop_adapter;
mod start_engine;
mod stop_engine;
pub(super) use cleanup_owned_engine_socket::cleanup_owned_engine_socket;
pub(crate) use engine_authentication_context::engine_authentication_context;
pub(crate) use engine_child::engine_child;
#[cfg(test)]
pub(crate) use engine_pid::engine_pid;
pub(crate) use start_engine::start_engine;
pub(crate) use stop_engine::stop_engine;
