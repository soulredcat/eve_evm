//! Eligible-source classification, ranking and bounded switching policy.

mod eligibility;
mod selection;
mod switching;

pub use eligibility::{
    SourceEligibility, SourceObservation, SourcePurpose, SourceVerification, ZoneId,
    evaluate_source,
};
pub use selection::select_preferred_source;
pub use switching::should_switch_source;
