// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    delta_serving_error_response::delta_serving_error_response,
    prepare_state_delta_query::prepare_state_delta_query, types::DeltaServingBudget,
};
use crate::consensus::application::ConsensusApplication;
use eve_consensus_comet::wire::tendermint::abci::{RequestQuery, ResponseQuery};

/// Exclusive serial actor access. Untrusted chunks require independent native H/H+1
/// and canonical import verification; this endpoint changes no durable/voting state.
pub(in crate::consensus) fn serve_state_delta_query(
    application: &mut ConsensusApplication,
    input: &RequestQuery,
    budget: DeltaServingBudget,
) -> ResponseQuery {
    prepare_state_delta_query(application, input, budget)
        .unwrap_or_else(delta_serving_error_response)
}
