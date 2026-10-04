// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::ingress_reservation_types::CheckpointIngressReservation;
use crate::sync::applied::{
    AppliedReader, reserve_applied_snapshot_staging, reserve_applied_working,
};
use anyhow::Result;

pub(super) fn reserve_checkpoint_ingress(
    reader: &AppliedReader,
    bytes: usize,
) -> Result<CheckpointIngressReservation> {
    let working = reserve_applied_working(reader, bytes)
        .map_err(|error| anyhow::anyhow!("checkpoint ingress reservation: {error:?}"))?;
    // The fixed runtime profile caps both request and JSON response at 256 KiB.
    // Five raw/RLP/response copy envelopes, two request/framing envelopes and 64 KiB
    // control cover every network callback independently of the decoded JSON estimate.
    let raw = 5 * 262_144 + 2 * 262_144 + 65_536;
    let staging = reserve_applied_snapshot_staging(reader, raw)
        .map_err(|error| anyhow::anyhow!("checkpoint raw staging reservation: {error:?}"))?;
    Ok(CheckpointIngressReservation {
        _working: working,
        _staging: staging,
    })
}
