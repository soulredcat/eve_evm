// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::approval::{ExecutionApproval, approved_state_block};
use anyhow::{Result, ensure};
use eve_evm::estimate_clone_reservation;
use eve_state::{JournalOperation, StateBudget, measure_complete_state_bytes};

/// Conservative logical retention accounting, explicitly not an allocator/RSS cap.
pub(in crate::consensus::application) fn approval_logical_charge(
    approval: &ExecutionApproval,
    budget: &StateBudget,
) -> Result<usize> {
    let prepared = approved_state_block(approval);
    let canonical = measure_complete_state_bytes(&prepared.commit.state, budget)
        .map_err(|_| anyhow::anyhow!("candidate canonical state byte limit"))?;
    let mut charge = estimate_clone_reservation(&prepared.commit.state)
        .map_err(|_| anyhow::anyhow!("candidate retention estimate overflow"))?
        .checked_add(canonical)
        .ok_or_else(|| anyhow::anyhow!("candidate retention overflow"))?;
    for bytes in prepared
        .commit
        .block
        .transactions
        .iter()
        .chain(&prepared.commit.block.receipts)
    {
        charge = charge
            .checked_add(
                bytes
                    .len()
                    .checked_mul(3)
                    .ok_or_else(|| anyhow::anyhow!("candidate byte overflow"))?,
            )
            .ok_or_else(|| anyhow::anyhow!("candidate retention overflow"))?;
    }
    charge = charge
        .checked_add(
            prepared
                .journal
                .operations
                .len()
                .checked_mul(512)
                .ok_or_else(|| anyhow::anyhow!("candidate journal overhead overflow"))?,
        )
        .ok_or_else(|| anyhow::anyhow!("candidate retention overflow"))?;
    for operation in &prepared.journal.operations {
        if let JournalOperation::PutCode { code, .. } = operation {
            charge = charge
                .checked_add(code.len())
                .ok_or_else(|| anyhow::anyhow!("candidate code retention overflow"))?;
        }
    }
    for outcome in &prepared.outcomes {
        let output = outcome.execution.output().map_or(0, |bytes| bytes.len());
        charge = charge
            .checked_add(output)
            .ok_or_else(|| anyhow::anyhow!("candidate output retention overflow"))?;
        for log in outcome.execution.logs() {
            let bytes = log
                .data
                .data
                .len()
                .checked_add(
                    log.data
                        .topics()
                        .len()
                        .checked_mul(32)
                        .ok_or_else(|| anyhow::anyhow!("candidate log retention overflow"))?,
                )
                .ok_or_else(|| anyhow::anyhow!("candidate log retention overflow"))?;
            charge = charge
                .checked_add(bytes)
                .ok_or_else(|| anyhow::anyhow!("candidate log retention overflow"))?;
        }
    }
    ensure!(
        charge <= usize::MAX / 2,
        "candidate retention bound overflow"
    );
    Ok(charge)
}
