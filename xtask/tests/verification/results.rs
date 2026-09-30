use xtask::verification::discovery::parse_test_results::parse_test_results;

#[test]
fn results_require_exact_nonzero_execution_and_zero_skipped_or_failed_cases() {
    let valid = "test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0s\ntest result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0s\n";
    assert_eq!(parse_test_results(valid, 2).unwrap(), 2);
    assert!(parse_test_results(valid, 3).is_err());
    assert!(parse_test_results("", 0).is_err());
    for replacement in ["1 failed", "1 ignored", "1 measured", "1 filtered out"] {
        let failed = valid.replace(&replacement.replacen('1', "0", 1), replacement);
        assert!(parse_test_results(&failed, 2).is_err(), "{replacement}");
    }
    assert!(parse_test_results(&valid.replace("ok.", "FAILED."), 2).is_err());
    assert!(parse_test_results("test result: ok. no counts\n", 2).is_err());
}
