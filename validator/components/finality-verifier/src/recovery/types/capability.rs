// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::{collections::BTreeMap, sync::Arc};

use crate::{
    AuthenticatedApplicationAnchor, DevelopmentFinalityVerifier, VerifiedDevelopmentHeader,
};
use eve_consensus_comet::consensus::certificates::ClassicalValidator;
use eve_state::{Address, StateCommit};

use super::envelope::{CompactRecoveryEnvelopeV1, NativeDataFrame};

/// Immutable local genesis or replayed state; no durable/fresh-head assertion.
#[derive(Clone, Debug)]
pub struct DevelopmentRecoveryState {
    pub(in crate::recovery) commit: Arc<StateCommit>,
    pub(in crate::recovery) finality: DevelopmentFinalityVerifier,
    pub(in crate::recovery) policy: Arc<FixedGenesisPolicy>,
    pub(in crate::recovery) lookahead: Option<Arc<NativeDataFrame>>,
    pub(in crate::recovery) lookahead_header: Option<VerifiedDevelopmentHeader>,
    pub(in crate::recovery) anchor: Option<AuthenticatedApplicationAnchor>,
}

/// A prepared immutable transition retains the exact compact record it verified.
/// Public owns actual resource reservation, admission, publication and durability.
#[derive(Debug)]
pub struct VerifiedRecoveryTransition {
    pub(in crate::recovery) state: Arc<DevelopmentRecoveryState>,
    pub(in crate::recovery) envelope: Arc<CompactRecoveryEnvelopeV1>,
}

/// This slice accepts no dynamic set, owner or key-epoch evolution.
#[derive(Clone, Debug)]
pub(in crate::recovery) struct FixedGenesisPolicy {
    pub(in crate::recovery) validators: Vec<ClassicalValidator>,
    pub(in crate::recovery) validator_hash: [u8; 32],
    pub(in crate::recovery) proposer_owners: BTreeMap<[u8; 20], Address>,
}

pub(in crate::recovery) struct PreparedRecoveryHistory {
    pub(in crate::recovery) finality: DevelopmentFinalityVerifier,
    pub(in crate::recovery) finalized: VerifiedDevelopmentHeader,
    pub(in crate::recovery) lookahead: VerifiedDevelopmentHeader,
}
