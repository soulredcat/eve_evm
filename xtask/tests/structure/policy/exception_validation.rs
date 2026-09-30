use crate::support::{Fixture, source_with_lines};

#[test]
fn t_l04_valid_exception_is_exact_bounded_and_expires_by_the_next_bulk() {
    let fixture = Fixture::new();
    fixture.write(
        "validator/src/encode_height.rs",
        &source_with_lines(401, true),
    );
    fixture.policy(&exception("validator/src/encode_height.rs", 401, 1));
    fixture.assert_pass();
    fixture.write(
        "validator/src/encode_height.rs",
        &source_with_lines(400, true),
    );
    fixture.assert_rejected();
}

#[test]
fn t_l04_missing_expired_unbounded_and_wildcard_exceptions_fail() {
    let fixture = Fixture::new();
    fixture.write(
        "validator/src/encode_height.rs",
        &source_with_lines(401, true),
    );
    fixture.assert_rejected();
    fixture.policy(&exception("validator/src/encode_height.rs", 401, 2));
    fixture.assert_rejected();
    fixture.policy(&exception("validator/src/*.rs", 401, 1));
    fixture.assert_rejected();
    fixture.write(
        "config/structure-policy.toml",
        &format!(
            "version = 1\ncurrent_bulk = 2\n{}",
            exception("validator/src/encode_height.rs", 401, 1)
        ),
    );
    fixture.assert_rejected();
}

#[test]
fn t_l04_incomplete_exception_fields_are_rejected() {
    for (field, value) in [
        ("reason", "Bounded fixture"),
        ("reviewer", "test-reviewer"),
        ("split_task", "Split encoding fixture in B1"),
    ] {
        let fixture = Fixture::new();
        fixture.write(
            "validator/src/encode_height.rs",
            &source_with_lines(401, true),
        );
        let policy = exception("validator/src/encode_height.rs", 401, 1).replace(
            &format!("{field} = \"{value}\""),
            &format!("{field} = \"\""),
        );
        fixture.policy(&policy);
        fixture.assert_rejected();
    }
}

#[test]
fn stale_size_reviews_and_exceptions_do_not_survive_a_smaller_file() {
    for registration in [
        exception("validator/src/encode_height.rs", 401, 1),
        "[[size_reviews]]\npath = \"validator/src/encode_height.rs\"\nlines = 201\nreason = \"Reviewed earlier size.\"\nreviewer = \"test-reviewer\"\n".into(),
    ] {
        let fixture = Fixture::new();
        fixture.write("validator/src/encode_height.rs", "fn encode_height() {}\n");
        fixture.policy(&registration);
        fixture.assert_rejected();
    }
}

#[test]
fn size_review_cannot_extend_the_adapter_ceiling() {
    let fixture = Fixture::new();
    fixture.write(
        "validator/src/codec_adapter.rs",
        &source_with_lines(201, true),
    );
    fixture.policy("[[adapters]]\npath = \"validator/src/codec_adapter.rs\"\nexternal_trait = \"upstream::Codec\"\nreason = \"Delegation boundary.\"\nreviewer = \"test-reviewer\"\n[[size_reviews]]\npath = \"validator/src/codec_adapter.rs\"\nlines = 201\nreason = \"Attempted oversized adapter.\"\nreviewer = \"test-reviewer\"\n");
    fixture.assert_rejected();
}

#[test]
fn exception_requires_a_nonempty_related_test_reference() {
    for replacement in ["related_tests = []", "related_tests = [\"\"]"] {
        let fixture = Fixture::new();
        fixture.write(
            "validator/src/encode_height.rs",
            &source_with_lines(401, true),
        );
        let policy = exception("validator/src/encode_height.rs", 401, 1)
            .replace("related_tests = [\"T-L04\"]", replacement);
        fixture.policy(&policy);
        fixture.assert_rejected();
    }
}

fn exception(path: &str, lines: usize, expires_bulk: u8) -> String {
    format!(
        "[[exceptions]]\npath = \"{path}\"\nlines = {lines}\nreason = \"Bounded fixture\"\nreviewer = \"test-reviewer\"\nsplit_task = \"Split encoding fixture in B1\"\nexpires_bulk = {expires_bulk}\nrelated_tests = [\"T-L04\"]\n"
    )
}
