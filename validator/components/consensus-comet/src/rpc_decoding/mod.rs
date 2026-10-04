// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Bounded native JSON shapes. Parsed data carries no finality or state authority.
mod decode_base64;
mod decode_decimal;
mod decode_hex;
mod decode_native_block;
mod decode_native_block_id;
mod decode_native_commit;
mod decode_native_commit_signature;
mod decode_native_header;
mod decode_native_validator;
mod decode_native_validators;
mod decode_timestamp;
mod types;
pub use decode_native_block::decode_native_block;
pub use decode_native_commit::decode_native_commit;
pub use decode_native_header::decode_native_header;
pub use decode_native_validators::decode_native_validators;
pub use types::NativeBlock;
