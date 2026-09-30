use eve_protocol_config::network::NetworkProfileBinding;

use crate::{PublicBudget, SourceObservation, SourcePurpose, evaluate_source};

/// Integer routing metrics only; no metric grants voting power or authenticates state.
pub fn select_preferred_source(
    budget: PublicBudget,
    candidates: &[(&NetworkProfileBinding, &SourceObservation)],
    purpose: SourcePurpose,
) -> Result<[u8; 32], &'static str> {
    if candidates.len() as u64 > budget.probe_candidates {
        return Err("candidate budget exceeded");
    }
    let mut eligible = candidates
        .iter()
        .filter_map(|(expected, source)| {
            evaluate_source(expected, budget, source, purpose)
                .ok()
                .map(|_| *source)
        })
        .collect::<Vec<_>>();
    eligible.sort_by(|left, right| {
        right
            .verified_height
            .cmp(&left.verified_height)
            .then(left.error_basis_points.cmp(&right.error_basis_points))
            .then(left.service_rtt_micros.cmp(&right.service_rtt_micros))
            .then(
                right
                    .useful_bytes_per_second
                    .cmp(&left.useful_bytes_per_second),
            )
            .then(left.identity.cmp(&right.identity))
    });
    eligible
        .first()
        .map(|source| source.identity)
        .ok_or("no eligible source")
}
