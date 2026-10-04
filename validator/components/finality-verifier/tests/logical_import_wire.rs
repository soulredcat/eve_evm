// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

#![allow(dead_code)] // Reused deterministic fixtures expose other scenario helpers.

#[path = "recovery/codec_mutation.rs"]
mod codec_mutation;
#[path = "support/compiler.rs"]
mod compiler;
#[path = "recovery/import/support.rs"]
mod import_support;
#[path = "recovery/import/logical_wire/mod.rs"]
mod logical_wire;
#[path = "support/native.rs"]
mod native;
#[path = "recovery/support.rs"]
mod recovery_support;
