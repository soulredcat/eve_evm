// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use alloy_primitives::Address;
use eve_consensus_comet::wire::tendermint::abci::ValidatorUpdate;
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct FixtureInput {
    pub version: u8,
    pub contract_address: Address,
    pub authority: Address,
    pub future_validators: Vec<FutureValidator>,
    pub transitions: Vec<Transition>,
    pub poison: Option<PoisonInput>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct FutureValidator {
    pub public_key: String,
    pub owner: Address,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Transition {
    pub action: u8,
    pub kind: TransitionKind,
    pub updates: Vec<UpdateInput>,
}

#[derive(Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub(super) enum TransitionKind {
    Rotate,
    Leave,
    Jail,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct UpdateInput {
    pub public_key: String,
    pub power: u64,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct PoisonInput {
    pub height: i64,
    pub public_key: String,
}

/// Only the bounded genesis-pinned loader constructs this validated capability.
pub(crate) struct AcceptanceFixture {
    pub(super) rejection_recorded: std::sync::atomic::AtomicBool,
    pub(super) digest: [u8; 32],
    pub(super) address: Address,
    pub(super) authority: Address,
    pub(super) future: BTreeMap<[u8; 32], Address>,
    pub(super) transitions: BTreeMap<u8, Vec<ValidatorUpdate>>,
    pub(super) poison: Option<(i64, [u8; 20])>,
}

pub(super) const TRAILER_TAG: &[u8] = b"EVE_B3_ACCEPTANCE_V1";
