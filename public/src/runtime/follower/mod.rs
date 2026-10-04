// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod advance_development_follower;
mod build_segmented_follower_configuration;
mod open_development_follower;
mod run_development_follower;
mod run_follower_actor;
mod types;
mod validate_development_follower;
pub use run_development_follower::run_development_follower;
pub use types::DevelopmentFollowerConfig;

mod wait_for_follower_shutdown_signal;

#[cfg(test)]
mod tests;

mod advance_follower_iteration;
