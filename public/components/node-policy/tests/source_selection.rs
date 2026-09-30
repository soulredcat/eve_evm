#[path = "support/mod.rs"]
mod support;

use eve_node_policy::{
    SourceEligibility, SourceObservation, SourcePurpose, SourceVerification, ZoneId,
    development_public_budget, evaluate_source, select_preferred_source, should_switch_source,
};

fn source(id: u8, height: u64, rtt: u64) -> SourceObservation {
    SourceObservation {
        zone: ZoneId(1),
        identity: [id; 32],
        binding: support::binding(),
        verification: SourceVerification::VerifiedByCaller,
        verified_height: height,
        independently_verified_head: Some(100),
        transport_identity_matched: true,
        service_rtt_micros: rtt,
        useful_bytes_per_second: 1_048_576,
        error_basis_points: 0,
    }
}

#[test]
fn freshness_and_verification_precede_speed_and_zone_never_grants_authority() {
    let budget = development_public_budget();
    let expected = support::binding();
    let valid = source(1, 100, 1_000);
    let stale = source(2, 90, 1);
    let mut forged = source(3, 100, 1);
    forged.verification = SourceVerification::Unverified;
    forged.zone = ZoneId(65_535);
    assert_eq!(
        evaluate_source(&expected, budget, &stale, SourcePurpose::HistoricalReplay),
        Ok(SourceEligibility::HistoricalOnly {
            freshness_unknown: false
        })
    );
    assert!(evaluate_source(&expected, budget, &stale, SourcePurpose::FreshHead).is_err());
    assert_eq!(
        select_preferred_source(
            budget,
            &[
                (&expected, &forged),
                (&expected, &stale),
                (&expected, &valid)
            ],
            SourcePurpose::FreshHead
        ),
        Ok([1; 32])
    );
    forged = valid.clone();
    forged.binding.evm_chain_id = 1;
    assert!(evaluate_source(&expected, budget, &forged, SourcePurpose::HistoricalReplay).is_err());
    forged = valid.clone();
    forged.error_basis_points = 1_001;
    assert!(evaluate_source(&expected, budget, &forged, SourcePurpose::FreshHead).is_err());
}

#[test]
fn uncertain_freshness_preserves_historical_replay_and_hold_down_is_bounded() {
    let budget = development_public_budget();
    let expected = support::binding();
    let current = source(1, 100, 1_000);
    let improved = source(2, 100, 800);
    assert!(!should_switch_source(
        budget,
        (&expected, &current),
        (&expected, &improved),
        SourcePurpose::FreshHead,
        29_999
    ));
    assert!(should_switch_source(
        budget,
        (&expected, &current),
        (&expected, &improved),
        SourcePurpose::FreshHead,
        30_000
    ));
    let weak_improvement = source(3, 100, 900);
    assert!(!should_switch_source(
        budget,
        (&expected, &current),
        (&expected, &weak_improvement),
        SourcePurpose::FreshHead,
        30_000
    ));
    let mut old = source(4, 50, 500);
    old.independently_verified_head = None;
    assert_eq!(
        evaluate_source(&expected, budget, &old, SourcePurpose::HistoricalReplay),
        Ok(SourceEligibility::HistoricalOnly {
            freshness_unknown: true
        })
    );
    assert!(evaluate_source(&expected, budget, &old, SourcePurpose::FreshHead).is_err());
    let candidates = vec![(&expected, &current); 33];
    assert!(select_preferred_source(budget, &candidates, SourcePurpose::FreshHead).is_err());
}
