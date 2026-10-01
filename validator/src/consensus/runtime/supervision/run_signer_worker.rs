// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::runtime::{
    channels::serve_signer_channel,
    types::{ChannelWorkerGuard, RunningNode},
};
use anyhow::Context;
use std::path::Path;

pub(in crate::consensus::runtime) fn run_signer_worker(node: &RunningNode, socket: &Path) {
    let _guard = ChannelWorkerGuard { node };
    super::report_channel_completion::report_channel_completion(
        node,
        serve_signer_channel(node, socket).context("native_signer_actor"),
    );
}
