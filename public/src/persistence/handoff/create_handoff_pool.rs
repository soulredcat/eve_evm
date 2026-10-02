// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{HandoffError, HandoffPool, types::Accounting};
use eve_node_policy::{PublicBudget, validate_public_budget};
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

pub fn create_handoff_pool(budget: PublicBudget) -> Result<Arc<HandoffPool>, HandoffError> {
    validate_public_budget(budget).map_err(|_| HandoffError::InvalidBudget)?;
    Ok(Arc::new(HandoffPool {
        budget,
        accounting: Mutex::new(Accounting {
            bytes: 0,
            next_id: 0,
            leases: BTreeMap::new(),
        }),
    }))
}
