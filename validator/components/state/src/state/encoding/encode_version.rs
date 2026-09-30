use super::{encode_identity, encode_list};
use crate::StateVersion;

pub(crate) fn encode_version(version: &StateVersion) -> Vec<u8> {
    let application = match version.application {
        None => encode_list(&[alloy_rlp::encode(0_u8)]),
        Some(commitment) => {
            encode_list(&[alloy_rlp::encode(1_u8), alloy_rlp::encode(commitment.0)])
        }
    };
    encode_list(&[
        encode_identity(&version.identity),
        alloy_rlp::encode(version.height),
        alloy_rlp::encode(version.timestamp),
        alloy_rlp::encode(version.execution_hash.0),
        alloy_rlp::encode(version.evm_root.0),
        alloy_rlp::encode(version.system_root.0),
        application,
        alloy_rlp::encode(version.content_digest),
    ])
}
