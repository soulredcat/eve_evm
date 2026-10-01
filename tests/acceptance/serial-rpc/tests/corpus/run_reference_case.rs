// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    decode_reference_transaction::decode_reference_transaction,
    materialize_state::materialize_state, reference_error_name::reference_error_name,
    types::FixtureCase,
};
use alloy_consensus::{Eip658Value, Receipt, ReceiptEnvelope};
use revm::{
    Context, ExecuteCommitEvm, ExecuteEvm, MainBuilder, MainContext,
    context_interface::{
        Transaction,
        result::{EVMError, ExecutionResult},
    },
    database::InMemoryDB,
    primitives::hardfork::SpecId,
};

pub fn run_reference_case(
    case: &FixtureCase,
) -> Result<(InMemoryDB, ExecutionResult, ReceiptEnvelope, u128), &'static str> {
    let (transaction, transaction_type) =
        decode_reference_transaction(&case.post["Shanghai"][0].txbytes)?;
    let effective_price = transaction.effective_gas_price(case.env.base_fee.to::<u128>());
    let context = Context::mainnet()
        .modify_cfg_chained(|cfg| {
            cfg.set_spec_and_mainnet_gas_params(SpecId::SHANGHAI);
            cfg.chain_id = case.config.chainid.to::<u64>();
        })
        .modify_block_chained(|block| {
            block.number = case.env.number;
            block.timestamp = case.env.timestamp;
            block.gas_limit = case.env.gas_limit.to::<u64>();
            block.basefee = case.env.base_fee.to::<u64>();
            block.beneficiary = case.env.coinbase;
            block.prevrandao = Some(case.env.random);
            block.difficulty = alloy_primitives::U256::ZERO;
            block.blob_excess_gas_and_price = None;
        })
        .with_db(materialize_state(&case.pre));
    let mut evm = context.build_mainnet();
    let completed = match evm.transact(transaction) {
        Ok(completed) => completed,
        Err(EVMError::Transaction(reason)) => return Err(reference_error_name(reason)),
        Err(other) => panic!("Reference infrastructure error: {other:?}"),
    };
    // CacheDB leaves deletion policy to its consumer. Apply Shanghai EIP-161
    // only to returned touched empty changes, preserving untouched prestate.
    let cleared: Vec<_> = completed
        .state
        .iter()
        .filter_map(|(address, account)| {
            (account.is_touched() && account.info.is_empty()).then_some(*address)
        })
        .collect();
    evm.commit(completed.state);
    for address in cleared {
        evm.ctx
            .journaled_state
            .database
            .cache
            .accounts
            .insert(address, revm::database::DbAccount::new_not_existing());
    }
    let execution = completed.result;
    let receipt = Receipt {
        status: Eip658Value::Eip658(execution.is_success()),
        cumulative_gas_used: execution.tx_gas_used(),
        logs: execution.logs().to_vec(),
    }
    .with_bloom();
    let envelope = match transaction_type {
        0 => ReceiptEnvelope::Legacy(receipt),
        1 => ReceiptEnvelope::Eip2930(receipt),
        2 => ReceiptEnvelope::Eip1559(receipt),
        _ => unreachable!(),
    };
    Ok((
        evm.ctx.journaled_state.database,
        execution,
        envelope,
        effective_price,
    ))
}
