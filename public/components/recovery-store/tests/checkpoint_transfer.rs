// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

#![cfg(target_os = "linux")]

#[path = "checkpoint_transfer/pending_probe.rs"]
mod pending_probe;

#[path = "checkpoint_transfer/completion_policy.rs"]
mod completion_policy;

#[path = "checkpoint_transfer/completed_reopen.rs"]
mod completed_reopen;
#[path = "checkpoint_transfer/completed_reopen_identity.rs"]
mod completed_reopen_identity;
#[path = "checkpoint_transfer/reopen_support.rs"]
mod reopen_support;

#[path = "checkpoint_transfer/completion_pending.rs"]
mod completion_pending;
#[path = "checkpoint_transfer/manifest.rs"]
mod manifest;
#[path = "checkpoint_transfer/process.rs"]
mod process;
#[path = "checkpoint_transfer/repair_pending.rs"]
mod repair_pending;
#[path = "checkpoint_transfer/repair_policy.rs"]
mod repair_policy;
#[path = "checkpoint_transfer/resources.rs"]
mod resources;
#[path = "checkpoint_transfer/resume.rs"]
mod resume;
#[path = "checkpoint_transfer/roundtrip.rs"]
mod roundtrip;
#[path = "checkpoint_transfer/security.rs"]
mod security;
#[path = "checkpoint_transfer/support.rs"]
mod support;
