use super::StateView;
use crate::StateVersion;

pub fn view_version(view: &StateView) -> &StateVersion {
    &view.version
}
