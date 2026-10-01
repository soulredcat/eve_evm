// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

#[path = "native_certificates/adversarial.rs"]
mod adversarial;
#[path = "native_classical/support/mod.rs"]
pub mod native_support;
#[path = "native_certificates/native_vectors.rs"]
mod native_vectors;
#[path = "native_certificates/set_bounds.rs"]
mod set_bounds;
#[path = "native_certificates/signature_failures.rs"]
mod signature_failures;
#[path = "native_certificates/support.rs"]
mod support;
#[path = "native_certificates/transaction_data.rs"]
mod transaction_data;
