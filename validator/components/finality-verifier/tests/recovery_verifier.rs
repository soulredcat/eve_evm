// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

#[path = "support/compiler.rs"]
mod compiler;

#[path = "recovery/codec_mutation.rs"]
mod codec_mutation;
#[path = "support/genesis.rs"]
mod genesis;
#[path = "support/native.rs"]
mod native;
#[path = "recovery/bounds/validate_recovery_envelope_bounds.rs"]
mod recovery_bounds;
#[path = "recovery/decoding/decode_compact_recovery_envelope.rs"]
mod recovery_decoding;
#[path = "recovery/bounds/validate_empty_recovery_envelope_bytes.rs"]
mod recovery_empty_bounds;
#[path = "recovery/encoding/encode_compact_recovery_envelope.rs"]
mod recovery_encoding;
#[path = "recovery/execution_validation.rs"]
mod recovery_execution_validation;
#[path = "recovery/history_guards.rs"]
mod recovery_history_guards;
#[path = "recovery/import/mod.rs"]
mod recovery_import;
#[path = "recovery/initialization.rs"]
mod recovery_initialization;
#[path = "recovery/parent_guards.rs"]
mod recovery_parent_guards;
#[path = "recovery/privacy.rs"]
mod recovery_privacy;
#[path = "recovery/replay.rs"]
mod recovery_replay;
#[path = "recovery/replay_guards.rs"]
mod recovery_replay_guards;
#[path = "recovery/support.rs"]
mod recovery_support;
