// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Real loopback transport of canonical untrusted development message fixtures.
mod checkpoint_download {
    mod deadlines;
    mod fixtures;
    mod foreign_response;
    mod leases;
    mod native_fixture;
    mod outer_deadlines;
    mod resources;
    mod responses;
    mod server;
    mod witnesses;
}
