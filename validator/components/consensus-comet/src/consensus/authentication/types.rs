// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

/// Required verification rule selected by authenticated network configuration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConsensusAuthenticationRequirement {
    ClassicalDev,
    ClassicalAndMldsa65,
}

/// The native engine cannot enforce two signature algorithms for one voter.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UnsupportedHybridConsensus;
