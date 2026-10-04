// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::sync::applied::AppliedPublication;
use eve_node_policy::PublicReadiness;
/// This foundation has no independent fresh-head evidence; valid historical state remains readable.
pub fn applied_readiness(publication: &AppliedPublication) -> PublicReadiness {
    if publication.storage_failed {
        return PublicReadiness::NotReady("storage failed");
    }
    if publication
        .segmented_position
        .is_some_and(|position| position.missing_from.is_some())
    {
        return PublicReadiness::NotReady("missing complete recovery tail");
    }
    PublicReadiness::NotReady("head freshness unknown")
}
