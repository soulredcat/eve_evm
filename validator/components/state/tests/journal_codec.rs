// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod support;

#[path = "journal_codec/allocation_preflight.rs"]
mod allocation_preflight;
#[path = "journal_codec/budget_limits.rs"]
mod budget_limits;
#[path = "journal_codec/fixtures.rs"]
mod fixtures;
#[path = "journal_codec/malformed_encodings.rs"]
mod malformed_encodings;
#[path = "journal_codec/roundtrip.rs"]
mod roundtrip;
#[path = "journal_codec/system_records.rs"]
mod system_records;
#[path = "journal_codec/version_fields.rs"]
mod version_fields;
