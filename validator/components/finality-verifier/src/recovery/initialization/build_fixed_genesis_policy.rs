// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::collections::BTreeMap;

use eve_consensus_comet::consensus::certificates::{
    CertificateError, ClassicalValidator, canonicalize_validator_set, hash_validator_set,
    validator_address,
};
use eve_state::DevelopmentGenesis;

use crate::recovery::{RecoveryError, types::capability::FixedGenesisPolicy};

pub(in crate::recovery) fn build_fixed_genesis_policy(
    genesis: &DevelopmentGenesis,
) -> Result<FixedGenesisPolicy, RecoveryError> {
    let mut proposer_owners = BTreeMap::new();
    let mut validators = Vec::with_capacity(genesis.validators.len());
    for validator in &genesis.validators {
        let native_address = validator_address(&validator.classical_public_key);
        if proposer_owners
            .insert(native_address, validator.owner)
            .is_some()
        {
            return Err(RecoveryError::Certificate(
                CertificateError::InvalidValidatorSet,
            ));
        }
        validators.push(ClassicalValidator {
            public_key: validator.classical_public_key,
            voting_power: i64::try_from(validator.voting_power)
                .map_err(|_| RecoveryError::Certificate(CertificateError::VotingPowerOverflow))?,
        });
    }
    let validators = canonicalize_validator_set(&validators).map_err(RecoveryError::Certificate)?;
    let validator_hash = hash_validator_set(&validators).map_err(RecoveryError::Certificate)?;
    Ok(FixedGenesisPolicy {
        validators,
        validator_hash,
        proposer_owners,
    })
}
