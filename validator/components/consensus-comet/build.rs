// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

#[path = "src/generation/generate_upstream_bindings.rs"]
mod generate_upstream_bindings;

fn main() {
    generate_upstream_bindings::generate_upstream_bindings()
        .expect("pinned CometBFT protobuf generation failed");
}
