// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::reading::load_opaque_record;
use crate::records::{OpaqueRecordCursor, OpaqueRecordRepository, hashing::hash_opaque_record};
use anyhow::{Result, ensure};

pub(in crate::records) fn check_exact_opaque_replay(
    store: &OpaqueRecordRepository,
    expected: OpaqueRecordCursor,
    payloads: &[Vec<u8>],
) -> Result<OpaqueRecordCursor> {
    ensure!(
        expected.sequence < store.head.sequence,
        "stale/future opaque compare"
    );
    let actual_parent = if expected.sequence == 0 {
        OpaqueRecordCursor {
            sequence: 0,
            content_hash: hash_opaque_record(store.identity, OpaqueRecordCursor::default(), 0, &[]),
        }
    } else {
        let parent = load_opaque_record(store, expected.sequence)?;
        OpaqueRecordCursor {
            sequence: parent.sequence,
            content_hash: parent.content_hash,
        }
    };
    ensure!(
        expected == actual_parent,
        "opaque replay parent identity/content mismatch"
    );
    let mut parent = expected;
    for payload in payloads {
        let sequence = parent
            .sequence
            .checked_add(1)
            .ok_or_else(|| anyhow::anyhow!("opaque sequence overflow"))?;
        ensure!(
            sequence <= store.head.sequence,
            "mixed opaque replay/new append is forbidden"
        );
        let retained = load_opaque_record(store, sequence)?;
        ensure!(
            retained.parent == parent && retained.payload == *payload,
            "opaque replay differs from exact retained parent/payload"
        );
        parent = OpaqueRecordCursor {
            sequence,
            content_hash: retained.content_hash,
        };
    }
    Ok(parent)
}
