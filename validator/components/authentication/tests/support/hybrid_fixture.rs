use ed25519_dalek::{Signer, SigningKey};
use eve_crypto::{
    EnrolledHybridIdentity, HybridAuthorization, HybridSignature, MlDsa65SigningKey,
    derive_mldsa65_key, encode_hybrid_message, export_mldsa65_public_key, sign_mldsa65,
};

pub(crate) struct Fixture {
    pub enrollment: EnrolledHybridIdentity,
    pub classical: SigningKey,
    pub pq: MlDsa65SigningKey,
}

pub(crate) struct Signed {
    pub classical: Vec<u8>,
    pub pq: Vec<u8>,
}

pub(crate) fn fixture() -> Fixture {
    let classical = SigningKey::from_bytes(&[47; 32]);
    let pq = derive_mldsa65_key(&[48; 32]).unwrap();
    let enrollment = EnrolledHybridIdentity {
        identity: [49; 32],
        key_epoch: 7,
        ed25519_public_key: classical.verifying_key().to_bytes(),
        mldsa65_public_key: export_mldsa65_public_key(&pq),
    };
    Fixture {
        enrollment,
        classical,
        pq,
    }
}

pub(crate) fn sign(fixture: &Fixture, authorization: &HybridAuthorization<'_>) -> Signed {
    let message = encode_hybrid_message(&fixture.enrollment, authorization).unwrap();
    Signed {
        classical: fixture.classical.sign(&message).to_bytes().to_vec(),
        pq: sign_mldsa65(&fixture.pq, &message, b"EVE_HYBRID_V1").unwrap(),
    }
}

pub(crate) fn proof<'a>(fixture: &Fixture, signed: &'a Signed) -> HybridSignature<'a> {
    HybridSignature {
        signer_identity: fixture.enrollment.identity,
        key_epoch: fixture.enrollment.key_epoch,
        ed25519_signature: &signed.classical,
        mldsa65_signature: &signed.pq,
    }
}
