// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::MeasurementServers;
pub(super) async fn stop_measurement_http(mut servers: MeasurementServers) {
    if let Some((http, ws)) = servers.handles.take() {
        let _ = http.stop();
        let _ = ws.stop();
        tokio::join!(http.stopped(), ws.stopped());
    }
}
