// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::delta_serving_error_response::delta_serving_error_response;
use super::{prepare_checkpoint_query::prepare_checkpoint_query, types::SnapshotServingBudget};
use crate::consensus::application::ConsensusApplication;
use eve_consensus_comet::wire::tendermint::abci::{RequestQuery, ResponseQuery};
pub(in crate::consensus) fn serve_checkpoint_query(
    application: &mut ConsensusApplication,
    request: &RequestQuery,
    budget: SnapshotServingBudget,
) -> ResponseQuery {
    match prepare_checkpoint_query(application, request, budget) {
        Ok(response) => response,
        Err(error) => delta_serving_error_response(error),
    }
}
