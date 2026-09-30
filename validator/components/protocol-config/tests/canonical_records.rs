#[path = "support/mod.rs"]
mod support;

use alloy_primitives::{B256, Bytes};
use eve_protocol_config::records::{
    DeltaOperation, RecordError, SystemNamespace, SystemRecord, SystemValue,
    encode_application_commitment, encode_delta_operation, encode_system_record,
    hash_application_commitment, hash_system_key, validate_canonical_record_bytes,
};

#[test]
fn freeze_minimal_integer_empty_data_deletion_and_presence_tags() {
    let record = support::fee_record();
    let encoded = encode_system_record(&record).unwrap();
    assert_eq!(hex::encode(&encoded), "ce018366656584706f6f6cc3800304");
    assert!(validate_canonical_record_bytes(&record, &encoded).is_ok());
    assert_eq!(
        validate_canonical_record_bytes(&record, b"noncanonical"),
        Err(RecordError::NonCanonical)
    );
    let delete = DeltaOperation {
        namespace: record.namespace,
        logical_key: record.logical_key.clone(),
        value: None,
    };
    assert_eq!(
        hex::encode(encode_delta_operation(&delete).unwrap()),
        "cb018366656584706f6f6c80"
    );
    let parameter = SystemRecord {
        schema_version: 1,
        namespace: SystemNamespace::Parameter,
        logical_key: Bytes::from_static(b"x"),
        value: SystemValue::Parameter {
            name: Bytes::from_static(b"p"),
            value: Bytes::new(),
        },
    };
    assert_eq!(
        hex::encode(encode_system_record(&parameter).unwrap()),
        "cf0189706172616d6574657278c27080"
    );
    let mut validator = SystemRecord {
        schema_version: 1,
        namespace: SystemNamespace::Validator,
        logical_key: Bytes::from_static(b"v"),
        value: SystemValue::Validator {
            owner: alloy_primitives::Address::repeat_byte(1),
            key: [2; 32],
            power: 1,
            activation: 1,
            removal: None,
            key_epoch: 0,
        },
    };
    assert!(hex::encode(encode_system_record(&validator).unwrap()).ends_with("0101c18080"));
    if let SystemValue::Validator { removal, .. } = &mut validator.value {
        *removal = Some(2);
    }
    assert!(hex::encode(encode_system_record(&validator).unwrap()).ends_with("0101c2010280"));
    let mut evidence = SystemRecord {
        schema_version: 1,
        namespace: SystemNamespace::Evidence,
        logical_key: Bytes::from_static(b"e"),
        value: SystemValue::Evidence {
            evidence_id: B256::repeat_byte(3),
            offense_height: 1,
            applied: false,
        },
    };
    assert!(
        encode_system_record(&evidence)
            .unwrap()
            .ends_with(&[1, 0x80])
    );
    if let SystemValue::Evidence { applied, .. } = &mut evidence.value {
        *applied = true;
    }
    assert!(encode_system_record(&evidence).unwrap().ends_with(&[1, 1]));
}

#[test]
fn application_commitment_has_exact_spec_field_order_and_domain() {
    let input = support::application();
    let expected = format!(
        "f8918a4556455f4150505f5631a0{}0107a0{}a0{}a0{}",
        "11".repeat(32),
        "22".repeat(32),
        "33".repeat(32),
        "44".repeat(32)
    );
    assert_eq!(
        hex::encode(encode_application_commitment(input).unwrap()),
        expected
    );
    let original = hash_application_commitment(input).unwrap();
    let mut changed = input;
    changed.system_root.0 = B256::repeat_byte(4);
    assert_ne!(hash_application_commitment(changed).unwrap(), original);
    changed = input;
    changed.execution_height = 0;
    assert_eq!(
        encode_application_commitment(changed),
        Err(RecordError::InvalidVersion)
    );
}

#[test]
fn rejects_wrong_schema_namespace_keys_and_delta_record_binding() {
    let mut record = support::fee_record();
    record.schema_version = 2;
    assert_eq!(
        encode_system_record(&record),
        Err(RecordError::InvalidVersion)
    );
    record.schema_version = 1;
    record.namespace = SystemNamespace::Reward;
    assert_eq!(
        encode_system_record(&record),
        Err(RecordError::InvalidRecord)
    );
    assert_eq!(
        hash_system_key(SystemNamespace::Fee, &[]),
        Err(RecordError::InvalidKey)
    );
    assert_eq!(
        hash_system_key(SystemNamespace::Fee, &[1; 129]),
        Err(RecordError::InvalidKey)
    );
    record = support::fee_record();
    let operation = DeltaOperation {
        namespace: SystemNamespace::Fee,
        logical_key: Bytes::from_static(b"other"),
        value: Some(record),
    };
    assert_eq!(
        encode_delta_operation(&operation),
        Err(RecordError::InvalidRecord)
    );
}
