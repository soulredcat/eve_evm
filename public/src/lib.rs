// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Public RPC, bounded local admission and explicit development orchestration.
//! Local durable execution is not authenticated validator finality.
//! Authenticated recovery application is an independent public-owned capability.
#![forbid(unsafe_code)]

pub mod development;
pub mod mempool;
pub mod persistence;
pub mod rpc;
pub mod runtime;
pub mod sync;
