// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    SigningError,
    types::{GO_ZERO_TIME_SECONDS, MAX_TIMESTAMP_SECONDS},
};
use prost_types::Timestamp;

/// Missing native nonnullable time means Go's year-one zero time, not Unix epoch.
pub(crate) fn normalize_timestamp(value: Option<&Timestamp>) -> Result<Timestamp, SigningError> {
    let timestamp = value.cloned().unwrap_or(Timestamp {
        seconds: GO_ZERO_TIME_SECONDS,
        nanos: 0,
    });
    if !(GO_ZERO_TIME_SECONDS..=MAX_TIMESTAMP_SECONDS).contains(&timestamp.seconds)
        || !(0..1_000_000_000).contains(&timestamp.nanos)
    {
        return Err(SigningError::InvalidTimestamp);
    }
    Ok(timestamp)
}
