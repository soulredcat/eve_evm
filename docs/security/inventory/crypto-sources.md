<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Cryptographic sources and maintained cross-verification

## Experimental Rust backend

`ml-dsa = 0.1.1` with `zeroize`, RustCrypto source commit
`f75d5b829948988f18d9463f286805fb9410bcdd`, package SHA-256
`add6b9d92e496f16f4526d68ff29da1483aba4b119baeab8bed3b9e3544a6f3d`.
License: Apache-2.0 OR MIT. The package README explicitly says the implementation
has never been independently audited. Source/API inspection, standard vectors and
independent implementation agreement do not replace an audit or FIPS validation.
Status: EXPERIMENTAL_UNAUDITED; production authentication is not approved here.

Selected parameter/interface: ML-DSA-65, pure external message encoding, 1952-byte
public key, 3309-byte signature, context length at most 255. No silent downgrade
to another parameter set, precomputed-mu or prehash interface is allowed. Existing
Ed25519 paired verification requires both signatures over one bound message;
authenticated enrollment and engine/client integration remain NOT_IMPLEMENTED.

The [FIPS 204 page](https://csrc.nist.gov/pubs/fips/204/final) was checked on
2026-09-30. Its 2026-07-31 potential-update spreadsheet was inspected and pinned:
[NIST errata](https://csrc.nist.gov/files/pubs/fips/204/final/docs/fips-204-potential-updates.xlsx),
SHA-256 `5bc93ce63bc647e6d1d456cb2d3a171426c15aca4a7a0e0edd40d08b7a34c793`.
It includes clarifications to NTT/message notation and internal signing bounds;
these potential corrections are not a new algorithm standard. The pinned Rust
signing loop ranges over a 16-bit counter in parameter-dependent steps rather
than the outdated 814-attempt bound. Full implementation/side-channel review and
the effect of signing failure handling remain required for a production decision.

Official NIST ACVP corpus commit:
`a7f283cdc87d2d6dd93c1bac59e5622c5f9f8324`; all 15 external/pure ML-DSA-65
verification cases are versioned in the authentication package with exact source
digests and the complete NIST notice. Inputs are public and contain no secret keys.

## Maintained OpenSSL reference

Final reference: OpenSSL 3.5.7, source commit
`8cf17aaeb4599f8af87fefd810b5b5fee90fe69e`, Apache-2.0 with upstream notices.
[Pinned source archive](https://github.com/openssl/openssl/releases/download/openssl-3.5.7/openssl-3.5.7.tar.gz):
SHA-256 `a8c0d28a529ca480f9f36cf5792e2cd21984552a3c8e4aa11a24aa31aeac98e8`;
downloaded bytes matched the publisher release digest before source execution.
The isolated reference uses the default provider, not a certified FIPS module.
Reviewed API: [pkeyutl](https://docs.openssl.org/3.5/man1/openssl-pkeyutl/) and
[ML-DSA signatures](https://docs.openssl.org/3.5/man7/EVP_SIGNATURE-ML-DSA/).
Pure mode and explicit context are preserved. Public-key SPKI serialization uses
[RFC 9881](https://www.rfc-editor.org/rfc/rfc9881.html), ML-DSA-65 OID
`2.16.840.1.101.3.4.3.18`, absent parameters and the full raw public key.

The strict versioned test executes RustCrypto -> OpenSSL and OpenSSL -> RustCrypto,
wrong-message/context, mutated/truncated signatures, exact SPKI/key/signature size,
and all 15 official NIST cases through OpenSSL. Missing or mismatched OpenSSL fails
the tests; there is no ignore/skip fallback. Processes appear only in test code.

Rebuild the task-local reference after digest validation:

```sh
tar xzf openssl-3.5.7.tar.gz
cd openssl-3.5.7
perl Configure linux-x86_64 no-shared no-tests no-docs
make -j2
apps/openssl version
```

Recorded compiler/runtime: GCC Debian 14.2.0-19, Perl 5.40.1, GNU Make 4.4.1,
Linux x86_64/WSL, Rust/Cargo 1.97.1 Windows GNU for the integration runner. The
`no-tests` flag disables the upstream source-build suite; it does not skip EVE's
required maintained-reference tests. An OpenSSL upstream full test suite was not
run and this result is not its replacement. Source path/build flags are recorded
because the generated binary identity is environment-specific:

| Task-local artifact | SHA-256 |
|---|---|
| `apps/openssl` | `a560fee7791ebf7dd1d139ba1a13ec37bfa869cae6b557da34edd045c4136acf` |
| `libcrypto.a` | `b208d373a9202b6fcc5755a82f74bdbf66ee0b4ff8389f96b1433dd082ee8db9` |
| `libssl.a` | `eca342df4db65a8618922c0c6bfefebd542ea1a5ece558a0bf5802a99f10c20a` |
| Runtime `libc.so.6` | `9792e3cbb541c8f44c7acf5f14f4022ea62998ecc787d326bed4d8b6547dfd92` |
| Runtime `ld-linux-x86-64.so.2` | `c8438e4fde1934e61c88311633f00949ff645d5c04cdb8671fa3d78164d2f307` |

The binary links static OpenSSL libraries; dynamically linked platform libraries
remain environment inputs, not published binaries. No system package was upgraded.
Raw source/build output remains ignored under `local-tests/security-interop-b0/`.

Windows/WSL reproduction environment:

```powershell
$env:EVE_OPENSSL_WSL = '1'
$env:EVE_OPENSSL = '/mnt/e/eve_evm/local-tests/security-interop-b0/openssl-3.5.7/apps/openssl'
cargo test --locked -p eve-crypto --test maintained_cross_implementation -- --test-threads=1
```

On Linux set `EVE_OPENSSL` to the pinned executable and omit the WSL variable.
The 2026-09-30 UTC patched run exited 0: 3 tests, none skipped, 4.15 seconds. Historical
3.5.6 results are superseded: [3.5.7 is a security patch](https://github.com/openssl/openssl/releases/tag/openssl-3.5.7)
with known fixes. A selected negative run against system 3.5.6 exited 1 at the
exact-version assertion, demonstrating that the final test rejects the old tool.

## Retired test reference and residual work

Historical dev-only PQClean wrappers `pqcrypto-mldsa 0.1.2`,
`pqcrypto-traits 0.3.5`, `pqcrypto-internals 0.2.11` had unmaintained advisories:
[0166](https://rustsec.org/advisories/RUSTSEC-2026-0166.html),
[0162](https://rustsec.org/advisories/RUSTSEC-2026-0162.html),
[0163](https://rustsec.org/advisories/RUSTSEC-2026-0163.html).
Their earlier cross-verification result is historical, not the final reference.
Those wrappers and their two legacy tests have now been removed from the manifests
and integrated lockfile. The checked local audit report
`local-tests/b0-dependency-audit.json` (local only) records zero known vulnerabilities
and two remaining unmaintained warnings: derivative 2.2.0 / RUSTSEC-2024-0388 and
paste 1.0.15 / RUSTSEC-2024-0436. These remaining transitive warnings stay visible;
absence of a known advisory is not an audit or cryptographic security proof.

The fresh reproducible provisioner builds the same patched source under
`local-tests/toolchain-b0/openssl/` and records its actual executable SHA256
`fee3bb2754da06f2855fbbda174a1ab42e4f02c72dbd9e9033b30b404a42868f`.
The earlier `a560...` table identifies the original bounded-spike build path,
not a portable universal executable hash. Full provisioning receipts bind source,
build recipe, version and actual artifact bytes. See
[tool provisioning](../../development/tool-provisioning.md) for fresh-clone setup.

SLH-DSA release/recovery alternatives and maintained authenticated hybrid transport
remain unimplemented. Consensus-engine paired verification, protected accounts,
client anchors, release/recovery and secured EVE core capacity remain
mandatory. No standardized algorithm/library/test certifies the complete network.
Bridge/external programs are deferred until EVE testnet under D40 and have their
own later trust/crypto acceptance; they are not core completion dependencies.
