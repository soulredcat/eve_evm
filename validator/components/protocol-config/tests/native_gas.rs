use eve_protocol_config::native::{
    NATIVE_GAS_V1, NativeGasInput, NativeMeterError, estimate_native_gas,
};

fn registration() -> NativeGasInput {
    NativeGasInput {
        zero_calldata_bytes: 64,
        nonzero_calldata_bytes: 128,
        proof_bytes: 32,
        classical_verifications: 1,
        mldsa65_verifications: 0,
        storage_reads: 2,
        new_storage_writes: 3,
        existing_storage_writes: 1,
    }
}

#[test]
fn freezes_worked_registration_and_hybrid_metering_vectors() {
    assert_eq!(
        estimate_native_gas(NATIVE_GAS_V1, registration()),
        Ok(80_016)
    );
    let mut hybrid = registration();
    hybrid.mldsa65_verifications = 1;
    hybrid.proof_bytes = 3_309;
    assert_eq!(estimate_native_gas(NATIVE_GAS_V1, hybrid), Ok(232_448));
}

#[test]
fn rejects_unregistered_schedule_size_storm_and_arithmetic_overflow() {
    let mut schedule = NATIVE_GAS_V1;
    schedule.mldsa65_verification = 1;
    assert_eq!(
        estimate_native_gas(schedule, registration()),
        Err(NativeMeterError::UnsupportedSchedule)
    );
    let mut input = registration();
    input.proof_bytes = 65_537;
    assert_eq!(
        estimate_native_gas(NATIVE_GAS_V1, input),
        Err(NativeMeterError::LimitExceeded)
    );
    input = registration();
    input.classical_verifications = 65;
    assert_eq!(
        estimate_native_gas(NATIVE_GAS_V1, input),
        Err(NativeMeterError::LimitExceeded)
    );
    input = registration();
    input.zero_calldata_bytes = u64::MAX;
    assert_eq!(
        estimate_native_gas(NATIVE_GAS_V1, input),
        Err(NativeMeterError::ArithmeticOverflow)
    );
}
