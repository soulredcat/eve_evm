// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_protocol_config::network::NetworkProfileBinding;

/// Operational placement metadata; never included in voting or genesis identity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ZoneId(pub u16);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SourcePurpose {
    HistoricalReplay,
    FreshHead,
}

/// Caller-provided verification metadata, not an authenticated capability.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SourceVerification {
    Unverified,
    VerifiedByCaller,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceObservation {
    pub zone: ZoneId,
    pub identity: [u8; 32],
    pub binding: NetworkProfileBinding,
    pub verification: SourceVerification,
    pub verified_height: u64,
    pub independently_verified_head: Option<u64>,
    pub transport_identity_matched: bool,
    pub service_rtt_micros: u64,
    pub useful_bytes_per_second: u64,
    pub error_basis_points: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SourceEligibility {
    HistoricalOnly { freshness_unknown: bool },
    FreshHead,
}
