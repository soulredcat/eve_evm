// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::application::{ConsensusApplication, context::native_application_hash};
use anyhow::{Result, ensure};
use eve_consensus_comet::wire::tendermint::abci::ResponseInfo;
use eve_storage::state::read_state_service;

pub(in crate::consensus) fn application_info(
    application: &ConsensusApplication,
) -> Result<ResponseInfo> {
    ensure!(
        !application.fenced,
        "application fenced; reconciliation required"
    );
    let head = read_state_service(&application.service)?;
    Ok(ResponseInfo {
        data: "EVE/CLASSICAL_DEV/LOCAL_ENGINE_EXECUTION".into(),
        version: "1".into(),
        app_version: u64::from(head.commit().target.identity.protocol_version),
        last_block_height: i64::try_from(head.commit().target.height)?,
        last_block_app_hash: native_application_hash(&head.commit().target)?,
    })
}
