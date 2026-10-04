// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Private master archive orchestration; canonical validator proofs grant authority.
mod archive;
mod cli;
mod config;
mod following;
mod recovery;
mod resources;
mod status;
mod types;

pub use cli::{MasterFollowerOptions, run_master_follower_cli};
pub use config::master_sync_development_config;
pub use following::{follow_master_to_height, import_master_wire};
pub use recovery::open_master_follower;
pub use status::{current_master_commit, master_sync_status};
pub use types::{MasterFollower, MasterSyncConfig, MasterSyncStatus};

#[cfg(test)]
#[cfg(target_os = "linux")]
pub(crate) mod tests;
