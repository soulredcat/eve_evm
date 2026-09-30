use std::sync::atomic::{AtomicUsize, Ordering};

pub(crate) fn release_snapshot_lease(leases: &AtomicUsize) {
    let previous = leases.fetch_sub(1, Ordering::AcqRel);
    debug_assert!(
        previous > 0,
        "snapshot lease must be reserved before release"
    );
}
