// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod decode_record;
mod encode_record;
mod restore_block_id;
mod validate_record;
mod validate_signed_message;
pub(super) use decode_record::decode_record;
pub(super) use encode_record::encode_record;
use restore_block_id::restore_block_id;
pub(super) use validate_record::validate_record;
use validate_signed_message::validate_signed_message;

pub(super) const MAXIMUM_RECORD_PAYLOAD_BYTES: usize = 16_384;
