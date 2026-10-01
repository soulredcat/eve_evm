// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

pub(in crate::records) const SCHEMA: &[u8] = b"EVEOPAQUE01";
pub(in crate::records) const SCHEMA_KEY: &[u8] = b"eve/opaque/v1/schema";
pub(in crate::records) const IDENTITY_KEY: &[u8] = b"eve/opaque/v1/identity";
pub(in crate::records) const HEAD_KEY: &[u8] = b"eve/opaque/v1/head";
pub(in crate::records) const RECORD_PREFIX: &[u8] = b"eve/opaque/v1/record/";
pub(in crate::records) const RECORD_HEADER_BYTES: usize = 88;
