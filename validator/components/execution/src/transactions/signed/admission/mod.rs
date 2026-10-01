// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod check_transaction_admission;
mod types;

pub use check_transaction_admission::check_transaction_admission;
pub use types::{TransactionAdmission, TransactionAdmissionError};
