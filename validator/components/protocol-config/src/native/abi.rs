/// ABI v1: dynamic bytes are bounded versioned key/possession/proof envelopes.
pub const NATIVE_FUNCTIONS_V1: &[&str] = &[
    "registerNode(bytes32,bytes)",
    "registerValidator(bytes,bytes,uint16)",
    "delegate(address,uint256)",
    "undelegate(address,uint256)",
    "claimUnbonded()",
    "claimRewards(uint8)",
    "rotateConsensusKey(bytes,bytes)",
    "setCommission(uint16)",
    "submitWorkReceipt(bytes)",
    "submitEvidence(bytes)",
    "getRegistry(address,uint8)",
    "getPendingActivation(address)",
    "getBondedBalance(address,address)",
    "getUnbondingBalance(address,address)",
    "getWorkTask(bytes32)",
    "getClaimableRewards(address,uint8)",
];

pub const NATIVE_EVENTS_V1: &[&str] = &[
    "NodeRegistered(address,bytes32,uint256)",
    "ValidatorRegistered(address,bytes32,uint256)",
    "Delegated(address,address,uint256)",
    "Undelegated(address,address,uint256,uint64)",
    "UnbondedClaimed(address,uint256)",
    "RewardsClaimed(address,uint8,uint256)",
    "ConsensusKeyRotated(address,bytes32,uint64)",
    "CommissionScheduled(address,uint16,uint64)",
    "WorkReceiptAccepted(bytes32,address,uint64)",
    "EvidenceApplied(bytes32,address,uint16)",
];
