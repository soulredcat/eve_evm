// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    DurableSigner,
    records::{encode_record, validate_record},
    types::{Hrs, SignedRecord},
};
use anyhow::{Result, anyhow};
use ed25519_dalek::Signer;
use eve_storage::records::compare_and_append_opaque_records;

/// Complete returned material is synced atomically before it becomes caller-visible.
pub(super) fn persist_signature(
    signer: &mut DurableSigner,
    hrs: Hrs,
    bytes: Vec<u8>,
) -> Result<Vec<u8>> {
    let signature = signer.key.sign(&bytes).to_bytes().to_vec();
    let record = SignedRecord {
        version: 1,
        genesis_hash: signer.config.genesis_hash,
        chain_id: signer.config.chain_id.clone(),
        public_key: signer.config.expected_public_key,
        profile: 1,
        key_epoch: signer.config.key_epoch,
        hrs,
        sign_bytes: bytes,
        signature: signature.clone(),
    };
    validate_record(&record, &signer.config)?;
    let payload = encode_record(&record)?;
    #[cfg(test)]
    if signer.simulated_failure == Some(super::types::SimulatedSignerFailure::BeforeWrite) {
        signer.fenced = true;
        return Err(anyhow!("SIMULATED signer failure before append; fenced"));
    }
    let acknowledgment =
        compare_and_append_opaque_records(&mut signer.repository, signer.cursor, &[payload]);
    let acknowledgment = match acknowledgment {
        Ok(value) => value,
        Err(_) => {
            signer.fenced = true;
            return Err(anyhow!("signer append failed; fenced"));
        }
    };
    signer.cursor = acknowledgment.store_head;
    signer.last = Some(record);
    #[cfg(test)]
    match signer.simulated_failure {
        Some(super::types::SimulatedSignerFailure::AfterSync) => {
            signer.fenced = true;
            return Err(anyhow!(
                "SIMULATED signer failure after successful append; fenced"
            ));
        }
        Some(super::types::SimulatedSignerFailure::ExitAfterSync) => std::process::exit(71),
        _ => {}
    }
    Ok(signature)
}
