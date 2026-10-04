// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

pub(super) fn read_process_rss() -> (u64, u64) {
    let status = std::fs::read_to_string("/proc/self/status").unwrap();
    let mut rss = None;
    let mut hwm = None;
    for line in status.lines() {
        let target = if line.starts_with("VmRSS:") {
            &mut rss
        } else if line.starts_with("VmHWM:") {
            &mut hwm
        } else {
            continue;
        };
        let kib = line
            .split_ascii_whitespace()
            .nth(1)
            .unwrap()
            .parse::<u64>()
            .unwrap();
        *target = Some(kib.checked_mul(1_024).unwrap());
    }
    (
        rss.expect("own-process VmRSS"),
        hwm.expect("own-process VmHWM"),
    )
}
