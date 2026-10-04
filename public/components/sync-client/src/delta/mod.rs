// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod fetch_state_delta_bytes;
mod fetch_state_delta_chunk;
mod required_state_delta_download_reservation;
mod types;
pub use fetch_state_delta_bytes::fetch_state_delta_bytes;
pub use required_state_delta_download_reservation::required_state_delta_download_reservation;
pub use types::DownloadedStateDelta;

#[cfg(test)]
mod tests;
