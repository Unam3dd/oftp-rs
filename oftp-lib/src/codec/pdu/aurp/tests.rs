use super::*;

#[test]
fn encode_decode_roundtrip() {
    let original = Aurp {
        response: [0x42u8; AURP_RESPONSE_LEN],
    };
    let wire = original.encode().unwrap();
    assert_eq!(wire.len(), AURP_LEN);

    let mut decoded = Aurp::default();
    decoded.decode(&wire).unwrap();
    assert_eq!(decoded, original);
}

#[test]
fn decode_bad_len() {
    let mut aurp = Aurp::default();
    let err = aurp.decode(&[AURPCMD]).unwrap_err();
    assert_eq!(err, AurpError::InvalidSizeError);
}
