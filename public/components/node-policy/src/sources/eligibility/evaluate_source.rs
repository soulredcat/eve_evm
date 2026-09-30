use eve_protocol_config::network::{NetworkProfileBinding, validate_network_profile};

use crate::{
    PublicBudget, SourceEligibility, SourceObservation, SourcePurpose, SourceVerification,
    validate_public_budget,
};

pub fn evaluate_source(
    expected_at_height: &NetworkProfileBinding,
    budget: PublicBudget,
    observation: &SourceObservation,
    purpose: SourcePurpose,
) -> Result<SourceEligibility, &'static str> {
    validate_public_budget(budget).map_err(|_| "invalid source budget")?;
    validate_network_profile(expected_at_height, &observation.binding)
        .map_err(|_| "wrong or unsupported network/profile")?;
    if !observation.transport_identity_matched
        || observation.verification != SourceVerification::VerifiedByCaller
        || observation.service_rtt_micros == 0
        || observation.useful_bytes_per_second == 0
        || u64::from(observation.error_basis_points) > budget.maximum_source_error_basis_points
    {
        return Err("source is not verified and healthy");
    }
    let Some(head) = observation.independently_verified_head else {
        return if purpose == SourcePurpose::HistoricalReplay {
            Ok(SourceEligibility::HistoricalOnly {
                freshness_unknown: true,
            })
        } else {
            Err("head freshness unknown")
        };
    };
    if head < observation.verified_height {
        return Err("source height exceeds corroborated history");
    }
    if head - observation.verified_height > budget.maximum_head_lag_blocks {
        return if purpose == SourcePurpose::HistoricalReplay {
            Ok(SourceEligibility::HistoricalOnly {
                freshness_unknown: false,
            })
        } else {
            Err("source is too stale for fresh head")
        };
    }
    Ok(SourceEligibility::FreshHead)
}
