// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_protocol_config::network::NetworkProfileBinding;

use crate::{PublicBudget, SourceObservation, SourcePurpose, evaluate_source};

/// Caller supplies monotonic local elapsed time. This affects routing only.
pub fn should_switch_source(
    budget: PublicBudget,
    current: (&NetworkProfileBinding, &SourceObservation),
    proposed: (&NetworkProfileBinding, &SourceObservation),
    purpose: SourcePurpose,
    elapsed_since_switch_ms: u64,
) -> bool {
    if evaluate_source(proposed.0, budget, proposed.1, purpose).is_err() {
        return false;
    }
    if evaluate_source(current.0, budget, current.1, purpose).is_err() {
        return true;
    }
    if current.1.identity == proposed.1.identity
        || elapsed_since_switch_ms < budget.switch_cooldown_ms
    {
        return false;
    }
    if proposed.1.verified_height > current.1.verified_height {
        return true;
    }
    if proposed.1.verified_height < current.1.verified_height
        || proposed.1.error_basis_points > current.1.error_basis_points
    {
        return false;
    }
    let improvement = current
        .1
        .service_rtt_micros
        .saturating_sub(proposed.1.service_rtt_micros);
    u128::from(improvement) * 10_000
        >= u128::from(current.1.service_rtt_micros)
            * u128::from(budget.switch_improvement_basis_points)
}
