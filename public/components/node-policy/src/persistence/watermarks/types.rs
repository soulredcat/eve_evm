#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FinalizedHeight(pub u64);
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AppliedHeight(pub u64);
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DurableRecoveryHeight(pub u64);
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CheckpointHeight(pub u64);
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AuthenticatedStateHeight(pub u64);

/// Caller-provided verification outcome, not an authenticated capability.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StateAuthentication {
    NextCertifiedHeader { header_height: u64 },
    VerifiedReplay,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PublicWatermarks {
    pub finalized: FinalizedHeight,
    pub applied: AppliedHeight,
    pub durable_recovery: DurableRecoveryHeight,
    pub checkpoint: CheckpointHeight,
    pub authenticated_state: AuthenticatedStateHeight,
    pub authentication: StateAuthentication,
    pub authenticated_snapshot_height: u64,
    pub oldest_retained_height: u64,
}
