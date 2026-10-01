// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    decode_replay_record,
    types::{ReplayDecision, ReplayRecord},
};
use crate::consensus::application::ConsensusApplication;
use anyhow::{Context, Result};
use eve_storage::records::{OpaqueRecordCursor, read_opaque_record};

/// Cache first; only historical misses scan the bounded retained opaque history.
pub(in crate::consensus::application) fn find_replay_decision(
    application: &ConsensusApplication,
    height: u64,
) -> Result<Option<(ReplayDecision, OpaqueRecordCursor)>> {
    if let (Some(decision), Some(cursor)) = (&application.completed, application.completed_cursor)
        && decision.target.height == height
    {
        return Ok(Some((decision.clone(), cursor)));
    }
    if let Some((decision, cursor)) = &application.retained_decision
        && decision.target.height == height
    {
        return Ok(Some((decision.clone(), *cursor)));
    }
    for sequence in 1..=application.replay_cursor.sequence {
        let record = read_opaque_record(&application.replay_repository, sequence)?
            .context("retained consensus decision record missing")?;
        if let ReplayRecord::Decided(decision) = decode_replay_record(&record.payload)?
            && decision.target.height == height
        {
            return Ok(Some((
                *decision,
                OpaqueRecordCursor {
                    sequence: record.sequence,
                    content_hash: record.content_hash,
                },
            )));
        }
    }
    Ok(None)
}
