// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::StorageJobChannels;
use crate::sync::applied::{
    AppliedOwner, applied_owner_reader,
    checkpoints::tests::fixtures::{stage_artifacts, witnesses},
    reserve_applied_snapshot_staging,
    tests::import_fixtures::ImportChain,
};
use std::{fs::File, os::unix::fs::PermissionsExt};

/// Reuse the canonical fixture's real chunk/proof fsync operations; no proof generator is duplicated.
pub(super) fn run_checkpoint_io_jobs(
    owner: AppliedOwner,
    chain: &ImportChain,
    channels: StorageJobChannels,
) -> (AppliedOwner, usize) {
    let mut count = 0;
    while channels.run.recv().is_ok() {
        if channels.entered.send(()).is_err() {
            break;
        }
        if channels.start.recv().is_err() {
            break;
        }
        let result = (|| -> Result<(), String> {
            let reader = applied_owner_reader(&owner);
            let _raw_input = reserve_applied_snapshot_staging(&reader, 8 * 1_048_576)
                .map_err(|_| "checkpoint raw input capacity")?;
            let directory = tempfile::Builder::new()
                .permissions(std::fs::Permissions::from_mode(0o700))
                .tempdir()
                .map_err(|_| "checkpoint temporary root")?;
            let file =
                File::open(directory.path()).map_err(|_| "checkpoint temporary root handle")?;
            let proofs = witnesses(chain, 1);
            let artifacts = stage_artifacts(&owner, &file, &chain.commits[1], &proofs)
                .map_err(|_| "actual checkpoint content/proof IO")?;
            drop(artifacts);
            Ok(())
        })();
        if result.is_ok() {
            count += 1;
        }
        if channels.done.send(result).is_err() {
            break;
        }
    }
    (owner, count)
}
