// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod follow_next_validator_delta;
mod types;
pub use follow_next_validator_delta::follow_next_validator_delta;
pub use types::ValidatorFollowerSource;
#[cfg(test)]
mod tests;
