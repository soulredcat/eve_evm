// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    append_synced_replay, decode_replay_record,
    types::{ReplayDecision, ReplayRecord},
    validate_replay_decision,
};
use crate::consensus::application::{ConsensusApplication, reading::retained_application_block};
use anyhow::{Context, Result, ensure};
use eve_storage::{
    records::{OpaqueRecordCursor, read_opaque_record},
    state::read_state_service,
};

pub(in crate::consensus::application) fn reconcile_application_replay(
    application: &mut ConsensusApplication,
) -> Result<()> {
    let mut completed: Option<ReplayDecision> = None;
    let mut pending: Option<(ReplayDecision, OpaqueRecordCursor)> = None;
    let mut previous = [0; 32];
    let head = read_state_service(&application.service)?;
    for sequence in 1..=application.replay_cursor.sequence {
        let stored = read_opaque_record(&application.replay_repository, sequence)?
            .context("required consensus replay record missing")?;
        let cursor = OpaqueRecordCursor {
            sequence: stored.sequence,
            content_hash: stored.content_hash,
        };
        match decode_replay_record(&stored.payload)? {
            ReplayRecord::Decided(decision) => {
                let decision = *decision;
                ensure!(
                    pending.is_none(),
                    "new consensus decision before prior synced completion"
                );
                validate_replay_decision(&decision, &application.config, previous)?;
                let expected_parent = completed
                    .as_ref()
                    .map_or(&application.config.genesis.target, |record| &record.target);
                ensure!(
                    &decision.parent == expected_parent,
                    "noncontiguous consensus replay parent"
                );
                pending = Some((decision, cursor));
            }
            ReplayRecord::Synced {
                target,
                commit_identity,
                decision_cursor,
            } => {
                let (decision, cursor) = pending
                    .take()
                    .context("synced replay marker lacks decided material")?;
                ensure!(
                    decision_cursor == cursor
                        && *target == decision.target
                        && commit_identity == decision.commit_identity,
                    "synced consensus replay marker mismatch"
                );
                let retained = retained_application_block(application, target.height)?;
                ensure!(
                    retained.version == *target && retained.commit_identity == commit_identity,
                    "synced consensus metadata differs from actual retained state"
                );
                previous = decision
                    .request
                    .hash
                    .as_slice()
                    .try_into()
                    .context("retained consensus hash width")?;
                completed = Some(decision);
                application.completed_cursor = Some(cursor);
            }
        }
    }
    application.completed = completed;
    if let Some((decision, cursor)) = pending {
        if head.commit().target == decision.target {
            let retained = retained_application_block(application, decision.target.height)?;
            ensure!(
                retained.commit_identity == decision.commit_identity,
                "pending decision differs from synced whole state"
            );
            append_synced_replay(application, &decision, cursor)?;
        } else {
            ensure!(
                head.commit().target == decision.parent,
                "pending consensus decision does not match actual state head"
            );
            application.retained_decision = Some((decision, cursor));
        }
    } else {
        let expected = application
            .completed
            .as_ref()
            .map_or(&application.config.genesis.target, |record| &record.target);
        ensure!(
            &head.commit().target == expected,
            "application state lacks matching consensus replay history"
        );
    }
    Ok(())
}
