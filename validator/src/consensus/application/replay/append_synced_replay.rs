// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    encode_replay_record,
    types::{ReplayDecision, ReplayRecord},
};
use crate::consensus::application::ConsensusApplication;
use anyhow::Result;
use eve_storage::records::{OpaqueRecordCursor, compare_and_append_opaque_records};

pub(in crate::consensus::application) fn append_synced_replay(
    application: &mut ConsensusApplication,
    decision: &ReplayDecision,
    cursor: OpaqueRecordCursor,
) -> Result<()> {
    let payload = encode_replay_record(&ReplayRecord::Synced {
        target: Box::new(decision.target.clone()),
        commit_identity: decision.commit_identity,
        decision_cursor: cursor,
    })?;
    let ack = compare_and_append_opaque_records(
        &mut application.replay_repository,
        application.replay_cursor,
        &[payload],
    )?;
    application.replay_cursor = ack.store_head;
    application.completed = Some(decision.clone());
    application.completed_cursor = Some(cursor);
    application.retained_decision = None;
    Ok(())
}
