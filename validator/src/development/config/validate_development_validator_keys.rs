// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::development::acceptance::{AcceptanceFixture, acceptance_key_enrolled};
use anyhow::{Result, ensure};
use ed25519_dalek::SigningKey;
use eve_protocol_config::genesis::DevelopmentGenesis;

/// EVE enrollment policy is stronger than native signature-verifier admission.
pub(crate) fn validate_development_validator_keys(
    genesis: &DevelopmentGenesis,
    key: &SigningKey,
    fixture: Option<&AcceptanceFixture>,
) -> Result<()> {
    ensure!(
        (4..=64).contains(&genesis.validators.len()),
        "development validator set requires four to sixty-four members"
    );
    for validator in &genesis.validators {
        eve_protocol_config::genesis::validate_classical_enrollment_key(
            &validator.classical_public_key,
        )
        .map_err(|_| anyhow::anyhow!("invalid canonical prime-order enrolled key"))?;
    }
    let public = key.verifying_key().to_bytes();
    ensure!(
        genesis
            .validators
            .iter()
            .any(|validator| validator.classical_public_key == public)
            || acceptance_key_enrolled(fixture, &public),
        "private signing key does not match development enrollment"
    );
    Ok(())
}
