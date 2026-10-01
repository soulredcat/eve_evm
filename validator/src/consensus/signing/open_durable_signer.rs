// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    DurableSigner, SignerConfig,
    records::{decode_record, validate_record},
    validate_signer_config,
};
use anyhow::{Context, Result, ensure};
use ed25519_dalek::SigningKey;
use eve_storage::{
    records::{
        OpaqueRecordBudget, OpaqueRecordIdentity, opaque_record_cursor,
        open_opaque_record_repository, read_opaque_record,
    },
    state::StateService,
};
use sha2::{Digest, Sha256};
use std::{path::Path, sync::Arc};

pub(in crate::consensus) fn open_durable_signer(
    path: &Path,
    config: SignerConfig,
    key: SigningKey,
    service: Arc<StateService>,
    budget: OpaqueRecordBudget,
) -> Result<DurableSigner> {
    validate_signer_config(&config, &key, &service)?;
    let identity = OpaqueRecordIdentity {
        genesis_hash: config.genesis_hash,
        owner: config.expected_public_key,
        domain: Sha256::digest(b"EVE_VALIDATOR_SIGNING_HISTORY_V1").into(),
    };
    let repository = open_opaque_record_repository(path, identity, budget)?;
    let cursor = opaque_record_cursor(&repository)?;
    let mut last = None;
    for sequence in 1..=cursor.sequence {
        let stored = read_opaque_record(&repository, sequence)?
            .context("required signer history record missing")?;
        let record = decode_record(&stored.payload)?;
        validate_record(&record, &config)?;
        ensure!(
            last.as_ref()
                .is_none_or(|prior: &super::types::SignedRecord| record.hrs > prior.hrs),
            "nonmonotonic signer history"
        );
        last = Some(record);
    }
    Ok(DurableSigner {
        config,
        service,
        key,
        repository,
        cursor,
        last,
        fenced: false,
        #[cfg(test)]
        simulated_failure: None,
    })
}
