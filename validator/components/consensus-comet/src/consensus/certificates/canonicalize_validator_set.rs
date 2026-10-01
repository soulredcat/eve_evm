// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CertificateError, ClassicalValidator, validate_validator_set::validate_validator_set,
    validator_address,
};

/// Reproduce native set ordering without copying proposer-priority state.
pub fn canonicalize_validator_set(
    validators: &[ClassicalValidator],
) -> Result<Vec<ClassicalValidator>, CertificateError> {
    validate_validator_set(validators, false)?;
    let mut ordered = validators.to_vec();
    ordered.sort_by_key(|validator| {
        (
            -validator.voting_power,
            validator_address(&validator.public_key),
        )
    });
    Ok(ordered)
}
