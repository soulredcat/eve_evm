// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::AcceptanceFixture;
use alloy_primitives::Address;
use anyhow::Result;
use std::collections::BTreeMap;

pub(crate) fn extend_acceptance_proposer_owners(
    _fixture: Option<&AcceptanceFixture>,
    _owners: &mut BTreeMap<[u8; 20], Address>,
) -> Result<()> {
    Ok(())
}
