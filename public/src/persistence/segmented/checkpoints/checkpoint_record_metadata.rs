// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::SealedCheckpointRecord;
use eve_storage::records::segmented::checkpoints::CheckpointBaseMetadata;
pub fn checkpoint_record_metadata(record: &SealedCheckpointRecord) -> CheckpointBaseMetadata {
    record.0.metadata
}
