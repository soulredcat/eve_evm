// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::DeltaServingError;
use eve_consensus_comet::wire::tendermint::abci::ResponseQuery;

pub(super) fn delta_serving_error_response(error: DeltaServingError) -> ResponseQuery {
    let (code, log) = match error {
        DeltaServingError::WrongNetwork => (1, "WRONG_NETWORK"),
        DeltaServingError::UnsupportedVersion => (2, "UNSUPPORTED_VERSION"),
        DeltaServingError::Gap => (3, "GAP"),
        DeltaServingError::ResourceLimit => (4, "RESOURCE_LIMIT"),
        DeltaServingError::NotReady => (5, "NOT_READY"),
        DeltaServingError::MalformedRequest => (6, "MALFORMED_REQUEST"),
    };
    ResponseQuery {
        code,
        codespace: "EVE_RECOVERY".into(),
        log: log.into(),
        ..Default::default()
    }
}
