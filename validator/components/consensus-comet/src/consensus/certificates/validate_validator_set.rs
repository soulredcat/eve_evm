// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CertificateError, ClassicalValidator,
    types::{MAX_DEVELOPMENT_VALIDATORS, MAX_NATIVE_VOTING_POWER},
    validator_address,
};
use std::collections::BTreeSet;

pub(super) fn validate_validator_set(
    validators: &[ClassicalValidator],
    ordered: bool,
) -> Result<i64, CertificateError> {
    if validators.is_empty() || validators.len() > MAX_DEVELOPMENT_VALIDATORS {
        return Err(CertificateError::InvalidValidatorSet);
    }
    let mut keys = BTreeSet::new();
    let mut addresses = BTreeSet::new();
    let mut total = 0_i64;
    let mut previous: Option<(i64, [u8; 20])> = None;
    for validator in validators {
        let address = validator_address(&validator.public_key);
        if !keys.insert(validator.public_key)
            || !addresses.insert(address)
            || validator.voting_power <= 0
        {
            return Err(CertificateError::InvalidValidatorSet);
        }
        if ordered
            && previous.is_some_and(|(power, prior)| {
                power < validator.voting_power
                    || (power == validator.voting_power && prior >= address)
            })
        {
            return Err(CertificateError::InvalidValidatorSet);
        }
        total = total
            .checked_add(validator.voting_power)
            .ok_or(CertificateError::VotingPowerOverflow)?;
        if total > MAX_NATIVE_VOTING_POWER {
            return Err(CertificateError::VotingPowerOverflow);
        }
        previous = Some((validator.voting_power, address));
    }
    Ok(total)
}
