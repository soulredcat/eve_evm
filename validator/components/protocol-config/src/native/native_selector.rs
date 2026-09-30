use alloy_primitives::keccak256;

pub fn native_selector(signature: &str) -> [u8; 4] {
    keccak256(signature.as_bytes())[..4]
        .try_into()
        .expect("fixed hash width")
}
