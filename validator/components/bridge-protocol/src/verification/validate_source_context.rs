use crate::{BridgeProtocolError, RequiredAuthenticationProfile, SourceVerificationContext};

/// Check policy reference namespaces without treating them as authenticated anchors.
pub fn validate_source_context(
    context: &SourceVerificationContext,
) -> Result<(), BridgeProtocolError> {
    if context.source_anchor.chain != context.route.source {
        return Err(BridgeProtocolError::WrongSourceNetwork);
    }
    if context.source_anchor.height == 0
        || context.source_anchor.block_hash == [0; 32]
        || context.source_anchor.historical_key_set_commitment == [0; 32]
    {
        return Err(BridgeProtocolError::InvalidSourceReference);
    }
    if context.source_verifier_identity == [0; 32] {
        return Err(BridgeProtocolError::AuthenticationUnavailable);
    }
    if context.destination_deployment.address != context.route.destination_custody
        || context.destination_deployment.implementation_commitment == [0; 32]
    {
        return Err(BridgeProtocolError::WrongDeployment);
    }
    if let RequiredAuthenticationProfile::ClassicalAndMldsa65 {
        encoding_version, ..
    } = context.required_profile
        && encoding_version != 1
    {
        return Err(BridgeProtocolError::UnsupportedProfile);
    }
    Ok(())
}
