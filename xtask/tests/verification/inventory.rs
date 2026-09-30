use xtask::verification::discovery::parse_test_inventory::parse_test_inventory;

#[test]
fn discovery_rejects_zero_missing_duplicate_and_unregistered_cases() {
    let required = vec!["positive_case".to_owned(), "negative_case".to_owned()];
    for output in [
        "",
        "0 tests, 0 benchmarks\n",
        "positive_case: test\n",
        "positive_case: test\npositive_case: test\nnegative_case: test\n",
        "positive_case: test\nnegative_case: test\nunregistered_case: test\n",
    ] {
        assert!(parse_test_inventory(output, &required).is_err(), "{output}");
    }
    let output =
        "Running tests\npositive_case: test\r\nnegative_case: test\r\n2 tests, 0 benchmarks\n";
    assert_eq!(parse_test_inventory(output, &required).unwrap().len(), 2);
    assert!(parse_test_inventory(output, &[]).is_err());
    assert!(
        parse_test_inventory(output, &["positive_case".into(), "positive_case".into()]).is_err()
    );
}
