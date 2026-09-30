use crate::SourceObservation;

/// Validated metadata/resource bounds only; it conveys no source authentication.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundedSourceObservation {
    pub(crate) observation: SourceObservation,
}
