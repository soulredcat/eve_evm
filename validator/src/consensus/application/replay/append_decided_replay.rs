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

pub(in crate::consensus::application) fn append_decided_replay(
    application: &mut ConsensusApplication,
    decision: ReplayDecision,
) -> Result<(ReplayDecision, OpaqueRecordCursor)> {
    let record = ReplayRecord::Decided(Box::new(decision));
    let payload = encode_replay_record(&record)?;
    let ack = compare_and_append_opaque_records(
        &mut application.replay_repository,
        application.replay_cursor,
        &[payload],
    )?;
    application.replay_cursor = ack.store_head;
    match record {
        ReplayRecord::Decided(decision) => Ok((*decision, ack.appended)),
        _ => unreachable!("local decided constructor"),
    }
}
