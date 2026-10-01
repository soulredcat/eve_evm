// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    BlockEnvironment, BlockExecutionError, BlockOutcome, TransactionOutcome,
    validate_block_environment,
};
use crate::{
    execution::serial::{
        fees::{credit_fee_pools, handler::EveFeeHandler, split_collected_fees},
        receipts::build_receipt,
        roots::compute_state_root,
    },
    transactions::signed::decode_signed_transaction,
};
use alloy_eips::eip2718::Encodable2718;
use alloy_primitives::{Bytes, U256};
use alloy_trie::root::ordered_trie_root_encoded;
use core::{convert::Infallible, marker::PhantomData};
use revm::{
    Context, ExecuteEvm, MainBuilder, MainContext,
    context_interface::{Transaction, result::EVMError},
    database::InMemoryDB,
    handler::Handler,
    primitives::hardfork::SpecId,
};

/// Execute an entire candidate block on an isolated complete-state overlay.
pub fn execute_serial_block(
    parent: &InMemoryDB,
    block: &BlockEnvironment,
    raw_transactions: &[Bytes],
) -> Result<BlockOutcome, BlockExecutionError> {
    validate_block_environment(block)?;
    let context = Context::mainnet()
        .modify_cfg_chained(|cfg| {
            cfg.set_spec_and_mainnet_gas_params(SpecId::SHANGHAI);
            cfg.chain_id = block.chain_id;
        })
        .modify_block_chained(|environment| {
            super::populate_block_environment::populate_block_environment(environment, block)
        })
        .with_db(parent.clone());
    let mut evm = context
        .build_mainnet()
        .with_precompiles(crate::execution::native::inactive_native_provider());
    let mut handler: EveFeeHandler<_, EVMError<Infallible>, _> = EveFeeHandler {
        marker: PhantomData,
    };
    let mut outcomes = Vec::with_capacity(raw_transactions.len());
    let mut receipts = Vec::with_capacity(raw_transactions.len());
    let mut gas_used = 0_u64;
    let mut collected = U256::ZERO;
    for (index, raw) in raw_transactions.iter().enumerate() {
        let transaction =
            decode_signed_transaction(raw, block.chain_id, block.maximum_transaction_bytes)
                .map_err(|reason| BlockExecutionError::InvalidTransaction { index, reason })?;
        if transaction.evm.gas_limit > block.gas_limit - gas_used {
            return Err(BlockExecutionError::BlockGasLimit { index });
        }
        let effective_gas_price = transaction.evm.effective_gas_price(block.base_fee.into());
        evm.ctx.tx = transaction.evm;
        let execution = handler
            .run(&mut evm)
            .map_err(|error| BlockExecutionError::Execution {
                index,
                message: format!("{error:?}"),
            })?;
        gas_used = gas_used
            .checked_add(execution.tx_gas_used())
            .ok_or(BlockExecutionError::ArithmeticOverflow)?;
        let fee = U256::from(execution.tx_gas_used())
            .checked_mul(U256::from(effective_gas_price))
            .ok_or(BlockExecutionError::ArithmeticOverflow)?;
        collected = collected
            .checked_add(fee)
            .ok_or(BlockExecutionError::ArithmeticOverflow)?;
        receipts.push(build_receipt(
            transaction.transaction_type,
            &execution,
            gas_used,
        ));
        let changes = evm.finalize();
        super::super::state::commit_shanghai_changes(
            &mut evm.ctx.journaled_state.database,
            changes,
        );
        outcomes.push(TransactionOutcome {
            hash: transaction.hash,
            sender: transaction.sender,
            effective_gas_price,
            execution,
        });
    }
    let mut state = evm.ctx.journaled_state.database;
    let fees = split_collected_fees(collected);
    credit_fee_pools(&mut state, block.fee_pools, fees)?;
    let encoded_receipts: Vec<_> = receipts.iter().map(Encodable2718::encoded_2718).collect();
    Ok(BlockOutcome {
        state_root: compute_state_root(&state),
        transactions_root: ordered_trie_root_encoded(raw_transactions),
        receipts_root: ordered_trie_root_encoded(&encoded_receipts),
        state,
        transactions: raw_transactions.to_vec(),
        outcomes,
        receipts,
        gas_used,
        fees,
    })
}
