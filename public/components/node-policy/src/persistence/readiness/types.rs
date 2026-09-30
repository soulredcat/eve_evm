#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PersistenceObservation {
    pub queued_bytes: u64,
    pub queued_batches: u64,
    pub oldest_queue_age_ms: u64,
    pub storage_failed: bool,
    pub recoverable_tail_available: bool,
    pub independently_verified_head: Option<u64>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PublicReadiness {
    Ready {
        serve_height: u64,
        durable_height: u64,
    },
    NotReady(&'static str),
}
