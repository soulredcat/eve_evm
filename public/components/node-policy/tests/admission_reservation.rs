use eve_node_policy::{development_public_budget, validate_admission_reservation};

#[test]
fn global_bulk_and_ordinary_reservations_bound_total_payload_ram() {
    let budget = development_public_budget();
    let mib = 1_048_576;
    assert_eq!(
        validate_admission_reservation(budget, true, 0, 32 * mib),
        Ok(32 * mib)
    );
    assert_eq!(
        validate_admission_reservation(budget, true, 32 * mib, 32 * mib),
        Ok(64 * mib)
    );
    assert!(validate_admission_reservation(budget, true, 64 * mib, 1).is_err());
    assert!(validate_admission_reservation(budget, false, 16 * mib, 1).is_err());
    assert!(validate_admission_reservation(budget, false, 0, mib + 1).is_err());
    assert!(validate_admission_reservation(budget, true, u64::MAX, 1).is_err());
}
