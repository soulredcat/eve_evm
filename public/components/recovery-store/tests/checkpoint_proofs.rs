// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

#![cfg(target_os = "linux")]

#[path = "checkpoint_proofs/pending_probe.rs"]
mod pending_probe;

#[path = "checkpoint_proofs/pending_policy.rs"]
mod pending_policy;

#[path = "checkpoint_proofs/completed_reopen.rs"]
mod completed_reopen;
#[path = "checkpoint_proofs/completed_reopen_identity.rs"]
mod completed_reopen_identity;
#[path = "checkpoint_proofs/reopen_support.rs"]
mod reopen_support;

#[path = "checkpoint_proofs/bounds.rs"]
mod bounds;
#[path = "checkpoint_proofs/process.rs"]
mod process;
#[path = "checkpoint_proofs/resume.rs"]
mod resume;
#[path = "checkpoint_proofs/roundtrip.rs"]
mod roundtrip;
#[path = "checkpoint_proofs/security.rs"]
mod security;
#[path = "checkpoint_proofs/support.rs"]
mod support;
