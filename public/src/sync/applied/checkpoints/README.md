<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Charged public checkpoint preparation

Public owns checkpoint transfer orchestration and RAM publication. The reusable
recovery store owns only bounded immutable local content and witness files.
Validator finality verification supplies the private authenticated capability.

Each constructor captures the actual public owner publication and charges the
same working pool before allocating retained metadata, copies, directory scans,
assembled content, decoded target, witness bodies or verifier work. Caller-owned
network input buffers require their own ingress reservation for their full
lifetime; borrowed bytes do not prove that the caller holds such a reservation.
All accounting is conservative logical capacity, not measured allocator or RSS.

Preparation checks complete target bytes against their immutable manifest and
authenticates the full retained 1 through H+1 proof stream from locally configured
genesis. Its exact derived identity must equal the captured actual parent. This
prevents a forged old proof prefix from becoming an unrecoverable advertised base.
It then separately starts from the captured actual ImportedState K and checks
K+1 through H+1, including the exact certified K+1 seed for nonzero K. Reopening
must independently repeat full genesis verification; checksum completion supplies
no trust, execution replay or fresh-head observation.

The candidate retains the complete target decoder charge and verifier session
charge after they are merged without releasing accounting. Old captured state
and content/proof metadata charges also remain alive. This conservative initial
implementation deliberately retains transient capacity with the generation to
avoid any gap in the lifetime of decoded maps, closing native data or policy.

Conditional activation must use the existing sole segmented worker and actual
checkpoint base acknowledgment. No H/cursor-zero state or second WAL is allowed.
The full runtime recovery, readiness, retention and B4 acceptance gates remain
required; source preparation alone is not complete B4 acceptance.
