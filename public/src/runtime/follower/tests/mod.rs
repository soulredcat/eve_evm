// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

#[cfg(target_os = "linux")]
mod checkpoint_directories;
mod checkpoint_guards;
mod checkpoint_profile;
mod guards;
mod profile;
