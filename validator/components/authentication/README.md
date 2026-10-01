<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Validator authentication component

Canonical owner: validator authentication. This package implements bounded
ML-DSA-65 encoding/signing/verification and strict paired Ed25519 AND ML-DSA
authorization over one bound identity/message. It is experimental integration,
not consensus, authenticated enrollment, FIPS validation or a verified PQ profile.

Required official verification fixtures and notices live inside `tests/fixtures/`
so packaging does not depend on a repository-external test directory.

The complete NIST terms remain in the extensionless `tests/fixtures/nist-acvp/NOTICE`
with its exact upstream association in root REUSE.toml; README-only Markdown
publication must not remove mandatory third-party rights or change fixture bytes.
Reusable source included in public/master distributions must be generated reproducibly
from this canonical component; no divergent handwritten implementation is allowed.

The complete NIST terms remain in extensionless `tests/fixtures/nist-acvp/NOTICE`
with the exact REUSE association. README-only publication does not remove upstream
rights or alter official/derived cryptographic fixture bytes.

The default shared workspace currently supplies build metadata. Complete role
distributions still need standalone manifests/lockfiles and isolated copy/build/run
verification; their runtime/package acceptance is not yet implemented.
