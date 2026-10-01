// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::runtime::{
    channels::serve_application_channels,
    types::{ChannelWorkerGuard, RunningNode},
};
use anyhow::Context;
use std::os::unix::net::UnixListener;

pub(in crate::consensus::runtime) fn run_application_worker(
    node: &RunningNode,
    listener: UnixListener,
) {
    let _guard = ChannelWorkerGuard { node };
    super::report_channel_completion::report_channel_completion(
        node,
        serve_application_channels(node, listener).context("native_application_actor"),
    );
}
