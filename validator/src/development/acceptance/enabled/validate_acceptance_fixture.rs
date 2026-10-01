// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    decode_acceptance_key::decode_acceptance_key,
    types::{AcceptanceFixture, FixtureInput, TRAILER_TAG},
    validate_acceptance_transitions::validate_acceptance_transitions,
};
use alloy_primitives::Address;
use anyhow::{Context, Result, ensure};
use eve_consensus_comet::consensus::certificates::validator_address;
use eve_protocol_config::genesis::DevelopmentGenesis;
use std::collections::BTreeMap;

pub(super) fn validate_acceptance_fixture(
    input: FixtureInput,
    digest: [u8; 32],
    genesis: &DevelopmentGenesis,
) -> Result<AcceptanceFixture> {
    let address = Address::from_slice(&hex::decode("000000000000000000000000000000000000f1b3")?);
    ensure!(
        input.version == 1 && input.contract_address == address && !input.authority.is_zero(),
        "unsupported acceptance fixture identity"
    );
    let contract = genesis
        .accounts
        .iter()
        .find(|account| account.address == address)
        .context("acceptance contract missing from canonical genesis")?;
    let mut trailer = input.authority.as_slice().to_vec();
    trailer.extend_from_slice(TRAILER_TAG);
    trailer.extend_from_slice(&digest);
    ensure!(
        contract.code.len() > trailer.len() && contract.code.ends_with(&trailer),
        "acceptance manifest/authority is not bound by canonical genesis code"
    );
    ensure!(
        genesis
            .accounts
            .iter()
            .any(|account| account.address == input.authority && account.code.is_empty()),
        "acceptance authority must be a genesis-enrolled transaction account"
    );
    ensure!(
        input.future_validators.len() == 1,
        "acceptance fixture requires exactly one future rotation key"
    );
    let mut future = BTreeMap::new();
    for validator in &input.future_validators {
        let key = decode_acceptance_key(&validator.public_key)?;
        ensure!(
            !genesis
                .validators
                .iter()
                .any(|initial| initial.classical_public_key == key)
                && genesis
                    .validators
                    .iter()
                    .any(|initial| initial.owner == validator.owner)
                && future.insert(key, validator.owner).is_none(),
            "invalid acceptance future-key owner or duplicate enrollment"
        );
    }
    let transitions = validate_acceptance_transitions(&input.transitions, genesis, &future)?;
    let poison = match input.poison {
        Some(poison) => {
            let key = decode_acceptance_key(&poison.public_key)?;
            ensure!(
                (1..=1_000).contains(&poison.height)
                    && genesis
                        .validators
                        .iter()
                        .any(|validator| validator.classical_public_key == key),
                "invalid bounded acceptance proposal fault target"
            );
            Some((poison.height, validator_address(&key)))
        }
        None => None,
    };
    Ok(AcceptanceFixture {
        rejection_recorded: std::sync::atomic::AtomicBool::new(false),
        digest,
        address,
        authority: input.authority,
        future,
        transitions,
        poison,
    })
}
