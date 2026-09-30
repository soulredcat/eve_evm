use super::release_snapshot_lease::release_snapshot_lease;
use crate::state::StateSnapshot;

impl std::ops::Drop for StateSnapshot<'_> {
    fn drop(&mut self) {
        release_snapshot_lease(&self.leases);
    }
}
