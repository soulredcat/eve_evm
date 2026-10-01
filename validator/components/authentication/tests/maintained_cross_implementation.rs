// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Strict OpenSSL 3.5.7 interoperability: missing/wrong tools are failures, not skips.
use eve_crypto::{derive_mldsa65_key, export_mldsa65_public_key, sign_mldsa65, verify_mldsa65};
use std::fs;

mod maintained_cross_implementation_support;
use maintained_cross_implementation_support::{
    assert_openssl_pin, openssl, tool_path, wrap_public,
};

#[test]
fn tp01_rustcrypto_signature_verifies_with_maintained_openssl() {
    assert_openssl_pin();
    let directory = tempfile::tempdir().unwrap();
    let public_path = directory.path().join("public.der");
    let message_path = directory.path().join("message.bin");
    let signature_path = directory.path().join("signature.bin");
    let key = derive_mldsa65_key(&[46; 32]).unwrap();
    let signature = sign_mldsa65(&key, b"cross implementation", b"shared context").unwrap();
    fs::write(&public_path, wrap_public(&export_mldsa65_public_key(&key))).unwrap();
    fs::write(&message_path, b"cross implementation").unwrap();
    fs::write(&signature_path, &signature).unwrap();
    let args = [
        "pkeyutl",
        "-verify",
        "-pubin",
        "-inkey",
        &tool_path(&public_path),
        "-in",
        &tool_path(&message_path),
        "-sigfile",
        &tool_path(&signature_path),
        "-pkeyopt",
        "context-string:shared context",
    ];
    assert!(openssl(&args).status.success());
    fs::write(&message_path, b"different message").unwrap();
    assert!(!openssl(&args).status.success());
    fs::write(&message_path, b"cross implementation").unwrap();
    let mut wrong_context = args;
    wrong_context[10] = "context-string:other context";
    assert!(!openssl(&wrong_context).status.success());
    let mut altered = signature.clone();
    altered[0] ^= 1;
    fs::write(&signature_path, &altered).unwrap();
    assert!(!openssl(&args).status.success());
}

#[test]
fn tp01_maintained_openssl_signature_verifies_with_rustcrypto() {
    assert_openssl_pin();
    let directory = tempfile::tempdir().unwrap();
    let secret = tool_path(&directory.path().join("ephemeral-test-key.pem"));
    let public_path = directory.path().join("public.der");
    let message_path = directory.path().join("message.bin");
    let signature_path = directory.path().join("signature.bin");
    fs::write(&message_path, b"cross implementation").unwrap();
    assert!(
        openssl(&["genpkey", "-algorithm", "ML-DSA-65", "-out", &secret])
            .status
            .success()
    );
    assert!(
        openssl(&[
            "pkey",
            "-in",
            &secret,
            "-pubout",
            "-outform",
            "DER",
            "-out",
            &tool_path(&public_path)
        ])
        .status
        .success()
    );
    assert!(
        openssl(&[
            "pkeyutl",
            "-sign",
            "-inkey",
            &secret,
            "-in",
            &tool_path(&message_path),
            "-out",
            &tool_path(&signature_path),
            "-pkeyopt",
            "context-string:shared context"
        ])
        .status
        .success()
    );
    let public_der = fs::read(public_path).unwrap();
    let public = &public_der[22..];
    assert_eq!(
        public_der,
        wrap_public(public),
        "exact RFC 9881 SPKI encoding"
    );
    assert_eq!(public.len(), 1952);
    let signature = fs::read(signature_path).unwrap();
    assert_eq!(signature.len(), 3309);
    assert_eq!(
        verify_mldsa65(
            public,
            b"cross implementation",
            b"shared context",
            &signature
        ),
        Ok(())
    );
    assert!(verify_mldsa65(public, b"different message", b"shared context", &signature).is_err());
    assert!(
        verify_mldsa65(
            public,
            b"cross implementation",
            b"other context",
            &signature
        )
        .is_err()
    );
    assert!(
        verify_mldsa65(
            public,
            b"cross implementation",
            b"shared context",
            &signature[..3308]
        )
        .is_err()
    );
}

#[test]
fn tp01_maintained_openssl_checks_all_official_pure_mldsa65_cases() {
    assert_openssl_pin();
    let fixture: serde_json::Value = serde_json::from_str(include_str!(
        "fixtures/nist-acvp/ml-dsa65-sigver-external-pure.json"
    ))
    .unwrap();
    let cases = fixture["tests"].as_array().unwrap();
    assert_eq!(cases.len(), 15);
    let directory = tempfile::tempdir().unwrap();
    let public = directory.path().join("public.der");
    let message = directory.path().join("message.bin");
    let signature = directory.path().join("signature.bin");
    for case in cases {
        fs::write(
            &public,
            wrap_public(&hex::decode(case["public_key"].as_str().unwrap()).unwrap()),
        )
        .unwrap();
        fs::write(
            &message,
            hex::decode(case["message"].as_str().unwrap()).unwrap(),
        )
        .unwrap();
        fs::write(
            &signature,
            hex::decode(case["signature"].as_str().unwrap()).unwrap(),
        )
        .unwrap();
        let context = format!("hexcontext-string:{}", case["context"].as_str().unwrap());
        let result = openssl(&[
            "pkeyutl",
            "-verify",
            "-pubin",
            "-inkey",
            &tool_path(&public),
            "-in",
            &tool_path(&message),
            "-sigfile",
            &tool_path(&signature),
            "-pkeyopt",
            &context,
        ]);
        assert_eq!(
            result.status.success(),
            case["expected_valid"].as_bool().unwrap(),
            "NIST tcId {}: {}",
            case["tc_id"],
            String::from_utf8_lossy(&result.stderr)
        );
    }
}
