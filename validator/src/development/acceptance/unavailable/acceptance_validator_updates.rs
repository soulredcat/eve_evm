// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::AcceptanceFixture;
use anyhow::Result;
use eve_consensus_comet::wire::tendermint::abci::ValidatorUpdate;
use eve_state::{BlockPayload, StateVersion};

pub(crate) fn acceptance_validator_updates(
    _fixture: Option<&AcceptanceFixture>,
    _version: &StateVersion,
    _block: &BlockPayload,
) -> Result<Vec<ValidatorUpdate>> {
    Ok(Vec::new())
}
