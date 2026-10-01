// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Test-only process/serialization adapter. No production code invokes OpenSSL.
use std::{
    path::Path,
    process::{Command, Output},
};

// RFC 9881 ML-DSA-65 OID 2.16.840.1.101.3.4.3.18, absent parameters,
// DER SubjectPublicKeyInfo containing a 1952-byte key and zero unused bits.
const PUBLIC_PREFIX: [u8; 22] = [
    0x30, 0x82, 0x07, 0xb2, 0x30, 0x0b, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x03,
    0x12, 0x03, 0x82, 0x07, 0xa1, 0x00,
];

pub fn wrap_public(public: &[u8]) -> Vec<u8> {
    assert_eq!(public.len(), 1952);
    [PUBLIC_PREFIX.as_slice(), public].concat()
}

pub fn openssl(arguments: &[&str]) -> Output {
    let mut command = if std::env::var_os("EVE_OPENSSL_WSL").is_some() {
        let mut runner = Command::new("wsl.exe");
        runner
            .arg("--exec")
            .arg(std::env::var_os("EVE_OPENSSL").unwrap_or_else(|| "/usr/bin/openssl".into()));
        runner
    } else {
        Command::new(std::env::var_os("EVE_OPENSSL").unwrap_or_else(|| "openssl".into()))
    };
    command
        .args(arguments)
        .output()
        .expect("required pinned OpenSSL tool must execute")
}

pub fn assert_openssl_pin() {
    let output = openssl(&["version"]);
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap().trim(),
        "OpenSSL 3.5.7 9 Jun 2026 (Library: OpenSSL 3.5.7 9 Jun 2026)",
        "external cryptographic reference version must match the SEC0 pin"
    );
}

pub fn tool_path(path: &Path) -> String {
    let native = path.to_str().expect("test path must be UTF-8");
    if std::env::var_os("EVE_OPENSSL_WSL").is_some() {
        let native = native.replace('\\', "/");
        assert!(native.len() >= 3 && native.as_bytes()[1] == b':');
        format!("/mnt/{}/{}", native[..1].to_ascii_lowercase(), &native[3..])
    } else {
        native.to_owned()
    }
}
