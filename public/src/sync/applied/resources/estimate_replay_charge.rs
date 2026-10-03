// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::EstimatedReplayCharge;
use crate::sync::applied::AppliedError;
use eve_state::StateBudget;

/// Conservative local logical estimate for empty blocks, not an allocator/RSS theorem.
/// R=2MiB+512A+256S+256K+C+512N+2Y+128H+2L; D=4MiB+16L.
/// P=4W+2Bc+256(A+S+K+N+H); J=2Jb+512Jn+256(A+S).
/// Total=actual parent oracle estimate+2R+D+P+J. One R remains with captured state.
pub(in crate::sync::applied) fn estimate_replay_charge(
    budget: &StateBudget,
    payload_bytes: usize,
    oracle_bytes: usize,
) -> Result<EstimatedReplayCharge, AppliedError> {
    let terms = [
        (budget.maximum_accounts, 512),
        (budget.maximum_storage_slots, 256),
        (budget.maximum_codes, 256),
        (budget.maximum_total_code_bytes, 1),
        (budget.maximum_system_records, 512),
        (budget.maximum_system_bytes, 2),
        (budget.maximum_block_hashes, 128),
        (payload_bytes, 2),
    ];
    let retained = terms
        .into_iter()
        .try_fold(2_097_152_usize, |sum, (count, factor)| {
            sum.checked_add(count.checked_mul(factor)?)
        })
        .ok_or(AppliedError::ArithmeticOverflow)?;
    let counts = [
        budget.maximum_accounts,
        budget.maximum_storage_slots,
        budget.maximum_codes,
        budget.maximum_system_records,
        budget.maximum_block_hashes,
    ]
    .into_iter()
    .try_fold(0_usize, |sum, count| sum.checked_add(count))
    .ok_or(AppliedError::ArithmeticOverflow)?;
    let journal_sets = budget
        .maximum_accounts
        .checked_add(budget.maximum_storage_slots)
        .ok_or(AppliedError::ArithmeticOverflow)?;
    let terms = [
        (retained, 2),
        (payload_bytes, 16),
        (budget.maximum_state_bytes, 4),
        (budget.maximum_commit_bytes, 2),
        (counts, 256),
        (budget.maximum_journal_bytes, 2),
        (budget.maximum_journal_operations, 512),
        (journal_sets, 256),
    ];
    let initial = oracle_bytes
        .checked_add(4_194_304)
        .ok_or(AppliedError::ArithmeticOverflow)?;
    let total = terms
        .into_iter()
        .try_fold(initial, |sum, (count, factor)| {
            sum.checked_add(count.checked_mul(factor)?)
        })
        .ok_or(AppliedError::ArithmeticOverflow)?;
    Ok(EstimatedReplayCharge { retained, total })
}
