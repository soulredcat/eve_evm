//! Fail-closed capability check for the unextended pinned consensus engine.

mod require_supported_authentication;
mod types;

pub use require_supported_authentication::require_supported_authentication;
pub use types::{ConsensusAuthenticationRequirement, UnsupportedHybridConsensus};
