use ml_dsa::Keypair;

use crate::MlDsa65SigningKey;

/// Export only public key material using the upstream standardized encoding.
pub fn export_mldsa65_public_key(signing_key: &MlDsa65SigningKey) -> Vec<u8> {
    signing_key.verifying_key().encode().to_vec()
}
