// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::encode_version;
use crate::state::validation::validate_version_metadata;
use crate::{Bytes, StateError, StateVersion};

pub fn encode_state_version(version: &StateVersion) -> Result<Bytes, StateError> {
    validate_version_metadata(version)?;
    Ok(encode_version(version).into())
}
