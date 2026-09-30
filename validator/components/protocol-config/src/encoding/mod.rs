//! Canonical RLP construction shared by this protocol domain.

mod encode_list;
mod encode_optional_height;

pub(crate) use encode_list::encode_list;
pub(crate) use encode_optional_height::encode_optional_height;
