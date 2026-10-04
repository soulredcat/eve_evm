// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::DownloadedCheckpointWitness;
pub fn downloaded_checkpoint_witness_wire<L>(downloaded: &DownloadedCheckpointWitness<L>) -> &[u8] {
    &downloaded.wire
}
