use super::{decode_value, take_bytes, take_list};
use crate::StateError;
use alloy_primitives::{Address, B256};
use eve_protocol_config::records::{SystemNamespace, SystemValue};

pub(crate) fn decode_system_value(
    namespace: SystemNamespace,
    input: &mut &[u8],
) -> Result<SystemValue, StateError> {
    let mut fields = take_list(input)?;
    let value = match namespace {
        SystemNamespace::Validator => {
            let owner: Address = decode_value(&mut fields)?;
            let key = take_bytes(&mut fields, 32)?
                .as_ref()
                .try_into()
                .map_err(|_| StateError::MalformedEncoding)?;
            let power = decode_value(&mut fields)?;
            let activation = decode_value(&mut fields)?;
            let mut optional = take_list(&mut fields)?;
            let removal = match decode_value::<u8>(&mut optional)? {
                0 => None,
                1 => Some(decode_value(&mut optional)?),
                _ => return Err(StateError::MalformedEncoding),
            };
            if !optional.is_empty() {
                return Err(StateError::MalformedEncoding);
            }
            SystemValue::Validator {
                owner,
                key,
                power,
                activation,
                removal,
                key_epoch: decode_value(&mut fields)?,
            }
        }
        SystemNamespace::Fee => SystemValue::Fee {
            burned: decode_value(&mut fields)?,
            node_pool: decode_value(&mut fields)?,
            validator_pool: decode_value(&mut fields)?,
        },
        SystemNamespace::Reward => SystemValue::Reward {
            owner: decode_value(&mut fields)?,
            role: decode_value(&mut fields)?,
            liability: decode_value(&mut fields)?,
            index: decode_value(&mut fields)?,
        },
        SystemNamespace::Parameter => SystemValue::Parameter {
            name: take_bytes(&mut fields, 64)?,
            value: take_bytes(&mut fields, 256)?,
        },
        SystemNamespace::Task => {
            let epoch = decode_value(&mut fields)?;
            let task_id: B256 = decode_value(&mut fields)?;
            let node = decode_value(&mut fields)?;
            let content = decode_value(&mut fields)?;
            let request_nonce = decode_value(&mut fields)?;
            let deadline_height = decode_value(&mut fields)?;
            let work_units = decode_value(&mut fields)?;
            let consumed = match decode_value::<u8>(&mut fields)? {
                0 => false,
                1 => true,
                _ => return Err(StateError::MalformedEncoding),
            };
            SystemValue::Task {
                epoch,
                task_id,
                node,
                content,
                request_nonce,
                deadline_height,
                work_units,
                consumed,
            }
        }
        SystemNamespace::Evidence => {
            let evidence_id = decode_value(&mut fields)?;
            let offense_height = decode_value(&mut fields)?;
            let applied = match decode_value::<u8>(&mut fields)? {
                0 => false,
                1 => true,
                _ => return Err(StateError::MalformedEncoding),
            };
            SystemValue::Evidence {
                evidence_id,
                offense_height,
                applied,
            }
        }
        SystemNamespace::Upgrade => SystemValue::Upgrade {
            old_version: decode_value(&mut fields)?,
            new_version: decode_value(&mut fields)?,
            activation_height: decode_value(&mut fields)?,
            code_digest: decode_value(&mut fields)?,
            migration: take_bytes(&mut fields, 128)?,
        },
    };
    if !fields.is_empty() {
        return Err(StateError::MalformedEncoding);
    }
    Ok(value)
}
