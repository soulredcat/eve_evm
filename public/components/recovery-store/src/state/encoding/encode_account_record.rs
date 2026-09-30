use eve_state::StateAccount;

/// Fixed-width local metadata; storage slots are separate and canonical roots stay upstream.
pub(crate) fn encode_account_record(account: &StateAccount) -> Vec<u8> {
    [
        account.nonce.to_be_bytes().as_slice(),
        account.balance.to_be_bytes::<32>().as_slice(),
        account.code_hash.as_slice(),
    ]
    .concat()
}
