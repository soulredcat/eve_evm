pub(crate) fn height_key(prefix: &[u8], height: u64) -> Vec<u8> {
    [prefix, &height.to_be_bytes()].concat()
}
