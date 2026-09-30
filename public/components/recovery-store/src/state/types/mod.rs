mod repository_types;
mod snapshot_types;
mod state_ack;
mod view_types;
#[cfg(test)]
pub(crate) use repository_types::SimulatedCommitFailure;
pub use repository_types::{StateReader, StateRepository};
pub use snapshot_types::StateSnapshot;
pub use state_ack::{CommitDisposition, DurableStateAck};
pub use view_types::ImmutableStateView;

pub(crate) const SCHEMA_KEY: &[u8] = b"eve/state/v1/schema";
pub(crate) const SCHEMA_BYTES: &[u8] = b"EVESTATE01";
pub(crate) const ACTIVE_KEY: &[u8] = b"eve/state/v1/active";
pub(crate) const GENESIS_KEY: &[u8] = b"eve/state/v1/genesis-commit";
pub(crate) const HEAD_KEY: &[u8] = b"eve/state/v1/durable-head";
pub(crate) const ACCOUNT_PREFIX: &[u8] = b"eve/state/v1/account/";
pub(crate) const SLOT_PREFIX: &[u8] = b"eve/state/v1/slot/";
pub(crate) const CODE_PREFIX: &[u8] = b"eve/state/v1/code/";
pub(crate) const SYSTEM_PREFIX: &[u8] = b"eve/state/v1/system/";
pub(crate) const HASH_PREFIX: &[u8] = b"eve/state/v1/execution-hash/";
pub(crate) const COMMIT_PREFIX: &[u8] = b"eve/state/v1/commit/";
pub(crate) const ID_PREFIX: &[u8] = b"eve/state/v1/commit-id/";
pub(crate) const HEADER_PREFIX: &[u8] = b"eve/state/v1/header/";
pub(crate) const TX_PREFIX: &[u8] = b"eve/state/v1/transactions/";
pub(crate) const RECEIPT_PREFIX: &[u8] = b"eve/state/v1/receipts/";
pub(crate) const ROOT_PREFIX: &[u8] = b"eve/state/v1/roots/";
