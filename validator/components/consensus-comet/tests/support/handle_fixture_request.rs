// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::{fs::File, io::Write, path::Path};

use anyhow::{Context, Result, ensure};
use eve_consensus_comet::wire::tendermint::abci::{self as abci, request, response};
use sha2::{Digest, Sha256};

use super::{API_TRANSACTION, FixtureState};

/// An API-contract fixture only: its hash is not EVE's application commitment.
pub fn handle_fixture_request(
    request: abci::Request,
    state: &mut FixtureState,
    checkpoint: &Path,
) -> Result<abci::Response> {
    let value = match request.value.context("empty ABCI request")? {
        request::Value::Echo(request) => response::Value::Echo(abci::ResponseEcho {
            message: request.message,
        }),
        request::Value::Flush(_) => response::Value::Flush(abci::ResponseFlush {}),
        request::Value::Info(_) => {
            state.operations.insert("Info");
            response::Value::Info(abci::ResponseInfo {
                data: "EVE_B0_API_FIXTURE_NOT_EVM".into(),
                version: "1".into(),
                app_version: 1,
                last_block_height: state.committed_height,
                last_block_app_hash: state.committed_hash.clone(),
            })
        }
        request::Value::InitChain(request) => {
            state.operations.insert("InitChain");
            ensure!(
                state.committed_height == 0,
                "InitChain after committed fixture state"
            );
            state.initial_validators = request.validators;
            response::Value::InitChain(abci::ResponseInitChain {
                app_hash: state.committed_hash.clone(),
                ..Default::default()
            })
        }
        request::Value::CheckTx(request) => {
            state.operations.insert("CheckTx");
            response::Value::CheckTx(abci::ResponseCheckTx {
                code: u32::from(request.tx != API_TRANSACTION),
                gas_wanted: 1,
                ..Default::default()
            })
        }
        request::Value::PrepareProposal(request) => {
            state.operations.insert("PrepareProposal");
            let mut remaining = request.max_tx_bytes;
            let txs = request
                .txs
                .into_iter()
                .filter(|tx| {
                    let size = i64::try_from(tx.len()).unwrap_or(i64::MAX);
                    if tx == API_TRANSACTION && size <= remaining {
                        remaining -= size;
                        true
                    } else {
                        false
                    }
                })
                .collect();
            response::Value::PrepareProposal(abci::ResponsePrepareProposal { txs })
        }
        request::Value::ProcessProposal(request) => {
            state.operations.insert("ProcessProposal");
            let valid = request.height == state.committed_height + 1
                && request.txs.iter().all(|tx| tx == API_TRANSACTION);
            response::Value::ProcessProposal(abci::ResponseProcessProposal {
                status: if valid { 1 } else { 2 },
            })
        }
        request::Value::FinalizeBlock(request) => {
            state.operations.insert("FinalizeBlock");
            ensure!(
                request.height == state.committed_height + 1,
                "fixture height mismatch"
            );
            ensure!(
                request.txs.iter().all(|tx| tx == API_TRANSACTION),
                "invalid fixture tx"
            );
            let mut hasher = Sha256::new();
            hasher.update(b"EVE_B0_API_FIXTURE_NOT_EVM_V1");
            hasher.update(request.height.to_be_bytes());
            hasher.update(&state.committed_hash);
            for transaction in &request.txs {
                hasher.update((transaction.len() as u64).to_be_bytes());
                hasher.update(transaction);
            }
            let hash = hasher.finalize().to_vec();
            state.pending = Some((request.height, hash.clone()));
            state.finalized_hashes.insert(request.height, hash.clone());
            state.transaction_count += request.txs.len();
            if let Some(commit) = request.decided_last_commit {
                let power = commit
                    .votes
                    .iter()
                    .filter_map(|vote| vote.validator.as_ref())
                    .map(|validator| validator.power)
                    .sum();
                state.last_commit_power.insert(request.height, power);
            }
            let validator_updates = if request.height == 2 {
                state
                    .initial_validators
                    .iter()
                    .cloned()
                    .map(|mut validator| {
                        validator.power = 20;
                        validator
                    })
                    .collect()
            } else {
                Vec::new()
            };
            response::Value::FinalizeBlock(abci::ResponseFinalizeBlock {
                tx_results: request
                    .txs
                    .iter()
                    .map(|_| abci::ExecTxResult {
                        gas_wanted: 1,
                        gas_used: 1,
                        ..Default::default()
                    })
                    .collect(),
                validator_updates,
                app_hash: hash,
                ..Default::default()
            })
        }
        request::Value::Commit(_) => {
            state.operations.insert("Commit");
            let (height, hash) = state
                .pending
                .take()
                .context("Commit without FinalizeBlock")?;
            let staged = checkpoint.with_extension("staged");
            let mut file = File::create(&staged)?;
            file.write_all(&serde_json::to_vec(&serde_json::json!({
                "height": height, "hash": hex::encode(&hash)
            }))?)?;
            file.sync_all()?;
            drop(file);
            std::fs::rename(staged, checkpoint)?;
            state.committed_height = height;
            state.committed_hash = hash;
            response::Value::Commit(abci::ResponseCommit { retain_height: 0 })
        }
        request::Value::Query(_) => response::Value::Query(abci::ResponseQuery {
            height: state.committed_height,
            ..Default::default()
        }),
        request::Value::ListSnapshots(_) => {
            response::Value::ListSnapshots(abci::ResponseListSnapshots::default())
        }
        _ => response::Value::Exception(abci::ResponseException {
            error: "unsupported B0 API-test operation".into(),
        }),
    };
    Ok(abci::Response { value: Some(value) })
}
