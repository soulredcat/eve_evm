use super::CompleteExecutionError;
use crate::FeeAllocation;
use alloy_primitives::Bytes;
use eve_state::hash_system_key;
use eve_state::{CompleteState, StateError, SystemNamespace, SystemRecord, SystemValue};

pub(crate) fn update_fee_ledger(
    state: &mut CompleteState,
    allocation: FeeAllocation,
) -> Result<(), CompleteExecutionError> {
    let key = hash_system_key(SystemNamespace::Fee, b"pool")
        .map_err(|error| CompleteExecutionError::State(StateError::SystemRecord(error)))?;
    let record = state
        .system
        .get(&key)
        .ok_or(CompleteExecutionError::State(StateError::CommitMismatch))?;
    let SystemValue::Fee {
        burned,
        node_pool,
        validator_pool,
    } = record.value
    else {
        return Err(CompleteExecutionError::State(StateError::CommitMismatch));
    };
    let overflow = CompleteExecutionError::State(StateError::ArithmeticOverflow);
    let burned = burned
        .checked_add(allocation.burn)
        .ok_or_else(|| overflow.clone())?;
    let node_pool = node_pool
        .checked_add(allocation.node_pool)
        .ok_or_else(|| overflow.clone())?;
    let validator_pool = validator_pool
        .checked_add(allocation.validator_pool)
        .ok_or(overflow)?;
    state.system.insert(
        key,
        SystemRecord {
            schema_version: 1,
            namespace: SystemNamespace::Fee,
            logical_key: Bytes::from_static(b"pool"),
            value: SystemValue::Fee {
                burned,
                node_pool,
                validator_pool,
            },
        },
    );
    Ok(())
}
