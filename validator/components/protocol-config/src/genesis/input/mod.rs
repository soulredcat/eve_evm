// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Pure bounded development specification decoding; runtime authority stays with callers.

mod decode_development_spec;
mod decode_public_key;
mod types;

pub use decode_development_spec::decode_development_spec;
pub use types::DevelopmentSpecError;
