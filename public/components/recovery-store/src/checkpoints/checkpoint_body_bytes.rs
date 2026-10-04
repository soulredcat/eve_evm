// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::CheckpointBody;

pub fn checkpoint_body_bytes(body: &CheckpointBody) -> &[u8] {
    &body.bytes
}
