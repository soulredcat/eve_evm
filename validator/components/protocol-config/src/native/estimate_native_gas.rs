use super::{NATIVE_GAS_V1, NativeGasInput, NativeGasSchedule, NativeMeterError};

/// Native execution charge, additional to ordinary EVM transaction intrinsic gas.
/// No refunds or host-time measurements enter the calculation.
pub fn estimate_native_gas(
    schedule: NativeGasSchedule,
    input: NativeGasInput,
) -> Result<u64, NativeMeterError> {
    if schedule != NATIVE_GAS_V1 {
        return Err(NativeMeterError::UnsupportedSchedule);
    }
    let calldata = input
        .zero_calldata_bytes
        .checked_add(input.nonzero_calldata_bytes)
        .ok_or(NativeMeterError::ArithmeticOverflow)?;
    let signatures = input
        .classical_verifications
        .checked_add(input.mldsa65_verifications)
        .ok_or(NativeMeterError::ArithmeticOverflow)?;
    let storage = input
        .storage_reads
        .checked_add(input.new_storage_writes)
        .and_then(|value| value.checked_add(input.existing_storage_writes))
        .ok_or(NativeMeterError::ArithmeticOverflow)?;
    if calldata > schedule.maximum_calldata_bytes
        || input.proof_bytes > schedule.maximum_proof_bytes
        || signatures > schedule.maximum_signature_checks
        || storage > schedule.maximum_storage_operations
    {
        return Err(NativeMeterError::LimitExceeded);
    }
    [
        (input.zero_calldata_bytes, schedule.calldata_zero_byte),
        (input.nonzero_calldata_bytes, schedule.calldata_nonzero_byte),
        (input.proof_bytes, schedule.proof_byte),
        (
            input.classical_verifications,
            schedule.classical_verification,
        ),
        (input.mldsa65_verifications, schedule.mldsa65_verification),
        (input.storage_reads, schedule.storage_read),
        (input.new_storage_writes, schedule.storage_new_write),
        (
            input.existing_storage_writes,
            schedule.storage_existing_write,
        ),
    ]
    .into_iter()
    .try_fold(schedule.dispatch, |total, (count, price)| {
        count
            .checked_mul(price)
            .and_then(|cost| total.checked_add(cost))
            .ok_or(NativeMeterError::ArithmeticOverflow)
    })
}
