use crate::{InteropError, RouteManifest, validate_route_manifest};

/// Bounded discovery returns untrusted metadata only, never a custody authorization.
pub fn find_route_manifest(
    routes: &[RouteManifest],
    route_id: [u8; 32],
) -> Result<&RouteManifest, InteropError> {
    if routes.len() > 64 {
        return Err(InteropError::RegistryCapacityExceeded);
    }
    for (index, route) in routes.iter().enumerate() {
        validate_route_manifest(route)?;
        if routes[..index]
            .iter()
            .any(|previous| previous.route_id == route.route_id)
        {
            return Err(InteropError::DuplicateRoute);
        }
    }
    routes
        .iter()
        .find(|route| route.route_id == route_id)
        .ok_or(InteropError::UnknownRoute)
}
