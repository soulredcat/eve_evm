// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod commands;
mod configuration;
mod home;
mod lifecycle;
mod types;
mod verification;

pub(crate) use commands::engine_node_id;
pub(crate) use home::initialize_engine_home;
#[cfg(test)]
pub(crate) use lifecycle::engine_pid;
pub(crate) use lifecycle::{
    engine_authentication_context, engine_child, start_engine, stop_engine,
};
pub(crate) use types::OwnedEngine;
#[cfg(test)]
pub(crate) use verification::verify_test_engine_image;
pub(crate) use verification::{
    EngineImageIdentity, VerifiedEngineImage, engine_process_image_identity,
    validate_engine_image_binding,
};

#[cfg(test)]
#[cfg(target_os = "linux")]
mod tests;
