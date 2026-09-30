use crate::{
    BoundedSourceObservation, BridgeProtocolError, InclusionLocator, SourceObservation,
    SourceVerificationContext,
};
use eve_interop::{ChainNamespace, TransferRequest, validate_transfer_request};

/// Check namespace/locators/amounts/bounds; never promote evidence to authenticated finality.
pub fn validate_source_observation(
    context: &SourceVerificationContext,
    observation: SourceObservation,
) -> Result<BoundedSourceObservation, BridgeProtocolError> {
    crate::verification::validate_source_context(context)?;
    let route = &context.route;
    if observation.source != route.source {
        return Err(BridgeProtocolError::WrongSourceNetwork);
    }
    if observation.source_custody != route.source_custody {
        return Err(BridgeProtocolError::WrongSourceCustody);
    }
    if observation.asset != route.asset {
        return Err(BridgeProtocolError::WrongOriginAsset);
    }
    if observation.source_height == 0 || observation.source_block_hash == [0; 32] {
        return Err(BridgeProtocolError::InvalidSourceReference);
    }
    validate_transfer_request(
        route,
        &TransferRequest {
            route_id: observation.route_id,
            source_amount: observation.source_amount,
            recipient: observation.recipient,
        },
    )
    .map_err(BridgeProtocolError::InvalidMetadata)?;
    match (route.source.namespace, observation.inclusion) {
        (ChainNamespace::Eve | ChainNamespace::Ethereum, InclusionLocator::EvmReceipt { .. })
        | (ChainNamespace::Solana, InclusionLocator::SolanaInstruction { .. }) => {}
        _ => return Err(BridgeProtocolError::WrongInclusionLocator),
    }
    let limits = context.proof_limits;
    if limits.maximum_total_bytes == 0
        || limits.maximum_total_bytes > 4 * 1024 * 1024
        || limits.maximum_segment_bytes == 0
        || limits.maximum_segment_bytes > 1024 * 1024
        || limits.maximum_segments == 0
        || limits.maximum_segments > 64
    {
        return Err(BridgeProtocolError::InvalidProofLimits);
    }
    if observation.proof.is_empty() {
        return Err(BridgeProtocolError::EmptyProof);
    }
    if observation.proof.len() > usize::from(limits.maximum_segments) {
        return Err(BridgeProtocolError::ProofLimitExceeded);
    }
    let mut total = 0usize;
    for segment in &observation.proof {
        if segment.bytes.is_empty() {
            return Err(BridgeProtocolError::EmptyProof);
        }
        if segment.bytes.len() > limits.maximum_segment_bytes as usize {
            return Err(BridgeProtocolError::ProofLimitExceeded);
        }
        total = total
            .checked_add(segment.bytes.len())
            .ok_or(BridgeProtocolError::ProofLimitExceeded)?;
        if total > limits.maximum_total_bytes as usize {
            return Err(BridgeProtocolError::ProofLimitExceeded);
        }
    }
    Ok(BoundedSourceObservation { observation })
}
