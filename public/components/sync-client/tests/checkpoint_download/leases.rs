// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
pub(super) struct Lease {
    pub bytes: usize,
    pub held: Arc<AtomicUsize>,
}
impl Drop for Lease {
    fn drop(&mut self) {
        self.held.fetch_sub(self.bytes, Ordering::SeqCst);
    }
}
pub(super) fn reserve(held: &Arc<AtomicUsize>, bytes: usize) -> anyhow::Result<Lease> {
    held.fetch_add(bytes, Ordering::SeqCst);
    Ok(Lease {
        bytes,
        held: held.clone(),
    })
}
