use crate::structure::types::policy_types::StructurePolicy;

pub fn review_size(
    path: &str,
    lines: usize,
    policy: &StructurePolicy,
) -> (Vec<String>, Vec<String>) {
    let mut warnings = Vec::new();
    let mut violations = Vec::new();
    if policy.size_reviews.iter().any(|review| {
        review.path == path && (!(201..=400).contains(&lines) || review.lines != lines)
    }) {
        violations.push(format!("{path}: stale decomposition review"));
    }
    if policy.exceptions.iter().any(|entry| {
        entry.path == path
            && (!(401..=600).contains(&lines)
                || entry.lines != lines
                || entry.expires_bulk < policy.current_bulk
                || entry.expires_bulk > policy.current_bulk.saturating_add(1))
    }) {
        violations.push(format!("{path}: stale or expired size exception"));
    }
    match lines {
        0..=200 => {}
        201..=400 => {
            let review = policy
                .size_reviews
                .iter()
                .find(|review| review.path == path);
            if review.is_some_and(|review| {
                review.lines == lines
                    && !review.reason.trim().is_empty()
                    && !review.reviewer.trim().is_empty()
            }) {
                warnings.push(format!("Reviewed {lines}-line file: {path}"));
            } else {
                violations.push(format!(
                    "{path}: {lines} lines requires a decomposition review"
                ));
            }
        }
        401..=600 => {
            let exception = policy
                .exceptions
                .iter()
                .find(|exception| exception.path == path);
            if exception.is_some_and(|entry| {
                entry.lines == lines
                    && !entry.reason.trim().is_empty()
                    && !entry.reviewer.trim().is_empty()
                    && !entry.split_task.trim().is_empty()
                    && !entry.related_tests.is_empty()
                    && entry
                        .related_tests
                        .iter()
                        .all(|test| !test.trim().is_empty())
                    && entry.expires_bulk >= policy.current_bulk
                    && entry.expires_bulk <= policy.current_bulk.saturating_add(1)
            }) {
                warnings.push(format!("Temporary {lines}-line exception: {path}"));
            } else {
                violations.push(format!(
                    "{path}: missing, expired or unbounded {lines}-line exception"
                ));
            }
        }
        _ => violations.push(format!(
            "{path}: {lines} physical lines exceeds the hard 600-line limit"
        )),
    }
    (warnings, violations)
}
