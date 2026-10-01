// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::state::encoding::{encode_list, encode_version};
use crate::{JournalOperation, StateBudget, StateError, StateJournal};
use eve_protocol_config::records::encode_system_record;

pub fn encode_state_journal(
    journal: &StateJournal,
    budget: &StateBudget,
) -> Result<Vec<u8>, StateError> {
    super::validate_journal_budget::validate_journal_budget(journal, budget)?;
    let operations = journal
        .operations
        .iter()
        .map(|operation| {
            Ok(match operation {
                JournalOperation::PutAccount {
                    address,
                    nonce,
                    balance,
                    code_hash,
                } => encode_list(&[
                    alloy_rlp::encode(1_u8),
                    alloy_rlp::encode(*address),
                    alloy_rlp::encode(*nonce),
                    alloy_rlp::encode(*balance),
                    alloy_rlp::encode(*code_hash),
                ]),
                JournalOperation::DeleteAccount { address } => {
                    encode_list(&[alloy_rlp::encode(2_u8), alloy_rlp::encode(*address)])
                }
                JournalOperation::PutStorage {
                    address,
                    slot,
                    value,
                } => encode_list(&[
                    alloy_rlp::encode(3_u8),
                    alloy_rlp::encode(*address),
                    alloy_rlp::encode(*slot),
                    alloy_rlp::encode(*value),
                ]),
                JournalOperation::DeleteStorage { address, slot } => encode_list(&[
                    alloy_rlp::encode(4_u8),
                    alloy_rlp::encode(*address),
                    alloy_rlp::encode(*slot),
                ]),
                JournalOperation::ClearStorage { address } => {
                    encode_list(&[alloy_rlp::encode(5_u8), alloy_rlp::encode(*address)])
                }
                JournalOperation::PutCode { code_hash, code } => encode_list(&[
                    alloy_rlp::encode(6_u8),
                    alloy_rlp::encode(*code_hash),
                    alloy_rlp::encode(code.as_ref()),
                ]),
                JournalOperation::DeleteCode { code_hash } => {
                    encode_list(&[alloy_rlp::encode(7_u8), alloy_rlp::encode(*code_hash)])
                }
                JournalOperation::PutSystem { key, record } => encode_list(&[
                    alloy_rlp::encode(8_u8),
                    alloy_rlp::encode(*key),
                    encode_system_record(record).map_err(StateError::SystemRecord)?,
                ]),
                JournalOperation::DeleteSystem { key } => {
                    encode_list(&[alloy_rlp::encode(9_u8), alloy_rlp::encode(*key)])
                }
                JournalOperation::SetExecutionBlockHash { height, hash } => encode_list(&[
                    alloy_rlp::encode(10_u8),
                    alloy_rlp::encode(*height),
                    alloy_rlp::encode(hash.0),
                ]),
            })
        })
        .collect::<Result<Vec<_>, StateError>>()?;
    Ok(encode_list(&[
        alloy_rlp::encode(b"EVE_STATE_JOURNAL_V1".as_slice()),
        encode_version(&journal.parent),
        alloy_rlp::encode(journal.target_height),
        encode_list(&operations),
    ]))
}
