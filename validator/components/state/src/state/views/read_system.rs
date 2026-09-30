use super::StateView;
use alloy_primitives::B256;
use eve_protocol_config::records::SystemRecord;

pub fn read_system(view: &StateView, key: B256) -> Option<&SystemRecord> {
    view.state.system.get(&key)
}
