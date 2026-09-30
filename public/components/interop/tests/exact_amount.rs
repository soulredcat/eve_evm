use eve_interop::{InteropError, U256, convert_exact_amount};

#[test]
fn ti06_exact_18_and_9_decimal_conversion_matches_base_units() {
    let one_eth = U256::from(1_000_000_000_000_000_000u64);
    let one_sol_scale = U256::from(1_000_000_000u64);
    assert_eq!(
        convert_exact_amount(one_eth, 18, 9, U256::from(u64::MAX)),
        Ok(one_sol_scale)
    );
    assert_eq!(
        convert_exact_amount(one_sol_scale, 9, 18, U256::MAX),
        Ok(one_eth)
    );
    assert_eq!(
        convert_exact_amount(U256::from(42), 0, 0, U256::from(42)),
        Ok(U256::from(42))
    );
}

#[test]
fn ti06_zero_dust_and_invalid_decimals_fail_closed() {
    assert_eq!(
        convert_exact_amount(U256::ZERO, 18, 9, U256::MAX),
        Err(InteropError::ZeroAmount)
    );
    assert_eq!(
        convert_exact_amount(U256::from(1_000_000_001u64), 18, 9, U256::MAX),
        Err(InteropError::NonExactAmount)
    );
    assert_eq!(
        convert_exact_amount(U256::from(1), 78, 0, U256::MAX),
        Err(InteropError::InvalidDecimals)
    );
    assert_eq!(
        convert_exact_amount(U256::from(1), 0, 255, U256::MAX),
        Err(InteropError::InvalidDecimals)
    );
}

#[test]
fn ti06_uint256_and_solana_uint64_bounds_are_enforced() {
    assert_eq!(
        convert_exact_amount(U256::MAX, 0, 1, U256::MAX),
        Err(InteropError::AmountOverflow)
    );
    assert_eq!(
        convert_exact_amount(
            U256::from(u64::MAX) + U256::from(1),
            9,
            9,
            U256::from(u64::MAX)
        ),
        Err(InteropError::AmountOverflow)
    );
    assert_eq!(
        convert_exact_amount(U256::from(u64::MAX), 9, 9, U256::from(u64::MAX)),
        Ok(U256::from(u64::MAX))
    );
    assert_eq!(
        convert_exact_amount(U256::from(1), 0, 0, U256::ZERO),
        Err(InteropError::AmountOverflow)
    );
}

#[test]
fn ti06_decimal_limit_and_exact_roundtrip_hold_for_every_supported_scale() {
    let mut ten_power = U256::from(1);
    for decimals in 0..=77 {
        assert_eq!(
            convert_exact_amount(U256::from(1), 0, decimals, U256::MAX),
            Ok(ten_power)
        );
        assert_eq!(
            convert_exact_amount(ten_power, decimals, 0, U256::MAX),
            Ok(U256::from(1))
        );
        if decimals < 77 {
            ten_power = ten_power.checked_mul(U256::from(10)).unwrap();
        }
    }
}
