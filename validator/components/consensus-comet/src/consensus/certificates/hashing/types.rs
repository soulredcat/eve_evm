// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

/// Standard protobuf wrappers used by the native Header.Hash cdcEncode routine.
#[derive(Clone, PartialEq, prost::Message)]
pub(in crate::consensus::certificates) struct StringValue {
    #[prost(string, tag = "1")]
    pub value: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub(in crate::consensus::certificates) struct Int64Value {
    #[prost(int64, tag = "1")]
    pub value: i64,
}

#[derive(Clone, PartialEq, prost::Message)]
pub(in crate::consensus::certificates) struct BytesValue {
    #[prost(bytes = "vec", tag = "1")]
    pub value: Vec<u8>,
}
