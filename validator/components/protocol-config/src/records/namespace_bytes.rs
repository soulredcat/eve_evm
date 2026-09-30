use super::SystemNamespace;

pub(crate) fn namespace_bytes(namespace: SystemNamespace) -> &'static [u8] {
    match namespace {
        SystemNamespace::Validator => b"validator",
        SystemNamespace::Fee => b"fee",
        SystemNamespace::Reward => b"reward",
        SystemNamespace::Parameter => b"parameter",
        SystemNamespace::Task => b"task",
        SystemNamespace::Evidence => b"evidence",
        SystemNamespace::Upgrade => b"upgrade",
    }
}
