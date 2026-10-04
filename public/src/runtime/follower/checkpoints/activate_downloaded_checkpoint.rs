// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::check_checkpoint_bootstrap_deadline::check_checkpoint_bootstrap_deadline;
use crate::sync::applied::{
    AppliedOwner,
    checkpoints::{
        ChargedCheckpointArtifacts, poll_applied_checkpoint_activation, prepare_applied_checkpoint,
        start_applied_checkpoint_activation,
    },
};
use anyhow::Result;
use eve_state::DevelopmentGenesis;
use std::time::{Duration, Instant};

pub(super) fn activate_downloaded_checkpoint(
    owner: &mut AppliedOwner,
    artifacts: ChargedCheckpointArtifacts,
    local_genesis: &DevelopmentGenesis,
    deadline: Instant,
) -> Result<()> {
    check_checkpoint_bootstrap_deadline(deadline)?;
    let prepared = prepare_applied_checkpoint(artifacts, local_genesis)
        .map_err(|error| anyhow::anyhow!("checkpoint canonical finality preparation: {error:?}"))?;
    check_checkpoint_bootstrap_deadline(deadline)?;
    start_applied_checkpoint_activation(owner, prepared)
        .map_err(|error| anyhow::anyhow!("checkpoint base submission: {error:?}"))?;
    loop {
        check_checkpoint_bootstrap_deadline(deadline)?;
        if poll_applied_checkpoint_activation(owner)
            .map_err(|error| anyhow::anyhow!("checkpoint durable activation: {error:?}"))?
            .is_some()
        {
            return check_checkpoint_bootstrap_deadline(deadline);
        }
        std::thread::sleep(Duration::from_millis(1));
    }
}
