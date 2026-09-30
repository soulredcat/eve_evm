use eve_crypto::verify_mldsa65;
use serde::Deserialize;

#[derive(Deserialize)]
struct Fixture {
    algorithm: String,
    parameter_set: String,
    signature_interface: String,
    pre_hash: String,
    tests: Vec<Case>,
}

#[derive(Deserialize)]
struct Case {
    tc_id: u32,
    public_key: String,
    message: String,
    context: String,
    signature: String,
    expected_valid: bool,
    reason: String,
}

fn verify_official_case(tc_id: u32) {
    let fixture: Fixture = serde_json::from_str(include_str!(
        "fixtures/nist-acvp/ml-dsa65-sigver-external-pure.json"
    ))
    .expect("pinned NIST fixture schema");
    assert_eq!(fixture.algorithm, "ML-DSA");
    assert_eq!(fixture.parameter_set, "ML-DSA-65");
    assert_eq!(fixture.signature_interface, "external");
    assert_eq!(fixture.pre_hash, "pure");
    assert_eq!(
        fixture.tests.len(),
        15,
        "fixture must not silently lose cases"
    );
    let case = fixture
        .tests
        .iter()
        .find(|case| case.tc_id == tc_id)
        .expect("required case");
    let result = verify_mldsa65(
        &hex::decode(&case.public_key).unwrap(),
        &hex::decode(&case.message).unwrap(),
        &hex::decode(&case.context).unwrap(),
        &hex::decode(&case.signature).unwrap(),
    );
    assert_eq!(
        result.is_ok(),
        case.expected_valid,
        "NIST tcId {tc_id}: {}",
        case.reason
    );
}

macro_rules! official_case {
    ($name:ident, $case:literal) => {
        #[test]
        fn $name() {
            verify_official_case($case);
        }
    };
}

official_case!(tp01_nist_mldsa65_tc31, 31);
official_case!(tp01_nist_mldsa65_tc32, 32);
official_case!(tp01_nist_mldsa65_tc33, 33);
official_case!(tp01_nist_mldsa65_tc34, 34);
official_case!(tp01_nist_mldsa65_tc35, 35);
official_case!(tp01_nist_mldsa65_tc36, 36);
official_case!(tp01_nist_mldsa65_tc37, 37);
official_case!(tp01_nist_mldsa65_tc38, 38);
official_case!(tp01_nist_mldsa65_tc39, 39);
official_case!(tp01_nist_mldsa65_tc40, 40);
official_case!(tp01_nist_mldsa65_tc41, 41);
official_case!(tp01_nist_mldsa65_tc42, 42);
official_case!(tp01_nist_mldsa65_tc43, 43);
official_case!(tp01_nist_mldsa65_tc44, 44);
official_case!(tp01_nist_mldsa65_tc45, 45);
