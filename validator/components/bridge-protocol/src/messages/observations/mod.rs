mod bounded;
mod source_observation;
mod validation;
pub use bounded::BoundedSourceObservation;
pub use source_observation::{InclusionLocator, ProofSegment, SourceObservation};
pub use validation::validate_source_observation;
