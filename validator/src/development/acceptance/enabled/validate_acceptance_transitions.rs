// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    decode_acceptance_key::decode_acceptance_key,
    types::{Transition, TransitionKind},
};
use alloy_primitives::Address;
use anyhow::{Context, Result, ensure};
use eve_consensus_comet::wire::tendermint::{
    abci::ValidatorUpdate,
    crypto::{PublicKey, public_key::Sum},
};
use eve_protocol_config::genesis::DevelopmentGenesis;
use std::collections::{BTreeMap, BTreeSet};

pub(super) fn validate_acceptance_transitions(
    input: &[Transition],
    genesis: &DevelopmentGenesis,
    future: &BTreeMap<[u8; 32], Address>,
) -> Result<BTreeMap<u8, Vec<ValidatorUpdate>>> {
    ensure!(
        input.len() == 3,
        "acceptance requires rotate, leave and jail transitions"
    );
    let mut roster: BTreeMap<_, _> = genesis
        .validators
        .iter()
        .map(|validator| {
            (
                validator.classical_public_key,
                (validator.owner, validator.voting_power),
            )
        })
        .collect();
    let mut transitions = BTreeMap::new();
    for (index, transition) in input.iter().enumerate() {
        let expected = match index {
            0 => TransitionKind::Rotate,
            1 => TransitionKind::Leave,
            _ => TransitionKind::Jail,
        };
        let expected_updates = usize::from(index == 0) + 1;
        ensure!(
            transition.action as usize == index + 1
                && transition.kind == expected
                && transition.updates.len() == expected_updates,
            "acceptance transition order/shape mismatch"
        );
        let mut seen = BTreeSet::new();
        let mut removed = None;
        let mut added = None;
        let mut native = Vec::new();
        for update in &transition.updates {
            let key = decode_acceptance_key(&update.public_key)?;
            ensure!(seen.insert(key), "duplicate acceptance update key");
            if update.power == 0 {
                let previous = roster
                    .remove(&key)
                    .context("acceptance removal key is not active")?;
                ensure!(
                    removed.replace(previous).is_none(),
                    "multiple acceptance removals"
                );
            } else {
                let owner = *future
                    .get(&key)
                    .context("acceptance addition key is not future-enrolled")?;
                ensure!(
                    added.replace((owner, update.power)).is_none()
                        && roster.insert(key, (owner, update.power)).is_none(),
                    "invalid acceptance addition"
                );
            }
            native.push(ValidatorUpdate {
                pub_key: Some(PublicKey {
                    sum: Some(Sum::Ed25519(key.to_vec())),
                }),
                power: i64::try_from(update.power)?,
            });
        }
        if index == 0 {
            ensure!(
                removed == added && removed.is_some(),
                "rotation must retain owner and backed power"
            );
        } else {
            ensure!(
                added.is_none() && removed.is_some(),
                "leave/jail must remove exactly one active key"
            );
        }
        let total = roster
            .values()
            .try_fold(0_u64, |sum, (_, power)| sum.checked_add(*power))
            .context("acceptance power overflow")?;
        ensure!(
            roster.len() >= 2 && total > 0 && total <= (i64::MAX as u64) / 8,
            "acceptance transition leaves invalid active power"
        );
        transitions.insert(transition.action, native);
    }
    Ok(transitions)
}
