use eve_state::StateVersion;

pub(crate) fn encode_root_record(version: &StateVersion) -> Vec<u8> {
    let mut bytes = [
        version.evm_root.0.as_slice(),
        version.system_root.0.as_slice(),
        version.content_digest.as_slice(),
    ]
    .concat();
    bytes.push(u8::from(version.application.is_some()));
    if let Some(application) = version.application {
        bytes.extend_from_slice(application.0.as_slice());
    }
    bytes
}
