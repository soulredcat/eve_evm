// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::decode_committed_submission::decode_committed_submission;
use serde_json::{Value, json};

#[cfg(test)]
fn committed_result() -> Value {
    json!({
        "check_tx": {"code": 0},
        "tx_result": {"code": 0},
        "hash": hex::encode_upper([0x5a; 32]),
        "height": "7"
    })
}

#[test]
fn committed_submission_requires_numeric_codes_expected_hash_and_positive_height() {
    let mut result = committed_result();
    assert_eq!(
        decode_committed_submission(&result, &[0x5a; 32]).unwrap(),
        7
    );
    result["hash"] = json!(hex::encode([0x5a; 32]));
    result["height"] = json!(i64::MAX.to_string());
    assert_eq!(
        decode_committed_submission(&result, &[0x5a; 32]).unwrap(),
        i64::MAX
    );
}

#[test]
fn committed_submission_rejects_missing_malformed_and_nonzero_codes() {
    for (field, prefix) in [
        ("check_tx", "B3_SUBMIT_CHECK_TX"),
        ("tx_result", "B3_SUBMIT_TX_RESULT"),
    ] {
        for value in [None, Some(Value::Null), Some(json!(0)), Some(json!([]))] {
            let mut result = committed_result();
            if let Some(value) = value {
                result[field] = value;
            } else {
                result.as_object_mut().unwrap().remove(field);
            }
            assert_eq!(
                decode_committed_submission(&result, &[0x5a; 32])
                    .unwrap_err()
                    .to_string(),
                format!("{prefix}_MISSING")
            );
        }
        for code in [
            None,
            Some(Value::Null),
            Some(json!("0")),
            Some(json!(-1)),
            Some(json!(0.0)),
            Some(json!(true)),
            Some(json!(u64::from(u32::MAX) + 1)),
        ] {
            let mut result = committed_result();
            if let Some(code) = code {
                result[field]["code"] = code;
            } else {
                result[field].as_object_mut().unwrap().remove("code");
            }
            assert_eq!(
                decode_committed_submission(&result, &[0x5a; 32])
                    .unwrap_err()
                    .to_string(),
                format!("{prefix}_CODE")
            );
        }
        let mut result = committed_result();
        result[field]["code"] = json!(1);
        result[field]["log"] = json!("untrusted-server-detail");
        let error = decode_committed_submission(&result, &[0x5a; 32]).unwrap_err();
        assert_eq!(error.to_string(), format!("{prefix}_REJECTED"));
        assert!(!format!("{error:#}").contains("untrusted-server-detail"));
    }
}

#[test]
fn committed_submission_rejects_wrong_hash_and_nonpositive_or_noncanonical_height() {
    for (hash, expected) in [
        (Value::Null, "B3_SUBMIT_HASH_MISSING"),
        (json!(1), "B3_SUBMIT_HASH_MISSING"),
        (json!(""), "B3_SUBMIT_HASH_FORMAT"),
        (json!(hex::encode([0x5a; 31])), "B3_SUBMIT_HASH_FORMAT"),
        (json!("g".repeat(64)), "B3_SUBMIT_HASH_FORMAT"),
        (json!(hex::encode([0x5b; 32])), "B3_SUBMIT_HASH_MISMATCH"),
    ] {
        let mut result = committed_result();
        result["hash"] = hash;
        assert_eq!(
            decode_committed_submission(&result, &[0x5a; 32])
                .unwrap_err()
                .to_string(),
            expected
        );
    }
    for height in [
        "",
        "0",
        "-1",
        "+1",
        "01",
        "1 ",
        "1e1",
        "9223372036854775808",
    ] {
        let mut result = committed_result();
        result["height"] = json!(height);
        assert_eq!(
            decode_committed_submission(&result, &[0x5a; 32])
                .unwrap_err()
                .to_string(),
            "B3_SUBMIT_HEIGHT_INVALID"
        );
    }
    for height in [Value::Null, json!(1), json!({})] {
        let mut result = committed_result();
        result["height"] = height;
        assert_eq!(
            decode_committed_submission(&result, &[0x5a; 32])
                .unwrap_err()
                .to_string(),
            "B3_SUBMIT_HEIGHT_MISSING"
        );
    }
}
