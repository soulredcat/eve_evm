use super::{decode_system_value::decode_system_value, decode_value, take_bytes, take_list};
use crate::StateError;
use eve_protocol_config::records::{SystemNamespace, SystemRecord, encode_system_record};

pub fn decode_system_record(bytes: &[u8]) -> Result<SystemRecord, StateError> {
    if bytes.len() > 4_096 {
        return Err(StateError::BudgetExceeded);
    }
    let mut remaining = bytes;
    let mut list = take_list(&mut remaining)?;
    let schema_version = decode_value(&mut list)?;
    let namespace = match take_bytes(&mut list, 16)?.as_ref() {
        b"validator" => SystemNamespace::Validator,
        b"fee" => SystemNamespace::Fee,
        b"reward" => SystemNamespace::Reward,
        b"parameter" => SystemNamespace::Parameter,
        b"task" => SystemNamespace::Task,
        b"evidence" => SystemNamespace::Evidence,
        b"upgrade" => SystemNamespace::Upgrade,
        _ => return Err(StateError::MalformedEncoding),
    };
    let logical_key = take_bytes(&mut list, 128)?;
    let value = decode_system_value(namespace, &mut list)?;
    let record = SystemRecord {
        schema_version,
        namespace,
        logical_key,
        value,
    };
    if !list.is_empty()
        || !remaining.is_empty()
        || encode_system_record(&record).map_err(StateError::SystemRecord)? != bytes
    {
        return Err(StateError::NonCanonicalEncoding);
    }
    Ok(record)
}
