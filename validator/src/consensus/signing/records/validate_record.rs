// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::validate_signed_message;
use crate::consensus::signing::{SignerConfig, types::SignedRecord};
use anyhow::{Result, ensure};
use eve_consensus_comet::consensus::certificates::verify_native_ed25519_signature;

pub(in crate::consensus::signing) fn validate_record(
    record: &SignedRecord,
    config: &SignerConfig,
) -> Result<()> {
    ensure!(
        record.version == 1 && record.profile == 1,
        "unsupported signer record schema/profile"
    );
    ensure!(
        record.genesis_hash == config.genesis_hash
            && record.chain_id == config.chain_id
            && record.public_key == config.expected_public_key
            && record.key_epoch == config.key_epoch,
        "signer record identity mismatch"
    );
    ensure!(
        record.hrs.height > 0 && record.hrs.round >= 0 && (1..=3).contains(&record.hrs.step),
        "invalid persisted signer HRS"
    );
    ensure!(
        record.sign_bytes.len() <= 4096 && record.signature.len() == 64,
        "invalid persisted signer byte widths"
    );
    validate_signed_message(record, config)?;
    ensure!(
        verify_native_ed25519_signature(
            &config.expected_public_key,
            &record.sign_bytes,
            &record.signature
        )
        .is_ok(),
        "invalid persisted signer signature"
    );
    Ok(())
}
