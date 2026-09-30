use alloy_primitives::keccak256;

use super::{
    ApplicationCommitment, ApplicationCommitmentInput, RecordError, encode_application_commitment,
};

pub fn hash_application_commitment(
    input: ApplicationCommitmentInput,
) -> Result<ApplicationCommitment, RecordError> {
    Ok(ApplicationCommitment(keccak256(
        encode_application_commitment(input)?,
    )))
}
