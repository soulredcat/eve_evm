// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::DownloadedCheckpointResponse;
use eve_storage::checkpoints::messages::CheckpointResponse;
pub fn downloaded_checkpoint_response<L>(
    downloaded: &DownloadedCheckpointResponse<L>,
) -> &CheckpointResponse {
    &downloaded.response
}
