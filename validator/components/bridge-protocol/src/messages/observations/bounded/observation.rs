use super::BoundedSourceObservation;
use crate::SourceObservation;

impl BoundedSourceObservation {
    pub fn observation(&self) -> &SourceObservation {
        &self.observation
    }
}
