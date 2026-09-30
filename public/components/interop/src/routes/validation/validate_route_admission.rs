use crate::{InteropError, RouteManifest, validate_route_manifest};

/// Every B0 route remains disabled. Metadata never creates a verified capability.
pub fn validate_route_admission(route: &RouteManifest) -> Result<(), InteropError> {
    validate_route_manifest(route)?;
    Err(InteropError::RouteDisabled(route.state))
}
