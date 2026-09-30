use super::{encode_identity, encode_list};
use crate::{CompleteState, StateError};
use eve_protocol_config::records::{ExecutionBlockHash, encode_system_record};

pub(crate) fn encode_state_data_with_history(
    state: &CompleteState,
    current: Option<(u64, ExecutionBlockHash)>,
) -> Result<Vec<u8>, StateError> {
    let accounts = state
        .accounts
        .iter()
        .map(|(address, account)| {
            let slots = account
                .storage
                .iter()
                .map(|(key, value)| {
                    encode_list(&[alloy_rlp::encode(*key), alloy_rlp::encode(*value)])
                })
                .collect::<Vec<_>>();
            encode_list(&[
                alloy_rlp::encode(*address),
                alloy_rlp::encode(account.nonce),
                alloy_rlp::encode(account.balance),
                alloy_rlp::encode(account.code_hash),
                encode_list(&slots),
            ])
        })
        .collect::<Vec<_>>();
    let codes = state
        .codes
        .iter()
        .map(|(hash, code)| {
            encode_list(&[alloy_rlp::encode(*hash), alloy_rlp::encode(code.as_ref())])
        })
        .collect::<Vec<_>>();
    let system = state
        .system
        .iter()
        .map(|(key, record)| {
            Ok(encode_list(&[
                alloy_rlp::encode(*key),
                encode_system_record(record).map_err(StateError::SystemRecord)?,
            ]))
        })
        .collect::<Result<Vec<_>, StateError>>()?;
    let mut hashes = state.block_hashes.clone();
    if let Some((height, hash)) = current {
        hashes.insert(height, hash);
    }
    let history = hashes
        .iter()
        .map(|(height, hash)| encode_list(&[alloy_rlp::encode(*height), alloy_rlp::encode(hash.0)]))
        .collect::<Vec<_>>();
    Ok(encode_list(&[
        encode_identity(&state.identity),
        encode_list(&accounts),
        encode_list(&codes),
        encode_list(&system),
        encode_list(&history),
    ]))
}
